/*!
Provides types and functions for market symbols.
*/

/// Type for a market ticker symbol. Consumers of symbols should use
/// `is_valid` to ensure the value they are provided is at least
/// syntactically correct.
pub type Symbol = String;

/// Maximum length assuming a root length of 6 and a 2 character suffix
pub const MAX_SYMBOL_LENGTH: usize = 8;

/// is the symbol valid; currently only string lengths are checked
#[inline(always)]
pub fn is_valid(symbol: Symbol) -> bool {
    symbol.len() >= 1 && symbol.len() <= MAX_SYMBOL_LENGTH
}

/// Type for a qualified ticker symbol using the same format
/// for the market and symbol itself.
pub struct QualifiedSymbol {
    pub market: Symbol,
    pub symbol: Symbol
}

/// Short-cut to test whether a `Symbol` is valid, and if not to return
/// `RequestError::BadSymbolError`.
///
/// This macro requires that `is_valid` and `request::RequestError` are
/// in scope where the assertion is made.
#[macro_export]
macro_rules! assert_is_valid {
    ($symbol:expr) => {
        match is_valid($symbol.to_string()) {
            true => (),
            false => return Err(RequestError::BadSymbolError($symbol))
        };
    };
}