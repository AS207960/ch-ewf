#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_derive_enum;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate log;

use std::str::FromStr;

mod proto;
mod gov_talk;
mod schema;
mod models;
mod psc;
mod base_types;
mod members;
mod officers;
mod grpc;

pub mod ch_ewf_grpc {
    #![allow(unknown_lints, clippy::all)]

    tonic::include_proto!("ch_ewf");

    pub mod base_types {
        tonic::include_proto!("ch_ewf.base_types");
    }

    pub mod psc {
        tonic::include_proto!("ch_ewf.psc");
    }

    pub mod company_data {
        tonic::include_proto!("ch_ewf.company_data");
    }

    pub mod e_reminders {
        tonic::include_proto!("ch_ewf.e_reminders");
    }

    pub mod payment_periods {
        tonic::include_proto!("ch_ewf.payment_periods");
    }

    pub mod members_data {
        tonic::include_proto!("ch_ewf.members_data");
    }

    pub mod form_submission {
        tonic::include_proto!("ch_ewf.form_submission");
    }

    pub mod accounting_reference_date {
        tonic::include_proto!("ch_ewf.accounting_reference_date");
    }

    pub mod change_of_location {
        tonic::include_proto!("ch_ewf.change_of_location");
    }

    pub mod change_of_name {
        tonic::include_proto!("ch_ewf.change_of_name");
    }

    pub mod change_registered_office {
        tonic::include_proto!("ch_ewf.change_registered_office");
    }

    pub mod confirmation_statement {
        tonic::include_proto!("ch_ewf.confirmation_statement");
    }

    pub mod officer_appointment {
        tonic::include_proto!("ch_ewf.officer_appointment");
    }

    pub mod officer_change {
        tonic::include_proto!("ch_ewf.officer_change");
    }

    pub mod officer_resignation {
        tonic::include_proto!("ch_ewf.officer_resignation");
    }

    pub mod psc_cessation {
        tonic::include_proto!("ch_ewf.psc_cessation");
    }

    pub mod psc_change_details {
        tonic::include_proto!("ch_ewf.psc_change_details");
    }

    pub mod psc_notification {
        tonic::include_proto!("ch_ewf.psc_notification");
    }

    pub mod psc_statement_notification {
        tonic::include_proto!("ch_ewf.psc_statement_notification");
    }

    pub mod psc_statement_withdrawal {
        tonic::include_proto!("ch_ewf.psc_statement_withdrawal");
    }

    pub mod register_elect_or_withdraw {
        tonic::include_proto!("ch_ewf.register_elect_or_withdraw");
    }

    pub mod sail_address {
        tonic::include_proto!("ch_ewf.sail_address");
    }

    pub mod members_register {
        tonic::include_proto!("ch_ewf.members_register");
    }

    pub mod members_register_update {
        tonic::include_proto!("ch_ewf.members_register_update");
    }

    pub mod return_allotment_shares {
        tonic::include_proto!("ch_ewf.return_allotment_shares");
    }

    pub mod company_incorporation {
        tonic::include_proto!("ch_ewf.company_incorporation");
    }

    pub mod charge_registration {
        tonic::include_proto!("ch_ewf.charge_registration");
    }

    pub mod charge_update {
        tonic::include_proto!("ch_ewf.charge_update");
    }

    pub mod charge_search {
        tonic::include_proto!("ch_ewf.charge_search");
    }
}

pub fn establish_connection(database_url: String) -> r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::pg::PgConnection>> {
    let conn = diesel::r2d2::ConnectionManager::<diesel::pg::PgConnection>::new(&database_url);
    r2d2::Pool::new(conn).unwrap()
}

embed_migrations!("migrations");

#[derive(Debug, Deserialize)]
struct Config {
    presenter_id: String,
    presenter_code: String,
    presenter_email: String,
    package_reference: String,
    database_url: String,
    documents_path: std::path::PathBuf,
    #[serde(default = "default_listen_url")]
    listen_socket: std::net::SocketAddr,
    #[serde(default)]
    test_mode: bool
}

fn default_listen_url() -> std::net::SocketAddr {
    std::net::SocketAddr::new(
        std::net::Ipv6Addr::from_str("::1").unwrap().into(), 50051
    )
}

#[tokio::main]
async fn main() {
    let args = clap::Command::new("AS207960 Companies House eFiller")
        .author("Q Misell <q@as207960.net>")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(clap::Arg::new("settings")
            .short('s')
            .long("settings")
            .takes_value(true)
            .required(true)
            .help("Location of the settings file"))
        .get_matches();

    if systemd_journal_logger::connected_to_journal() {
        systemd_journal_logger::init().unwrap();
        log::set_max_level(log::LevelFilter::Debug);
    } else {
        pretty_env_logger::init();
    }

    let settings: Config = config::Config::builder()
        .add_source(config::File::with_name(args.value_of("settings").unwrap()))
        .add_source(config::Environment::with_prefix("CH_EWF"))
        .build()
        .unwrap()
        .try_deserialize()
        .unwrap();

    let connection = establish_connection(settings.database_url);

    info!("Migrating database...");
    embedded_migrations::run(&connection.get().expect("Unable to get DB connection"))
        .expect("Unable to apply migrations");

    let sender = gov_talk::GovTalkSender::new(
        &settings.presenter_email, &settings.presenter_id, &settings.presenter_code,
        settings.test_mode
    );
    let service = grpc::CHFillingService {
        sender,
        connection,
        documents_path: settings.documents_path,
        presenter_id: settings.presenter_id,
        package_reference: settings.package_reference,
    };

    info!("Starting submission watcher...");
    let w_service = service.clone();
    tokio::task::spawn(async move {
        w_service.watcher().await
    });

    info!("Starting server...");
    tonic::transport::Server::builder()
        .add_service(ch_ewf_grpc::ch_filling_server::ChFillingServer::new(service))
        .serve(settings.listen_socket)
        .await
        .expect("Unable to start listener");
}
