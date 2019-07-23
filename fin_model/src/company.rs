/*!
Provides structs and traits that provide company information.

The majority of the data provided here is information published by
the company itself usually in regulatory filings.
*/

use crate::prelude::*;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// Information regarding the security that represents a company. This
/// includes the market and issue type for the security. Finally, for
/// those providers that enable it, a sector classification is included;
/// this value may be provided by the underlying market or simply the
/// provider itself.
pub struct Security {
    pub symbol: String,
    pub market: String,
    pub security_name: String,
    pub issue_type: Option<String>,
    pub sector: Option<String>,
}

/// Basic information reported about a company. This information, other
/// than the number of employees, rarely changes and can easily be cached.
pub struct About {
    pub company_name: String,
    pub web_site: String,
    pub employees: u32,
    pub description: String,
    pub logo_url: Option<String>,
    pub security: Security,
    pub tags: Vec<String>,
}

/// For data returned below, this indicates the time-scale represented
/// by the data.
pub enum ReportingPeriod {
    Quarter,
    Annual,
}

/// Reported income, categorized.
pub struct IncomeStatement {
    pub total_revenue: i64,
    pub cost_of_revenue: i64,
    pub gross_profit: i64,

    pub research_and_development: i64,
    pub sales_and_administration: i64,
    pub non_recurring: i64,
    pub other_operating_expense: i64,
    pub operating_income: i64,

    pub additional_income_or_expense: i64,
    pub earnings_before_interest_and_tax: i64,
    pub interest_expense: i64,
    pub earnings_before_tax: i64,
    pub income_tax_expense: i64,
    pub minority_interest: i64,
    pub equity_earnings: i64,
    pub net_income: i64,
    pub net_income_to_shareholders: i64,
}

/// Reported balance sheet information in detail.
pub struct BalanceSheet {
    pub current_cash: i64,
    pub short_term_investments: i64,
    pub net_receivables: i64,
    pub inventory: i64,
    pub other_current_assets: i64,
    pub total_current_assets: i64,

    pub long_term_investments: i64,
    pub property_plant_equipment: i64,
    pub goodwill: i64,
    pub intangible_assets: i64,
    pub accumulated_amortization: i64,
    pub other_assets: i64,
    pub deferred_asset_charges: i64,
    pub total_assets: i64,

    pub accounts_payable: i64,
    pub current_long_term_debt: i64,
    pub other_current_liabilities: i64,
    pub total_current_liabilities: i64,

    pub long_term_debt: i64,
    pub other_liabilities: i64,
    pub deferred_liability_charges: i64,
    pub minority_interest: i64,
    pub total_liabilities: i64,

    pub redeemable_preferred_stock: i64,
    pub preferred_stock: i64,
    pub common_stock: i64,
    pub retained_earnings: i64,
    pub treasury_stock: i64,
    pub capital_surplus: i64,
    pub other_shareholder_equity: i64,
    pub total_shareholder_equity: i64,
    pub net_tangible_assets: i64,
}

/// Major statistic calculated over time.
pub struct Statistics {
    pub shares_outstanding: u64,
    pub market_cap: i64,
    pub share_float: u64,
    pub pe_ratio: f64,
    pub beta: f64,

    pub t52w_high: Money,
    pub t52w_low: Money,
    pub t52w_change: f64,

    pub t10d_average_volume: u64,
    pub t30d_average_volume: u64,

    pub t50d_moving_average: Money,
    pub t200d_moving_average: Money,

    pub t12m_eps: Money,

    pub t12m_dividend_rate: f64,
    pub dividend_yield: f64,

    pub last_ex_dividend_date: Date,
    pub next_ex_dividend_date: Date,
    pub next_earnings_date: Date,

    pub t5d_change_percentage: f64,
    pub t30d_change_percentage: f64,
    pub t1m_change_percentage: f64,
    pub t3m_change_percentage: f64,
    pub t6m_change_percentage: f64,
    pub t1y_change_percentage: f64,
    pub t2y_change_percentage: f64,
    pub t5y_change_percentage: f64,

    pub ytd_change_percentage: f64,
    pub max_change_percentage: f64,
}

pub enum TypedUrl {
    HTML(String),
    PDF(String),
    Word(String),
    Excel(String),
}

/// Details of an individual filing.
pub struct RegulatoryFiling {
    pub identifier: String,
    pub form_type: String,
    pub description: String,
    pub links: Vec<TypedUrl>,
}

pub type RegulatoryFilings = Vec<Snapshot<RegulatoryFiling>>;

// ------------------------------------------------------------------------------------------------
// Public Traits
// ------------------------------------------------------------------------------------------------

/// Fetch basic information about a company.
pub trait FetchCompanyInformation {
    fn about(&self, for_symbol: Symbol) -> RequestResult<About>;

    fn filings(
        &self,
        for_symbol: Symbol,
        start_date: Option<Date>,
        form_type: Option<String>,
    ) -> RequestResult<RegulatoryFilings>;
}

/// Fetch ongoing financial information for a company.
pub trait FetchCompanyFinancials {
    fn reported_income(
        &self,
        for_symbol: Symbol,
        last: u8,
        period: ReportingPeriod,
    ) -> RequestResult<Vec<Snapshot<IncomeStatement>>>;

    fn reported_balance_sheet(
        &self,
        for_symbol: Symbol,
        last: u8,
        period: ReportingPeriod,
    ) -> RequestResult<Vec<Snapshot<BalanceSheet>>>;
}

/// Fetch basic statistics regarding a company.
pub trait FetchCompanyStatistics {
    fn core_statistics(&self, for_symbol: Symbol) -> RequestResult<Statistics>;
}
