/*!
Provides structs and traits that represent common market analysis.
*/

use std::collections::HashMap;

use crate::prelude::*;
use crate::reporting::FinancialPeriod;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// Used to count things.
pub type Counter = u32;

/// The type of an analyst recommendation/position.
#[derive(PartialEq, Eq, Hash)]
pub enum RatingType {
    /// Also known as strong buy and _on the recommended list_. Needless to say,
    /// buy is a recommendation to purchase a specific security.
    Buy,
    /// Also known as _moderate buy_, _accumulate_, and _overweight_. Outperform
    /// is an analyst recommendation meaning a stock is expected to do slightly
    /// better than the market return.
    Outperform,
    /// In general terms, a company with a hold recommendation is expected to
    /// perform at the same pace as comparable companies or in-line with the market.
    Hold,
    /// A recommendation that means a stock is expected to do slightly worse than
    /// the overall stock market return. Underperform can also be expressed as
    /// _moderate sell_, _weak hold_, and _underweight_.
    Underperform,
    /// Also known as strong sell, it's a recommendation to sell a security or
    /// to liquidate an asset.
    Sell,
}

/// The set of recommendation trends over some period of time.
pub struct Ratings {
    /// a mapping of available rating types to counts, not all types may be available
    pub ratings: HashMap<RatingType, Counter>,
    /// a standardized represention of the consensus of recommendations
    pub scale_mark: Option<f32>,
}

/// Consensus price targets; high, low, and average.
pub struct PriceTarget {
    /// anticipated high price
    pub high: Money,
    /// anticipated low price
    pub low: Money,
    /// anticipated average price
    pub average: Money,
    /// number of analysts that provided recommendations
    pub number_of_analysts: Counter,
}

/// Consensus Earnings per Share (EPS) targets for some fiscal period.
pub struct EPSConsensus {
    /// anticipated earnings per share
    pub consensus: Money,
    /// number of analysts that provided recommendations
    pub number_of_estimates: Counter,
    /// expected for this period
    pub fiscal_period: FinancialPeriod,
    /// the company's end date for `fiscal_period`
    pub fiscal_end_date: Date,
    /// anticipated next reporting date
    pub next_report_date: Date,
}

// ------------------------------------------------------------------------------------------------
// Public Traits
// ------------------------------------------------------------------------------------------------

impl Ratings {
    /// Calculate the scaled/weighted average of the current set of ratings.
    pub fn scaled_average(&self) -> f64 {
        let (count, total) = self.ratings.iter().fold((0, 0), |(c, t), (k, v)| {
            (
                c + *v,
                t + match *k {
                    RatingType::Buy => 1,
                    RatingType::Outperform => 2,
                    RatingType::Hold => 3,
                    RatingType::Underperform => 4,
                    RatingType::Sell => 5,
                } * *v,
            )
        });
        f64::from(total) / f64::from(count)
    }
}

/// This trait is implemented by providers to return a set of symbols that are expected
/// to represent peer companies to `for_symbol`. This set of peers could be provided by
/// the market or the service provider itself.
pub trait Peers {
    /// Return a set of peer symbols.
    fn peers(&self, for_symbol: Symbol) -> RequestResult<Symbols>;
}

/// This trait is implemented by providers to return various analyst recommendations.
pub trait AnalystRecommendations {
    /// Return the target price recommendations for the symbol
    fn target_price(&self, for_symbol: Symbol) -> RequestResult<Snapshot<PriceTarget>>;

    /// Return the consensus ratings for the symbol
    fn consensus_rating(&self, for_symbol: Symbol) -> RequestResult<Vec<Bounded<Ratings>>>;

    /// Return the consensus earnings per share (EPS) for the symbol
    fn consensus_eps(&self, for_symbol: Symbol) -> RequestResult<Vec<EPSConsensus>>;
}
