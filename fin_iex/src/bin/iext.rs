#[macro_use]
extern crate log;
extern crate flexi_logger;

use fin_model::provider::Provider;
use fin_model::quote::FetchPriceQuote;

use fin_iex::IEXProvider;

fn main() {
    flexi_logger::Logger::with_env()
        .start()
        .unwrap();
    info!("iext::main started");

    let provider = match IEXProvider::new() {
        Ok(provider) => provider,
        Err(err) => panic!("Error configuring provider, {:?}", err)
    };

    println!("{}. ({})", provider.attribution(), provider.url());

    println!("{}", provider.latest_price_only("AMZN".to_string()).unwrap());
}