/*!
*/

use crate::prelude::*;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub struct Security {
    pub symbol: String,
    pub market: String,
    pub security_name: String,
    pub issue_type: Option<String>,
    pub sector: Option<String>
}

pub struct About {
    pub company_name: String,
    pub web_site: String,
    pub employees: u32,
    pub description: String,
    pub logo_url: Option<String>,
    pub security: Security,
    pub tags: Vec<String>
}

pub enum ReportingPeriod {
    Quarter,
    Annual
}

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
    pub net_income_to_shareholders: i64
}

pub struct BalanceSheet {
    pub current_cash: i64,
    pub short_term_investments: i64,
    pub receivables: i64,
    pub inventory: i64,
    pub current_assets: i64,
    pub other_current_assets: i64,
    pub intangible_assets: i64,
    pub other_assets: i64,
    pub total_assets: i64,
    pub long_term_investments: i64,
    pub property_plant_equipment: i64,
    pub goodwill: i64,
    pub accounts_payable: i64,
    pub current_long_term_debt: i64,
    pub other_current_liabilities: i64,
    pub total_current_liabilities: i64,
    pub long_term_debt: i64,
    pub other_liabilities: i64,
    pub minority_interest: i64,
    pub total_liabilities: i64,
    pub common_stock: i64,
    pub retained_earnings: i64,
    pub treasury_stock: i64,
    pub capital_surplus: i64,
    pub shareholder_equity: i64,
    pub net_tangible_assets: i64
}

// ------------------------------------------------------------------------------------------------
// Public Traits
// ------------------------------------------------------------------------------------------------

pub trait FetchCompanyInformation {

    fn about(&self, for_symbol: Symbol) -> RequestResult<About>;
}

pub trait FetchCompanyFinancials {

    fn reported_income(&self, for_symbol: Symbol, last: u8, period: ReportingPeriod)
        -> RequestResult<Vec<Snapshot<IncomeStatement>>>;

    fn reported_balance_sheet(&self, for_symbol: Symbol, last: u8, period: ReportingPeriod)
                       -> RequestResult<Vec<Snapshot<BalanceSheet>>>;
}