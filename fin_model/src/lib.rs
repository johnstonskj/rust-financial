extern crate steel_cent;
extern crate chrono;

use chrono::{DateTime,Local};

pub type Symbol = String;

pub struct QualifiedSymbol {
    market: Symbol,
    symbol: Symbol
}

pub struct Snapshot<T> {
    date: DateTime<Local>,
    data: T
}

pub struct Bounded<T> {
    start_date: DateTime<Local>,
    end_date: DateTime<Local>,
    data: T
}

pub mod quote;

pub mod analysis;

pub mod provider;

pub mod request;
