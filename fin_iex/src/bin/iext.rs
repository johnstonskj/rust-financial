#![cfg(feature = "iex-tool")]

#[macro_use]
extern crate log;
extern crate flexi_logger;
extern crate num_format;
#[macro_use]
extern crate prettytable;
extern crate term_size;

use num_format::{SystemLocale, ToFormattedString};

use prettytable::Table;

use fin_model::classification::Code;
use fin_model::market::Market;
use fin_model::prelude::*;
use fin_model::quote::{FetchPriceQuote, FetchPriceRangeSeries, SeriesInterval};

use fin_data::classifiers::naics;
use fin_data::classifiers::uk_sic;
use fin_data::classifiers::us_sic;
use fin_data::markets::ISORegistry;

use fin_iex::IEXProvider;

enum Command {
    Price(String),
    Quote(String, bool),
    Historical(String, String),
    Lookup(String, String),
    None,
}

fn main() {
    flexi_logger::Logger::with_env().start().unwrap();
    info!("iext::main started");

    let cmd = handle_args();

    match cmd {
        Command::Price(_) | Command::Quote(_, _) | Command::Historical(_, _) => {
            provider_commands(cmd)
        }
        Command::Lookup(_, _) => registry_commands(cmd),
        Command::None => (),
    }
}

// ------------------------------------------------------------------------------------------------

fn provider_commands(cmd: Command) {
    let provider = match IEXProvider::new() {
        Ok(provider) => provider,
        Err(RequestError::ConfigurationError(err)) => {
            println!("Error configuring provider: {}", err);
            return ();
        }
        Err(err) => {
            println!("Unknown error from provider: {:?}", err);
            return ();
        }
    };

    let locale = SystemLocale::default().unwrap();

    match cmd {
        Command::Price(symbol) => match provider.latest_price_only(symbol) {
            Err(e) => {
                println!("Call failed: {:?}", e);
            }
            Ok(p) => println!("Latest price: {}", p),
        },
        Command::Quote(symbol, true) => match provider.delayed(symbol) {
            Err(e) => {
                println!("Call failed: {:?}", e);
            }
            Ok(q) => {
                println!(
                    "Delayed price: {} (by {} minutes)",
                    q.data.latest.price, q.data.delayed_by
                );
                let table = table!(
                    ["Low", "High", "Volume"],
                    [
                        q.data.low,
                        q.data.high,
                        match q.data.volume {
                            None => "N/A".to_string(),
                            Some(v) => v.to_formatted_string(&locale),
                        }
                    ]
                );
                table.printstd();
            }
        },
        Command::Quote(symbol, false) => match provider.real_time(symbol) {
            Err(e) => {
                println!("Call failed: {:?}", e);
            }
            Ok(q) => {
                println!(
                    "Real-Time price: {} ({} {}%), updated {}",
                    q.data.latest.price,
                    q.data.latest.change.unwrap(),
                    q.data.latest.percentage.unwrap(),
                    q.date
                );
                if let Some(range) = q.data.range {
                    let table = table!(
                        ["Open", "Low", "High", "Close", "Volume"],
                        [
                            range.open,
                            range.low,
                            range.high,
                            range.close,
                            match range.volume {
                                None => "N/A".to_string(),
                                Some(v) => v.to_formatted_string(&locale),
                            }
                        ]
                    );
                    table.printstd();
                }
                if let Some(extended) = q.data.extended {
                    println!(" Extended price: {}", extended.price);
                }
            }
        },
        Command::Historical(symbol, interval) => {
            let interval = match interval.as_str() {
                "1d" => SeriesInterval::Day,
                "5d" => SeriesInterval::FiveDays,
                "1m" => SeriesInterval::OneMonth,
                "3m" => SeriesInterval::ThreeMonths,
                "6m" => SeriesInterval::SixMonths,
                "ytd" => SeriesInterval::YearToDate,
                "1y" => SeriesInterval::OneYear,
                "2y" => SeriesInterval::TwoYears,
                "5y" => SeriesInterval::FiveYears,
                _ => {
                    warn!("invalid interval: {}", interval);
                    return;
                }
            };
            match provider.last(symbol, interval) {
                Err(e) => {
                    println!("Call failed: {:?}", e);
                }
                Ok(series) => {
                    let mut table = Table::new();
                    table.add_row(row!["Date", "Open", "Low", "High", "Close", "Volume"]);
                    for range in series.series {
                        table.add_row(row![
                            range.date.date(),
                            range.data.open,
                            range.data.low,
                            range.data.high,
                            range.data.close,
                            match range.data.volume {
                                None => "N/A".to_string(),
                                Some(v) => v.to_formatted_string(&locale),
                            }
                        ]);
                    }
                    table.printstd();
                }
            }
        }
        _ => (),
    }

    println!(
        "Data provided by {} ({}).",
        provider.attribution(),
        provider.url()
    );

    provider.finish();
}

