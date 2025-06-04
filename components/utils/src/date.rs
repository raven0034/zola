use libs::chrono::{DateTime, Local};
use libs::chrono_english::{parse_date_string, Dialect};
use libs::chrono_tz::Tz;
use libs::time::format_description::well_known::Rfc3339;
use libs::time::OffsetDateTime;

pub fn parse_human_date(date_str: &str, base: Option<DateTime<Tz>>) -> Option<DateTime<Tz>> {
    let base_date = if let Some(b) = base { b } else { Local::now().with_timezone(&Tz::Europe__London) };
    parse_date_string(date_str, base_date, Dialect::Uk).ok()
}

pub fn chrono_to_time_date(datetime: DateTime<Tz>) -> OffsetDateTime {
    OffsetDateTime::parse(&datetime.to_rfc3339(), &Rfc3339)
        .expect("`chrono` and `time` should parse RFC3339 equivalently")
}
