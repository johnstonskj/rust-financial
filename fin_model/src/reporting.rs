/*!
Provides common types for reporting data.

Primarily this module provides the `FinancialPeriod` type that can represent
periods such as "Q2 2019", "H1 2018", or "2020". This period may be used as a
calendar period where the assumption is that the periods are relative to
January 1st, or as a fiscal period where the start of a fiscal year may not
align with calendar years. For this latter case the `FiscalPeriod` struct
also contains a date for the fiscal year start.

The financial period type implements both `fmt::Display` and `str::FromStr`
and so supports the ability to read and write the period as a string in a
common manner.
*/

use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

use regex::Regex;

use crate::prelude::*;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// Represents common financial periods, whole years, half years, and quarter
/// years.
///
/// By default the assumption is that periods represent calendar time;
/// for example, the values "2019", "Q1 2019", and "H1 2019" are all assumed
/// to start on January 1st.
#[derive(Debug, PartialEq)]
pub enum FinancialPeriod {
    Quarter {
        /// the quarter within the year (values: 1..4)
        quarter: u8,
        /// the year itself (values: 1900..9999)
        year: u16},
    Half {
        /// the half of the year (values: 1..2)
        half: u8,
        /// the year itself (values: 1900..9999)
        year: u16},
    Year {
        /// the year itself (values: 1900..9999)
        year: u16}
}

/// Represents a fiscal periods, with a reference start date, allowing it
/// to represent years that do not align with the calendar start date.
///
/// For example, "Q1 2019" with a start date of April 1st ends on June 30th.
#[derive(Debug, PartialEq)]
pub struct FiscalPeriod {
    /// the period within the fiscal year
    pub period: FinancialPeriod,
    /// the start date for the fiscal year
    pub fiscal_year_start_date: Date
}

// ------------------------------------------------------------------------------------------------
// Trait Implementations
// ------------------------------------------------------------------------------------------------

impl FinancialPeriod {
    /// Validate the period to ensure that quarter, half, and year
    /// values are within correct ranges.
    pub fn is_valid(&self) -> bool {
        match self {
            FinancialPeriod::Quarter {quarter, year} =>
                *quarter >= 1 && *quarter <= 4
                    && is_valid_year(*year),
            FinancialPeriod::Half {half, year} =>
                *half >= 1 && *half <= 2
                    && is_valid_year(*year),
            FinancialPeriod::Year {year} =>
                is_valid_year(*year)
        }
    }
}

impl Display for FinancialPeriod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FinancialPeriod::Quarter {quarter, year} =>
                write!(f, "Q{} {}", quarter, year),
            FinancialPeriod::Half {half, year} =>
                write!(f, "H{} {}", half, year),
            FinancialPeriod::Year {year} =>
                write!(f, "{}", year),
        }
    }
}

/// Errors that can result from parsing a `FinancialPeriod` from a string.
#[derive(Debug, PartialEq)]
pub enum ParseError {
    /// the string is empty
    EmptyString,
    /// the string is syntactically invalid
    InvalidPeriodString,
    /// the string parsed correctly but failed validation
    InvalidPeriodValue
}

impl FromStr for FinancialPeriod {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(([QH])(\d) )?(\d{4})$").unwrap();
        }

        if s.len() == 0 {
            return Err(ParseError::EmptyString);
        }

        let period = match RE.captures(s) {
            None => Err(ParseError::InvalidPeriodString),
            Some(captures) => {
                if let Some(period) = captures.get(2) {
                    if period.as_str() == "Q" {
                        Ok(FinancialPeriod::Quarter {
                            quarter: u8::from_str(&captures[3]).unwrap(),
                            year: u16::from_str(&captures[4]).unwrap()
                        })
                    } else {
                        Ok(FinancialPeriod::Half {
                            half: u8::from_str(&captures[3]).unwrap(),
                            year: u16::from_str(&captures[4]).unwrap()
                        })
                    }
                } else {
                    Ok(FinancialPeriod::Year {
                        year: u16::from_str(&captures[4]).unwrap()
                    })
                }
            }
        };
        match period {
            Err(e) => Err(e),
            Ok(period) => if period.is_valid() {
                Ok(period)
            } else {
                Err(ParseError::InvalidPeriodValue)
            }
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn is_valid_year(year: u16) -> bool {
    year >= 1900 && year <= 9999
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::{is_valid_year, FinancialPeriod, ParseError};
    use std::str::FromStr;

    #[test]
    fn test_is_valid_year() {
        assert_eq!(is_valid_year(1972), true);
        assert_eq!(FinancialPeriod::Year {year: 1972}.is_valid(), true);

        assert_eq!(is_valid_year(1492), false);
        assert_eq!(FinancialPeriod::Year {year: 1492}.is_valid(), false);
    }

    #[test]
    fn test_is_valid_quarter() {
        assert_eq!(FinancialPeriod::Quarter {quarter: 2, year: 1972}.is_valid(), true);

        assert_eq!(FinancialPeriod::Quarter {quarter: 5, year: 1972}.is_valid(), false);
    }

    #[test]
    fn test_is_valid_half() {
        assert_eq!(FinancialPeriod::Half {half: 1, year: 1972}.is_valid(), true);

        assert_eq!(FinancialPeriod::Half {half: 5, year: 1972}.is_valid(), false);
    }

    #[test]
    fn test_to_string() {
        assert_eq!(FinancialPeriod::Quarter {quarter: 2, year: 1972}.to_string(), "Q2 1972".to_string());
        assert_eq!(FinancialPeriod::Half {half: 2, year: 1972}.to_string(), "H2 1972".to_string());
        assert_eq!(FinancialPeriod::Year {year: 1972}.to_string(), "1972".to_string());
    }

    #[test]
    fn test_from_string() {
        assert_eq!(FinancialPeriod::from_str("Q2 1972").unwrap(), FinancialPeriod::Quarter {quarter: 2, year: 1972});
        assert_eq!(FinancialPeriod::from_str("H2 1972").unwrap(), FinancialPeriod::Half {half: 2, year: 1972});
        assert_eq!(FinancialPeriod::from_str("1972").unwrap(), FinancialPeriod::Year {year: 1972});

        assert_eq!(FinancialPeriod::from_str("").err().unwrap(), ParseError::EmptyString);

        assert_eq!(FinancialPeriod::from_str("Y1972").err().unwrap(), ParseError::InvalidPeriodString);

        assert_eq!(FinancialPeriod::from_str("1492").err().unwrap(), ParseError::InvalidPeriodValue);
    }
}