/*!
IEX API wrapper
*/

use serde;
use serde::{Deserialize, Serialize};

use fin_model::news::*;
use fin_model::prelude::*;
use fin_model::symbol::is_valid;

use crate::internal::convert::*;
use crate::internal::metric::{record_api_use, ApiName};
use crate::internal::request;
use crate::IEXProvider;

// ------------------------------------------------------------------------------------------------
// API Types (internal)
// ------------------------------------------------------------------------------------------------

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct IEXNewsItem {
    pub datetime: String,
    pub headline: String,
    pub source: String,
    pub url: String,
    pub summary: String,
    pub related: String,
    pub image: String,
    pub lang: String,
    pub has_paywall: bool,
}

pub type IEXNewsFeed = Vec<IEXNewsItem>;

// ------------------------------------------------------------------------------------------------
// Trait Implementations
// ------------------------------------------------------------------------------------------------

impl FetchNews for IEXProvider {
    fn latest_news(&self, for_symbol: Symbol, max_items: usize) -> RequestResult<NewsFeed> {
        debug!(
            "IEXProvider::<FetchNews>::latest_news for_symbol: {}; max items: {}",
            for_symbol, max_items
        );
        assert_is_valid!(for_symbol);

        if max_items < 1 || max_items > 50 {
            return Err(RequestError::BadRequestError);
        }

        let api_url = self.make_api_url(
            format!("stock/{}/news/last/{}", for_symbol, max_items),
            None,
        );

        let response: RequestResult<IEXNewsFeed> = request::make_json_call(api_url);
        match response {
            Ok(values) => {
                record_api_use(ApiName::NewsFeed);
                let series: RequestResult<NewsFeed> =
                    values.iter().map(|v| to_news_item(v)).collect();
                series
            }
            Err(err) => {
                warn!(
                    "IEXProvider::<FetchNews>::latest_news returning error: {:?}",
                    err
                );
                Err(err)
            }
        }
    }

    fn news_from(
        &self,
        for_symbol: Symbol,
        start_date: Date,
        max_items: usize,
    ) -> RequestResult<NewsFeed> {
        debug!(
            "IEXProvider::<FetchNews>::news_from for_symbol: {}; start date: {}, max items: {}",
            for_symbol, start_date, max_items
        );
        assert_is_valid!(for_symbol);
        Err(RequestError::Unsupported)
    }
}

fn to_news_item(v: &IEXNewsItem) -> RequestResult<Snapshot<NewsItem>> {
    Ok(Snapshot {
        date: datetime_from_date_string(&v.datetime)?,
        data: NewsItem {
            headline: v.headline.to_string(),
            source: v.source.to_string(),
            url: v.url.to_string(),
            summary: v.summary.to_string(),
            image_url: Some(v.image.to_string()),
            language: Some(v.lang.to_string()),
            categories: None,
            sub_categories: None,
        },
    })
}
