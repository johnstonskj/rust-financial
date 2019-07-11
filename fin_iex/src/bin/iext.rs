#[macro_use]
extern crate log;
extern crate flexi_logger;

use num_format::{SystemLocale, ToFormattedString};

use fin_model::provider::Provider;
use fin_model::quote::FetchPriceQuote;
use fin_model::request::RequestError;

use fin_iex::IEXProvider;

enum Command {
    Price(String),
    Quote(String, bool),
    None
}

fn main() {
    flexi_logger::Logger::with_env()
        .start()
        .unwrap();
    info!("iext::main started");

    let cmd = handle_args();

    let provider = match IEXProvider::new() {
        Ok(provider) => provider,
        Err(RequestError::ConfigurationError(err)) => {
            println!("Error configuring provider: {}", err);
            return ();
        },
        Err(err) => {
            println!("Unknown error from provider: {:?}", err);
            return ();
        }
    };

    let locale = SystemLocale::default().unwrap();

    match cmd {
        Command::Price(symbol) => {
            match provider.latest_price_only(symbol) {
                Err(e) => {
                    println!("Call failed: {:?}", e);
                },
                Ok(p) => {
                    println!("Latest price: {}", p)
                }
            }
        },
        Command::Quote(symbol, true) => {
            match provider.delayed(symbol) {
                Err(e) => {
                    println!("Call failed: {:?}", e);
                },
                Ok(q) => {
                    println!("Delayed price: {} (by {} minutes)",
                             q.data.latest.price,
                             q.data.delayed_by);
                    println!(" Price Ranges: {} ... {} @ {}",
                             q.data.low,
                             q.data.high,
                             match q.data.volume {
                                 None => "N/A".to_string(),
                                 Some(v) => v.to_formatted_string(&locale)
                             });
                }
            }
        },
        Command::Quote(symbol, false) => {
            match provider.real_time(symbol) {
                Err(e) => {
                    println!("Call failed: {:?}", e);
                },
                Ok(q) => {
                    println!("Real-Time price: {} ({} {}%), updated {} {:?}",
                             q.data.latest.price,
                             q.data.latest.change.unwrap(),
                             q.data.latest.percentage.unwrap(),
                             q.date,
                             q.data.latest_source);
                    println!("   Price Ranges: {} {} ... {} {} @ {})",
                             q.data.range.open,
                             q.data.range.low,
                             q.data.range.high,
                             q.data.range.close,
                             match q.data.range.volume {
                                 None => "N/A".to_string(),
                                 Some(v) => v.to_formatted_string(&locale)
                             });
                    if let Some(extended) = q.data.extended {
                        println!(" Extended price: {}",
                                 extended.price);
                    }
                }
            }
        },
        Command::None => ()
    }

    println!("Data provided by {} ({}).", provider.attribution(), provider.url());
}

// ------------------------------------------------------------------------------------------------

extern crate clap;

use clap::{App, Arg, SubCommand};

fn handle_args() -> Command {
    let matches = App::new("iext")
        .about("IEX Tool")
        .version("v1.0-alpha")
        .subcommand(SubCommand::with_name("price")
            .about("Fetch latest price")
            .arg(Arg::with_name("symbol")
                .help("The security symbol")
                .required(true)
                .index(1)))
        .subcommand(SubCommand::with_name("quote")
            .about("Fetch price quote")
            .arg(Arg::with_name("delayed")
                .short("d")
                .long("delayed")
                .help("Fetch delayed quotes only"))
            .arg(Arg::with_name("symbol")
                .help("The security symbol")
                .required(true)
                .index(1)))
        .get_matches();

    match matches.subcommand() {
        ("price", Some(matches))      =>
            Command::Price(
                matches.value_of("symbol").unwrap().to_string()),
        ("quote", Some(matches))      =>
            Command::Quote(
                matches.value_of("symbol").unwrap().to_string(),
                matches.is_present("delayed")),
        _ => {
            println!("Pick a [valid] command");
            Command::None
        }
    }
}
