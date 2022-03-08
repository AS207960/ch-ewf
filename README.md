# Companies House eFilling Client

A client for the Companies House XML filling API, exposing the forms as gRPC services.

## How to use

Pass the pass to the settings file using the command line option `-s`. 
The file can be JSON, TOML, or YAML. A PostgreSQL (or compatible) database is required.

```toml
presenter_id = "<your presenter ID>"
presenter_code = "<your presenter authentication code>"
presenter_email = "<last resort contact email>"
package_reference = "<assigned by companies house>"
database_url = "postgres://user:pass@host/db_name"
documents_path = "<path to store response PDFs>"
```

### Presenter account 

You will need to apply for a presenter account from Companies House, either with a credit account
to file fee bearing forms, or without if you don't require that. The application forms are
[available here](https://www.gov.uk/government/publications/apply-for-a-companies-house-online-filing-presenter-account).

### Package reference

You will need to email [xml@companieshouse.gov.uk](mailto:xml@companieshouse.gov.uk) to request a test
package reference and complete the required testing to recieve a live package reference.
We're unable to openly share our own live package reference.

### gRPC spec

The gRPC protobuf files are available in [`proto/`](tree/root/proto/).

### Building

```shell
cargo build --release
```

## Contributing

Contributions are very welcome and appreciated. We'd like to know if you use this
(purely for bragging rights)!

## Feature requests

Feature requests can be made by submitting a GitHub issue.

## Wanting to use this commercially?

The source code is provided fully free of charge and with no strings attached.
You're welcome to use it however you want!

If however you'd like to discuss a support contract or need help using this project
drop us an email at [hello@glauca.digital](mailto:hello@glauca.digital).