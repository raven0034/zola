use libs::chrono::{DateTime, Local};
use libs::chrono_english::{parse_date_string, Dialect};
use libs::chrono_tz::Tz;
use libs::time::format_description::well_known::Rfc3339;
use libs::time::OffsetDateTime;
use errors::{anyhow, Context, Result};

pub fn parse_human_date(date_str: &str, base: Option<DateTime<Tz>>) -> Result<DateTime<Tz>> {
    let base_date = if let Some(b) = base { b } else { Local::now().with_timezone(&Tz::Europe__London) };
    parse_date_string(date_str, base_date, Dialect::Uk)
        .with_context(|| anyhow!("Unable to parse datetime: {}", date_str))
}

pub fn chrono_to_time_date(datetime: DateTime<Tz>) -> OffsetDateTime {
    OffsetDateTime::parse(&datetime.to_rfc3339(), &Rfc3339)
        .expect("`chrono` and `time` should parse RFC3339 equivalently")
}
