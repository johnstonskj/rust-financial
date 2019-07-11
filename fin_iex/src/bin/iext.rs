#[macro_use]
extern crate log;
extern crate flexi_logger;
extern crate num_format;

use num_format::{SystemLocale, ToFormattedString};

use fin_data::markets::ISORegistry;
use fin_data::classifiers::naics;
use fin_data::classifiers::uk_sic;
use fin_data::classifiers::us_sic;

use fin_model::classification::Code;
use fin_model::market::Market;
use fin_model::provider::Provider;
use fin_model::quote::FetchPriceQuote;
use fin_model::registry::Registry;
use fin_model::request::RequestError;

use fin_iex::IEXProvider;

enum Command {
    Price(String),
    Quote(String, bool),
    Lookup(String, String),
    None
}

fn main() {
    flexi_logger::Logger::with_env()
        .start()
        .unwrap();
    info!("iext::main started");

    let cmd = handle_args();

    match cmd {
        Command::Price(_) | Command::Quote(_,_) =>
            provider_commands(cmd),
        Command::Lookup(_,_) =>
            registry_commands(cmd),
        Command::None => ()
    }
}

// ------------------------------------------------------------------------------------------------

fn provider_commands(cmd: Command) {
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
        _ => ()
    }

    println!("Data provided by {} ({}).", provider.attribution(), provider.url());

    provider.finish();
}

fn registry_commands(cmd: Command) {
    match cmd {
        Command::Lookup(scheme, code) => {
            match scheme.as_str() {
                "mic" => {
                    let registry: ISORegistry = ISORegistry::new();
                    println!("Registry Scheme: {} ({})", registry.name(), registry.acronym());
                    println!(" Governing Body: {}", registry.governing_body());
                    if let Some(date) = registry.last_updated() {
                        println!("   Last updated: {}", date);
                    }
                    registry_lookup::<String, Market>(
                        &code,
                        registry.get(code.to_string()),
                        &|m:&Market| println!("{}: {} ({})", code, m.description, m.country_code,));
                },
                "naics" => {
                    let registry: naics::Scheme = naics::Scheme::new();
                    let code = code.parse::<u32>().unwrap();
                    println!("Registry Scheme: {} ({})", registry.name(), registry.acronym());
                    println!(" Governing Body: {}", registry.governing_body());
                    if let Some(date) = registry.last_updated() {
                        println!("   Last updated: {}", date);
                    }
                    registry_lookup::<u32, Code<u32>>(
                        &code,
                        registry.get(code),
                        &|c:&Code<u32>| println!("{}: {}", code, c.description));
                },
                "uksic" => {
                    let registry: uk_sic::Scheme = uk_sic::Scheme::new();
                    let code = code.parse::<u32>().unwrap();
                    println!("Registry Scheme: {} ({})", registry.name(), registry.acronym());
                    println!(" Governing Body: {}", registry.governing_body());
                    if let Some(date) = registry.last_updated() {
                        println!("   Last updated: {}", date);
                    }
                    registry_lookup::<u32, Code<u32>>(
                        &code,
                        registry.get(code),
                        &|c:&Code<u32>| println!("{}: {}", code, c.description));
                },
                "ussic" => {
                    let registry: us_sic::Scheme = us_sic::Scheme::new();
                    let code = code.parse::<u16>().unwrap();
                    println!("Registry Scheme: {} ({})", registry.name(), registry.acronym());
                    println!(" Governing Body: {}", registry.governing_body());
                    if let Some(date) = registry.last_updated() {
                        println!("   Last updated: {}", date);
                    }
                    registry_lookup::<u16, Code<u16>>(
                        &code,
                        registry.get(code),
                        &|c:&Code<u16>| println!("{}: {}", code, c.description));
                },
                _ => (),
            }
        },
        _ => ()
    }
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
        .subcommand(SubCommand::with_name("lookup")
            .about("Lookup a code from within a classification scheme")
            .arg(Arg::with_name("scheme")
                .short("s")
                .long("scheme")
                .takes_value(true)
                .help("Classifier scheme"))
            .arg(Arg::with_name("code")
                .help("The classifier code")
                .takes_value(true)
                .required(true)))
        .get_matches();

    match matches.subcommand() {
        ("price", Some(matches))      =>
            Command::Price(
                matches.value_of("symbol").unwrap().to_string()),
        ("quote", Some(matches))      =>
            Command::Quote(
                matches.value_of("symbol").unwrap().to_string(),
                matches.is_present("delayed")),
        ("lookup", Some(matches))     =>
            Command::Lookup(
                matches.value_of("scheme").unwrap().to_string(),
                matches.value_of("code").unwrap().to_string()),
        _ => {
            println!("Pick a [valid] command");
            Command::None
        }
    }
}
// ------------------------------------------------------------------------------------------------

fn registry_lookup<C: std::fmt::Display, T>(code: &C, value: Option<&T>, printer: &Fn(&T)) {
    match value {
        None =>
            println!("No value found for code {}", code),
        Some(value) =>
            printer(value)
    }
}