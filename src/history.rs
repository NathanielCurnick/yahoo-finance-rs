use chrono::{DateTime, Utc};

use crate::{error::YahooError, yahoo, Bar, Interval};

fn aggregate_bars(data: yahoo::Data) -> Result<Vec<Bar>, YahooError> {
    let mut result = Vec::new();

    let timestamps = &data.timestamps;
    let quotes = &data.indicators.quotes;

    // if we have no timestamps & no quotes we'll assume there is no data
    if timestamps.is_empty() && quotes.is_empty() {
        return Ok(result);
    }

    // otherwise see if one is empty and reflects bad data from Yahoo!

    if timestamps.is_empty() {
        return Err(YahooError::MissingData {
            reason: "No timestamp for OHLCV data".to_string(),
        });
    };

    if quotes.is_empty() {
        return Err(YahooError::MissingData {
            reason: "No OHLCV data".to_string(),
        });
    };

    // make sure timestamps lines up with the OHLCV data
    let quote = &quotes[0];

    if timestamps.len() != quote.volumes.len() {
        return Err(YahooError::MissingData {
            reason: "Timestamps do not line up with OHLCV data".to_string(),
        });
    };
    if timestamps.len() != quote.opens.len() {
        return Err(YahooError::MissingData {
            reason: "Open values do not line up with timestamps".to_string(),
        });
    };
    if timestamps.len() != quote.highs.len() {
        return Err(YahooError::MissingData {
            reason: "High values do not line up with the timestamps".to_string(),
        });
    };
    if timestamps.len() != quote.lows.len() {
        return Err(YahooError::MissingData {
            reason: "Low values do not line up with timestamps".to_string(),
        });
    };
    if timestamps.len() != quote.closes.len() {
        return Err(YahooError::MissingData {
            reason: "Close values do not line up with the timestamps".to_string(),
        });
    };

    #[allow(clippy::needless_range_loop)]
    for i in 0..timestamps.len() {
        // skip days where we have incomplete data
        if quote.opens[i].is_none()
            || quote.highs[i].is_none()
            || quote.lows[i].is_none()
            || quote.closes[i].is_none()
        {
            continue;
        }

        result.push(Bar {
            timestamp: timestamps[i] * 1000,
            open: quote.opens[i].unwrap(),
            high: quote.highs[i].unwrap(),
            low: quote.lows[i].unwrap(),
            close: quote.closes[i].unwrap(),
            volume: quote.volumes[i],
        })
    }
    Ok(result)
}

/// Retrieves (at most) 6 months worth of OCLHV data for a symbol
/// ending on the last market close.
///
/// # Examples
///
/// Get 6 months worth of Apple data:
///
/// ``` no_run
/// use yahoo_finance::{ history, Timestamped };
///
/// #[tokio::main]
/// async fn main() {
///    match history::retrieve("AAPL").await {
///       Err(e) => println!("Failed to call Yahoo: {:?}", e),
///       Ok(data) =>
///          for bar in &data {
///             println!("On {} Apple closed at ${:.2}", bar.datetime().format("%b %e %Y"), bar.close)
///          }
///    }
/// }
/// ```
pub async fn retrieve(symbol: &str) -> Result<Vec<Bar>, YahooError> {
    aggregate_bars(yahoo::load_daily(symbol, Interval::_6mo).await?)
}

/// Retrieves a configurable amount of OCLHV data for a symbol
/// ending on the last market close.  The amount of data returned
/// might be less than the interval specified if the symbol
/// is new.
///
/// # Examples
///
/// Get 5 days worth of Apple data:
///
/// ``` no_run
/// use yahoo_finance::{ history, Interval, Timestamped };
///
/// #[tokio::main]
/// async fn main() {
///    match history::retrieve_interval("AAPL", Interval::_5d).await {
///       Err(e) => println!("Failed to call Yahoo: {:?}", e),
///       Ok(data) =>
///          for bar in &data {
///             println!("On {} Apple closed at ${:.2}", bar.datetime().format("%b %e %Y"), bar.close)
///          }
///    }
/// }
/// ```
pub async fn retrieve_interval(symbol: &str, interval: Interval) -> Result<Vec<Bar>, YahooError> {
    // pre-conditions

    if interval.is_intraday() {
        return Err(YahooError::NoIntraday { interval: interval });
    };

    aggregate_bars(yahoo::load_daily(symbol, interval).await?)
}

/// Retrieves OCLHV data for a symbol between a start and end date.
///
/// # Examples
///
/// Get 5 days worth of Apple data:
///
/// ``` no_run
/// use chrono::{Duration, TimeZone, Utc};
/// use yahoo_finance::{ history, Timestamped };
///
/// #[tokio::main]
/// async fn main() {
///    let now = Utc::now();
///    match history::retrieve_range("AAPL", now - Duration::days(30), Some(now - Duration::days(10))).await {
///       Err(e) => println!("Failed to call Yahoo {:?}", e),
///       Ok(data) =>
///          for bar in &data {
///             println!("On {} Apple closed at ${:.2}", bar.datetime().format("%b %e %Y"), bar.close)
///          }
///    }
/// }
/// ```
pub async fn retrieve_range(
    symbol: &str,
    start: DateTime<Utc>,
    end: Option<DateTime<Utc>>,
) -> Result<Vec<Bar>, YahooError> {
    // pre-conditions
    let end = end.unwrap_or_else(Utc::now);

    if end.signed_duration_since(start).num_seconds() > 0 {
        return Err(YahooError::InvalidStartDate);
    };

    aggregate_bars(yahoo::load_daily_range(symbol, start.timestamp(), end.timestamp()).await?)
}
