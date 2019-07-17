use std::collections::HashMap;
use std::str::FromStr;

use serde;
use serde::{Serialize, Deserialize};

use fin_model::prelude::*;
use fin_model::analysis::*;
use fin_model::reporting::FinancialPeriod;
use fin_model::symbol::is_valid;

use crate::IEXProvider;
use crate::internal::convert::*;
use crate::internal::metric::{ApiName, record_api_use};
use crate::internal::request;

// ------------------------------------------------------------------------------------------------
// API Types (internal)
// ------------------------------------------------------------------------------------------------

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
struct IEXPriceTarget {
    symbol: String,
    updated_date: String,
    price_target_average: f64,
    price_target_high: f64,
    price_target_low: f64,
    number_of_analysts: f64
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
struct IEXRecommendationTrends {
    consensus_end_date: f64,
    consensus_start_date: f64,
    corporate_actions_applied_date: f64,
    rating_buy: f64,
    rating_hold: f64,
    rating_none: f64,
    rating_sell: f64,
    rating_overweight: f64,
    rating_underweight: f64,
    rating_scale_mark: f64
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
struct IEXEstimates {
    symbol: String,
    estimates: Vec<IEXEstimateData>
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
struct IEXEstimateData {
    #[serde(rename = "consensusEPS")]
    consensus_eps: f64,
    number_of_estimates: f64,
    fiscal_period: String,
    fiscal_end_date: String,
    report_date : String
}

// ------------------------------------------------------------------------------------------------
// Trait Implementations
// ------------------------------------------------------------------------------------------------

impl Peers for IEXProvider {

    fn peers(&self, for_symbol: Symbol) -> RequestResult<Symbols> {
        debug!("IEXProvider::<Peers>::peers for_symbol: {}", for_symbol);
        assert_is_valid!(for_symbol);

        let api_url = self.make_api_url(
            format!("/stock/{}/peers", for_symbol),
            None);

        let response: RequestResult<Symbols> = request::make_json_call(api_url);
        match response {
            Ok(values) => {
                record_api_use(ApiName::Peers);
                Ok(values)
            },
            Err(err) => {
                println!("IEXProvider::<Peers>::peers returned error: {:?}", err);
                Err(err)
            }
        }
    }
}

impl AnalystRecommendations for IEXProvider {

    fn target_price(&self, for_symbol: Symbol) -> RequestResult<Snapshot<PriceTarget>> {
        debug!("IEXProvider::<AnalystRecommendations>::target_price for_symbol: {}", for_symbol);
        assert_is_valid!(for_symbol);

        let api_url = self.make_api_url(
            format!("/stock/{}/price-target", for_symbol),
            None);

        let response: RequestResult<IEXPriceTarget> = request::make_json_call(api_url);
        let dc = self.get_default_currency();
        match response {
            Ok(target) => {
                record_api_use(ApiName::TargetPrice);
                Ok(Snapshot {
                    date: datetime_from_date_string(&target.updated_date)?,
                    data: PriceTarget {
                        high: price_from_float(dc, target.price_target_high)?,
                        low: price_from_float(dc, target.price_target_low)?,
                        average: price_from_float(dc, target.price_target_average)?,
                        number_of_analysts: target.number_of_analysts as u32
                    }
                })
            },
            Err(err) => {
                println!("IEXProvider::<AnalystRecommendations>::target_price returned error: {:?}", err);
                Err(err)
            }
        }
    }

    fn consensus_rating(&self, for_symbol: Symbol) -> RequestResult<Vec<Bounded<Ratings>>> {
        debug!("IEXProvider::<AnalystRecommendations>::consensus_rating for_symbol: {}", for_symbol);
        assert_is_valid!(for_symbol);

        let api_url = self.make_api_url(
            format!("/stock/{}/recommendation-trends", for_symbol),
            None);

        let response: RequestResult<Vec<IEXRecommendationTrends>> = request::make_json_call(api_url);
        match response {
            Ok(consensus) => {
                record_api_use(ApiName::ConsensusRatings);
                let series: RequestResult<Vec<Bounded<Ratings>>> = consensus.iter().map(|v|
                    to_rating(v)
                ).collect();
                match series {
                    Ok(data) => Ok(data),
                    Err(err) => Err(err)
                }
            },
            Err(err) => {
                println!("IEXProvider::<AnalystRecommendations>::consensus_rating returned error: {:?}", err);
                Err(err)
            }
        }
    }

    fn consensus_eps(&self, for_symbol: Symbol) -> RequestResult<Vec<EPSConsensus>> {
        debug!("IEXProvider::<AnalystRecommendations>::consensus_eps for_symbol: {}", for_symbol);
        assert_is_valid!(for_symbol);

        let api_url = self.make_api_url(
            format!("/stock/{}/recommendation-trends", for_symbol),
            None);

        let response: RequestResult<IEXEstimates> = request::make_json_call(api_url);
        let dc = self.get_default_currency();
        match response {
            Ok(estimates) => {
                record_api_use(ApiName::ConsensusEPS);
                let series: RequestResult<Vec<EPSConsensus>> = estimates.estimates.iter().map(|v|
                    to_estimate(dc, v)
                ).collect();
                match series {
                    Ok(data) => Ok(data),
                    Err(err) => Err(err)
                }
            },
            Err(err) => {
                println!("IEXProvider::<AnalystRecommendations>::consensus_eps returned error: {:?}", err);
                Err(err)
            }
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Implementations
// ------------------------------------------------------------------------------------------------

fn to_rating(v : &IEXRecommendationTrends) -> RequestResult<Bounded<Ratings>> {
    let mut ratings: HashMap<RatingType, Counter> = HashMap::new();
    ratings.insert(RatingType::Buy, v.rating_buy as Counter);
    ratings.insert(RatingType::Hold, v.rating_hold as Counter);
    ratings.insert(RatingType::Sell, v.rating_sell as Counter);
    ratings.insert(RatingType::Underweight, v.rating_underweight as Counter);
    ratings.insert(RatingType::Overweight, v.rating_overweight as Counter);

    Ok(Bounded {
        start_date: date_from_timestamp(v.consensus_start_date)?,
        end_date: date_from_timestamp(v.consensus_end_date)?,
        data: Ratings {
            ratings,
            scale_mark: Some(v.rating_scale_mark as f32)
        }
    })
}

fn to_estimate(dc: &String, v : &IEXEstimateData) -> RequestResult<EPSConsensus> {
    Ok(EPSConsensus {
        consensus: price_from_float(dc, v.consensus_eps)?,
        number_of_estimates: v.number_of_estimates as Counter,
        fiscal_period: FinancialPeriod::from_str(v.fiscal_period.as_str()).unwrap(),
        fiscal_end_date: date_from_string(&v.fiscal_end_date)?,
        next_report_date: date_from_string(&v.report_date)?,
    })
}