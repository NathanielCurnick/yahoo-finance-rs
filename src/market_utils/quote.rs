/// The trading session where a quote has occurred
#[derive(Clone, Copy, Debug)]
pub enum TradingSession {
    /// The period of trading before the regular market session
    PreMarket,

    /// The period of trading during the regular market session
    Regular,

    /// The period of trading immediately after the regular market session
    AfterHours,

    /// Undefined right now - for future use
    Other,
}

/// A symbol's quote at a period in time
#[derive(Clone, Debug)]
pub struct Quote {
    /// The symbol for the quote
    pub symbol: String,

    /// The timestamp of the quote in millisecond accuracy
    pub timestamp: i64,

    /// The trading session of the quote - pre market / regular hours / after hours
    pub session: TradingSession,

    /// The price of the quote
    pub price: f64,

    /// The volume (daily or transactional) of the symbol
    pub volume: u64,
}
impl Timestamped for Quote {
    /// Gets the timestamp in millisecond accuracy
    fn timestamp_millis(&self) -> i64 {
        self.timestamp
    }
}

#[cfg(test)]
mod tests {
    use super::{Quote, Timestamped, TradingSession};
    #[cfg(feature = "formatting")]
    use chrono::{TimeZone, Utc};

    #[cfg(feature = "formatting")]
    #[test]
    fn verify_datetime() {
        let dt = Utc.ymd(1970, 1, 1).and_hms_milli(0, 0, 3, 123);
        let quote = Quote {
            symbol: "none".to_string(),
            timestamp: dt.timestamp_millis(),
            session: TradingSession::Other,
            price: 0.1,
            volume: 0,
        };
        assert_eq!(dt, quote.datetime())
    }

    #[test]
    fn verify_timestamp_millis() {
        let quote = Quote {
            symbol: "none".to_string(),
            timestamp: 3002,
            session: TradingSession::Other,
            price: 0.1,
            volume: 0,
        };
        assert_eq!(3002, quote.timestamp_millis())
    }

    #[test]
    fn verify_timestamp_seconds() {
        let quote = Quote {
            symbol: "none".to_string(),
            timestamp: 3002,
            session: TradingSession::Other,
            price: 0.1,
            volume: 0,
        };
        assert_eq!(3, quote.timestamp_seconds())
    }
}

#[cfg(feature = "formatting")]
use chrono::{DateTime, NaiveDateTime, Utc};

///  A collection of methods for a structure that has a single timestamp
pub trait Timestamped {
    #[cfg(feature = "formatting")]
    /// Converts the timestamp into a chrono DateTime structure for easier date / time formatting
    fn datetime(&self) -> DateTime<Utc> {
        let millis = (self.timestamp_millis() % 1_000) * 1_000_000;
        DateTime::<Utc>::from_utc(
            NaiveDateTime::from_timestamp(self.timestamp_seconds() as i64, millis as u32),
            Utc,
        )
    }

    /// Gets the timestamp as the number milliseconds that have elapsed since the EPOCH
    fn timestamp_millis(&self) -> i64;

    /// Gets the timestamp as the number of seconds that have elapsed since the EPOCH
    fn timestamp_seconds(&self) -> i64 {
        self.timestamp_millis() / 1_000
    }
}
