use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use reqwest::Url;
use serde::Deserialize;

use std::env;

use crate::{error::YahooError, Interval};

const BASE_URL: &'static str = "https://query1.finance.yahoo.com/v8/finance/chart/";

/// Helper function to build up the main query URL
fn build_query(symbol: &str) -> Result<Url, YahooError> {
    let base = env::var("TEST_URL").unwrap_or(BASE_URL.to_string());

    let parse = match Url::parse(&base) {
        Ok(x) => x,
        Err(y) => {
            return Err(YahooError::InternalURL {
                url: base,
                source: y,
            })
        }
    };

    return match parse.join(symbol) {
        Ok(x) => Ok(x),
        Err(y) => Err(YahooError::InternalURL {
            url: base,
            source: y,
        }),
    };
}

ez_serde!(Meta {
   symbol: String,

   #[serde(with = "ts_seconds")]
   first_trade_date: DateTime<Utc>,

   #[serde(rename = "regularMarketPrice")]
   current_price: f32,

   #[serde(rename = "chartPreviousClose")]
   previous_close: f32
});

ez_serde!(OHLCV {
   #[serde(rename = "open", default)]
   opens: Vec<Option<f64>>,

   #[serde(rename = "high", default)]
   highs: Vec<Option<f64>>,

   #[serde(rename = "low", default)]
   lows: Vec<Option<f64>>,

   #[serde(rename = "close", default)]
   closes: Vec<Option<f64>>,

   #[serde(rename = "volume", default)]
   volumes: Vec<Option<u64>>
});

ez_serde!(Indicators { #[serde(rename = "quote", default)] quotes: Vec<OHLCV> });

ez_serde!(Data {
   meta: Meta,

   #[serde(rename = "timestamp", default)]
   timestamps: Vec<i64>,

   indicators: Indicators
});

ez_serde!(Error {
    code: String,
    description: String
});
ez_serde!(Chart { result: Option<Vec<Data>>, error: Option<Error> });
ez_serde!(ChartResponse { chart: Chart });

async fn load(url: &Url) -> Result<Data, YahooError> {
    // make the call - we do not really expect this to fail.
    // ie - we won't 404 if the symbol doesn't exist
    let response = match reqwest::get(url.clone()).await {
        Ok(x) => match x.status().is_success() {
            true => x,
            false => {
                return Err(YahooError::CallFailed {
                    url: url.clone().to_string(),
                    status: x.status().as_u16(),
                })
            }
        },
        Err(y) => return Err(YahooError::RequestFailed { source: y }),
    };

    let data = match response.text().await {
        Ok(x) => x,
        Err(y) => {
            return Err(YahooError::UnexpectedErrorRead {
                url: url.to_string(),
                source: y,
            })
        }
    };

    let chart = match serde_json::from_str::<ChartResponse>(&data) {
        Ok(x) => x.chart,
        Err(y) => return Err(YahooError::BadData { source: y }),
    };

    if chart.result.is_none() {
        return Err(YahooError::ChartFailed {
            description: "error block exists without values".to_string(),
        });
    }

    // we have a result to process

    let result = match chart.result {
        Some(x) => x,
        None => return Err(YahooError::UnexpectedErrorYahoo),
    };

    return if result.len() > 0 {
        Ok(result[0].clone())
    } else {
        Err(YahooError::UnexpectedErrorYahoo)
    };
}

pub async fn load_daily(symbol: &str, period: Interval) -> Result<Data, YahooError> {
    let mut lookup = build_query(symbol)?;
    lookup
        .query_pairs_mut()
        .append_pair("range", &period.to_string())
        .append_pair("interval", "1d");

    load(&lookup).await
}

pub async fn load_daily_range(symbol: &str, start: i64, end: i64) -> Result<Data, YahooError> {
    let mut lookup = build_query(symbol)?;
    lookup
        .query_pairs_mut()
        .append_pair("period1", &start.to_string())
        .append_pair("period2", &end.to_string())
        .append_pair("interval", "1d");

    load(&lookup).await
}
