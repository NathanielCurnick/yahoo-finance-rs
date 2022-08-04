use futures::executor::block_on;
use tokio::runtime::Runtime;
use yahoo_finance::history;

#[test]
fn mock() {
    let rt = Runtime::new().unwrap();
    let future = call_history();

    rt.block_on(future);
}

async fn call_history() {
    let data = history::retrieve("AAPL").await;
    match data {
        Ok(x) => println!("OK"),
        Err(y) => println!("Err {:?}", y),
    };
    // for bar in &data {
    //     println!("On {} Apple closed at ${:.2}", bar.timestamp, bar.close);
    // }

    let data = history::retrieve("eriygfuikygefkurg").await;
    match data {
        Ok(x) => println!("OK"),
        Err(y) => println!("Err {:?}", y),
    };
}
