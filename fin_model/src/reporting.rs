/*!
*/

use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

use regex::Regex;

// ------------------------------------------------------------------------------------------------
// PUBLIC TYPES
// ------------------------------------------------------------------------------------------------

#[derive(Debug, PartialEq)]
pub enum FinancialPeriod {
    Quarter {quarter: u8, year: u16},
    Half {half: u8, year: u16},
    Year {year: u16}
}

impl FinancialPeriod {
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

fn is_valid_year(year: u16) -> bool {
    year >= 1900 && year <= 9999
}

// ------------------------------------------------------------------------------------------------

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

#[derive(Debug, PartialEq)]
pub enum ParseError {
    EmptyString,
    InvalidPeriodString,
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