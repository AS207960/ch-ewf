pub mod govtalk;
pub mod base_types;
pub mod company_data;
pub mod members_data;
pub mod payment_periods;
pub mod e_reminders;
pub mod psc;
pub mod register;
pub mod form_submission;
pub mod submission_status;
pub mod document;
pub mod confirmation_statement;
pub mod accounting_reference_date;
pub mod change_of_name;
pub mod change_registered_office;
pub mod increase_nominal_capital;
pub mod members_register;
pub mod members_register_update;
pub mod officer_appointment;
pub mod officer_change;
pub mod officer_resignation;
pub mod psc_cessation;
pub mod psc_change_details;
pub mod psc_notification;
pub mod psc_statement_notification;
pub mod psc_statement_withdrawal;
pub mod change_of_location;
pub mod register_elect_or_withdraw;
pub mod return_allotment_shares;
pub mod sail_address;
pub mod company_incorporation;
pub mod corporation_tax_information;
pub mod charge_search;
pub mod charge_registration;
pub mod charge_update;

use chrono::prelude::*;

fn is_false(b: &bool) -> bool {
    !(*b)
}

struct DateTimeVisitor;

impl<'de> serde::de::Visitor<'de> for DateTimeVisitor {
    type Value = DateTime<Utc>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a formatted date and time string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match value.parse::<DateTime<Utc>>() {
            Ok(v) => Ok(v.with_timezone(&Utc)),
            Err(_) => match Utc.datetime_from_str(value, "%FT%T%.f") {
                Ok(t) => Ok(t),
                Err(_) => Utc.datetime_from_str(value, "%FT%T").map_err(E::custom),
            },
        }
    }
}

struct DateVisitor;

impl<'de> serde::de::Visitor<'de> for DateVisitor {
    type Value = Date<Utc>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a formatted date string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match NaiveDate::parse_from_str(value, "%Y-%m-%d") {
            Ok(v) => match chrono_tz::Europe::London.from_local_date(&v) {
                chrono::offset::LocalResult::None => Err(E::custom("invalid date")),
                chrono::offset::LocalResult::Ambiguous(_, _) => Err(E::custom("invalid date")),
                chrono::offset::LocalResult::Single(d) => Ok(d.with_timezone(&Utc)),
            },
            Err(e) => Err(E::custom(e)),
        }
    }
}

struct OptDateTimeVisitor;

impl<'de> serde::de::Visitor<'de> for OptDateTimeVisitor {
    type Value = Option<DateTime<Utc>>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a formatted date and time string")
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(None)
    }

    fn visit_some<D>(self, d: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        d.deserialize_str(DateTimeVisitor).map(Some)
    }
}

struct OptDateVisitor;

impl<'de> serde::de::Visitor<'de> for OptDateVisitor {
    type Value = Option<Date<Utc>>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a formatted date string")
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(None)
    }

    fn visit_some<D>(self, d: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        d.deserialize_str(DateVisitor).map(Some)
    }
}

fn deserialize_datetime_opt<'de, D>(d: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let date = d.deserialize_option(OptDateTimeVisitor)?;
    Ok(match date {
        Some(d) => {
            if d == Utc.ymd(1, 1, 1).and_hms(0, 0, 0) {
                None
            } else {
                Some(d)
            }
        }
        None => None,
    })
}

fn deserialize_date<'de, D>(d: D) -> Result<Date<Utc>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    d.deserialize_str(DateVisitor)
}

fn deserialize_date_opt<'de, D>(d: D) -> Result<Option<Date<Utc>>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let date = d.deserialize_option(OptDateVisitor)?;
    Ok(match date {
        Some(d) => {
            if d == Utc.ymd(1, 1, 1) {
                None
            } else {
                Some(d)
            }
        }
        None => None,
    })
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn serialize_date<S>(d: &Date<Utc>, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::ser::Serializer,
{
    s.serialize_str(&d.format("%Y-%m-%d").to_string())
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn serialize_date_opt<S>(d: &Option<Date<Utc>>, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::ser::Serializer,
{
    match d {
        Some(d) => s.serialize_str(&d.format("%Y-%m-%d").to_string()),
        None => s.serialize_none(),
    }
}
