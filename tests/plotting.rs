use std::convert::TryInto;

use plotly::{common::Mode, Candlestick, Plot, Scatter};
use tokio::runtime::Runtime;
use yahoo_finance::history;

#[test]
fn plot_candlestick() {
    let rt = Runtime::new().unwrap();
    let future = call_history();

    rt.block_on(future);
}

async fn call_history() {
    let data = match history::retrieve("AAPL").await {
        Ok(x) => x,
        Err(y) => panic!("Error in obtaining data {:?}", y),
    };

    data.plot_candlestick_interactive();
}
