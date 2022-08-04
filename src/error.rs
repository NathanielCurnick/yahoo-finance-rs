use reqwest;

use crate::market_utils::interval::Interval;

/// All possible errors that can occur when using yahoo finance
#[derive(Debug)]
pub enum YahooError {
    BadData {
        source: serde_json::Error,
    },

    CallFailed {
        url: String,
        status: u16,
    },

    ChartFailed {
        description: String,
    },

    InternalLogic {
        reason: String,
    },

    InternalURL {
        url: String,
        source: url::ParseError,
    },

    InvalidStartDate,

    MissingData {
        reason: String,
    },

    NoIntraday {
        interval: Interval,
    },

    RequestFailed {
        source: reqwest::Error,
    },

    UnexectedFailure {
        url: String,
        code: u16,
    },

    UnexpectedErrorRead {
        url: String,
        source: reqwest::Error,
    },

    UnexpectedErrorYahoo,

    Unknown,

    UnsupportedSecurity {
        kind: String,
    },
}
