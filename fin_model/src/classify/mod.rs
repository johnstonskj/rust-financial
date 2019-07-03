use chrono::{NaiveDate};

#[derive(Debug, Clone)]
pub struct Code<T> {
    pub code: T,
    pub parent_code: Option<T>,
    pub description: String
}

pub trait ClassificationScheme<T>
    where Self: std::marker::Sized {

    fn new() -> Self;

    fn name() -> String;

    fn acronym() -> String;

    fn source() -> String;

    fn governing_body() -> Option<String>;

    fn last_updated() -> Option<NaiveDate>;

    fn get(&self, code: T) -> Option<&Code<T>>;

    fn get_children(&self, parent: T) -> Option<Vec<&Code<T>>>;
}

pub mod uk_sic;

pub mod us_sic;