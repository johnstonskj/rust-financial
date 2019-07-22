/*!
Provides structs and traits that retrieve news.
*/

use crate::prelude::*;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub struct NewsItem {
    pub headline: String,
    pub source: String,
    pub url: String,
    pub summary: String,
    pub image_url: String,
    pub language: Option<String>,
    pub categories: Option<Vec<String>>,
    pub sub_categories: Option<Vec<String>>,
}

pub type NewsFeed = Vec<Snapshot<NewsItem>>;

// ------------------------------------------------------------------------------------------------
// Public Traits
// ------------------------------------------------------------------------------------------------

pub trait FetchNews {
    fn news(&self, for_symbol: Symbol) -> RequestResult<NewsFeed>;
}
