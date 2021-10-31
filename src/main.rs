mod hello;

use hello::hello_handler;
use lambda_runtime::{handler_fn, Error};
use log::LevelFilter;
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // required to enable CloudWatch error logging by the runtime
    // can be replaced with any other method of initializing `log`
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();
    log::info!("Hello lambda is warming up");

    let func = handler_fn(hello_handler);
    lambda_runtime::run(func).await?;
    Ok(())
}