fn registry_commands(cmd: Command) {
    match cmd {
        Command::Lookup(scheme, code) => match scheme.as_str() {
            "mic" => {
                let registry: ISORegistry = ISORegistry::new();
                registry_details(&registry);
                registry_lookup::<String, Market>(
                    &code,
                    registry.get(code.to_string()),
                    &|m: &Market| println!("{}: {} ({})", code, m.description, m.country_code,),
                );
            }
            "naics" => {
                let registry: naics::Scheme = naics::Scheme::new();
                let code = code.parse::<u32>().unwrap();
                registry_details(&registry);
                registry_lookup::<u32, Code<u32>>(&code, registry.get(code), &|c: &Code<u32>| {
                    println!("{}: {}", code, c.description)
                });
            }
            "uksic" => {
                let registry: uk_sic::Scheme = uk_sic::Scheme::new();
                let code = code.parse::<u32>().unwrap();
                registry_details(&registry);
                registry_lookup::<u32, Code<u32>>(&code, registry.get(code), &|c: &Code<u32>| {
                    println!("{}: {}", code, c.description)
                });
            }
            "ussic" => {
                let registry: us_sic::Scheme = us_sic::Scheme::new();
                let code = code.parse::<u16>().unwrap();
                registry_details(&registry);
                registry_lookup::<u16, Code<u16>>(&code, registry.get(code), &|c: &Code<u16>| {
                    println!("{}: {}", code, c.description)
                });
            }
            _ => (),
        },
        _ => (),
    }
}

// ------------------------------------------------------------------------------------------------

extern crate clap;

use clap::{App, Arg, SubCommand};

fn handle_args() -> Command {
    let matches = App::new("iext")
        .about("IEX Tool")
        .version("v1.0-alpha")
        .subcommand(
            SubCommand::with_name("price")
                .about("Fetch latest price")
                .arg(
                    Arg::with_name("symbol")
                        .help("The security symbol")
                        .takes_value(true)
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("quote")
                .about("Fetch price quote")
                .arg(
                    Arg::with_name("delayed")
                        .short("d")
                        .long("delayed")
                        .help("Fetch delayed quotes only"),
                )
                .arg(
                    Arg::with_name("symbol")
                        .help("The security symbol")
                        .takes_value(true)
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("history")
                .about("Fetch price history")
                .arg(
                    Arg::with_name("interval")
                        .short("i")
                        .long("interval")
                        .takes_value(true)
                        .help("The interval over which to fetch prices"),
                )
                .arg(
                    Arg::with_name("symbol")
                        .help("The security symbol")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("lookup")
                .about("Lookup a code from within a classification scheme")
                .arg(
                    Arg::with_name("scheme")
                        .short("s")
                        .long("scheme")
                        .takes_value(true)
                        .help("Classifier scheme"),
                )
                .arg(
                    Arg::with_name("code")
                        .help("The classifier code")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("price", Some(matches)) => Command::Price(matches.value_of("symbol").unwrap().to_string()),
        ("quote", Some(matches)) => Command::Quote(
            matches.value_of("symbol").unwrap().to_string(),
            matches.is_present("delayed"),
        ),
        ("history", Some(matches)) => Command::Historical(
            matches.value_of("symbol").unwrap().to_string(),
            matches.value_of("interval").unwrap().to_string(),
        ),
        ("lookup", Some(matches)) => Command::Lookup(
            matches.value_of("scheme").unwrap().to_string(),
            matches.value_of("code").unwrap().to_string(),
        ),
        _ => {
            println!("Pick a [valid] command");
            Command::None
        }
    }
}
// ------------------------------------------------------------------------------------------------

fn registry_details<C: std::fmt::Display, T>(registry: &dyn Registry<C, T>) {
    println!(
        "Registry Scheme: {} ({})",
        registry.name(),
        registry.acronym()
    );
    println!(" Governing Body: {}", registry.governing_body());
    if let Some(date) = registry.last_updated() {
        println!("   Last updated: {}", date);
    }
}

fn registry_lookup<C: std::fmt::Display, T>(code: &C, value: Option<&T>, printer: &dyn Fn(&T)) {
    match value {
        None => println!("No value found for code {}", code),
        Some(value) => printer(value),
    }
}

#[allow(dead_code)]
fn get_term_width() -> Option<usize> {
    match term_size::dimensions() {
        Some((w, _h)) => Some(w),
        None => None,
    }
}

#[allow(dead_code)]
fn display_scale_mark(scale_mark: f64, max_width: Option<usize>) {
    let default_width = 25;
    let width = max_width.unwrap_or(default_width);
    let width = width - ((width - 5) % 4);
    let pad = (width - 5) / 4;
    let dashes = "-".repeat(pad);
    let scale = vec!["1", "2", "3", "4", "5"].join(dashes.as_str());
    let mark = scale_mark.to_string();
    let pos = ((scale_mark.trunc() as usize - 1) * (pad + 1))
        + (((pad + 1) as f64 * scale_mark.fract()) as usize);
    if pos > 3 && mark.len() < pos - 2 {
        println!("{} {} v", " ".repeat(pos - mark.len() - 2), mark);
    } else {
        println!("{}v {}", " ".repeat(pos), mark);
    }
    println!("{}", scale);
}
