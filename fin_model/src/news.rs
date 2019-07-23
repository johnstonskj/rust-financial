/*!
Provides structs and traits that retrieve news.

This module provides a trait to fetch news associated with a symbol. In most cases this is the
symbol for a security and fetches press stories related to that symbol (or it's segment). The
additional trait, `FetchCategoryNews`, which allows for fetching news not by symbol but by
category classification. This is optional as only some providers expose categorization as a
part of their news feeds.
*/

use crate::prelude::*;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// A single news item, or story.
pub struct NewsItem {
    /// The headline for the story
    pub headline: String,
    /// The source, either URL or name, of the story
    pub source: String,
    /// The click-through URL
    pub url: String,
    /// A brief summary, or preview, of the story
    pub summary: String,
    /// An optional link to an image for the story
    pub image_url: Option<String>,
    /// The language the story is in
    pub language: Option<String>,
    /// The category, or categories, applicable to this story
    pub categories: Option<Vec<String>>,
    /// The sub-category, or sub-categories, applicable to this story
    pub sub_categories: Option<Vec<String>>,
}

/// A news feed is a vector of items, each with a publication date
pub type NewsFeed = Vec<Snapshot<NewsItem>>;

// ------------------------------------------------------------------------------------------------
// Public Traits
// ------------------------------------------------------------------------------------------------

/// This trait supports the fetching of news items keyed by a symbol. It supports
/// either the latest items working back from _now_, or items working forward from
/// _start date_.
pub trait FetchNews {
    fn latest_news(&self, for_symbol: Symbol, max_items: usize) -> RequestResult<NewsFeed>;

    fn news_from(
        &self,
        for_symbol: Symbol,
        start_date: Date,
        max_items: usize,
    ) -> RequestResult<NewsFeed>;
}

/// This trait supports the fetching of news items by category and sub-category. It
/// supports either the latest items working back from _now_, or items working
/// forward from _start date_.
pub trait FetchCategoryNews {
    fn latest_news(
        &self,
        category: String,
        sub_category: Option<String>,
        max_items: usize,
    ) -> RequestResult<NewsFeed>;

    fn news_from(
        &self,
        category: String,
        sub_category: Option<String>,
        start_date: Date,
        max_items: usize,
    ) -> RequestResult<NewsFeed>;
}
