/*!
Provides types and functions for market symbols.

In general the `Symbol` type is loosely defined, it is stored as a `String`
but there is little standardization across markets and geographies
concerning length or character set. To this end the function `is_valid`
simply takes the commonly known limits and returns true/false, although
this should probably be _maybe true_/false.

The macro [`assert_is_valid`](../macro.assert_is_valid.html) can be used by
providers as it will do nothing if a symbol is valid but return a
`request::RequestResult` if it is not.
*/

use std::collections::HashSet;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// Type for a market ticker symbol. Consumers of symbols should use
/// `is_valid` to ensure the value they are provided is at least
/// syntactically correct.
pub type Symbol = String;

/// Where a set of symbols are used together.
pub type Symbols = HashSet<Symbol>;

/// Maximum length assuming a root length of 6 and a 2 character suffix.
pub const MAX_SYMBOL_LENGTH: usize = 8;

/// is the symbol valid; currently only string lengths are checked.
#[inline(always)]
pub fn is_valid(symbol: Symbol) -> bool {
    symbol.len() >= 1 && symbol.len() <= MAX_SYMBOL_LENGTH
}

/// Type for a qualified ticker symbol using the same format
/// for the market and symbol itself.
pub struct QualifiedSymbol {
    /// the symbol for the market that qualifies `symbol`
    pub market: Symbol,
    /// the target security symbol
    pub symbol: Symbol
}

// ------------------------------------------------------------------------------------------------
// Macros
// ------------------------------------------------------------------------------------------------

/// Short-cut to test whether a `Symbol` is valid, and if not to return
/// a `RequestResult` containing the error `RequestError::BadSymbolError`.
///
/// This macro *requires* that `is_valid` and `request::RequestError` are
/// in scope where the assertion is made.
/// ## Example
///
/// The following example shoes the use of the macro, and specifically the
/// necessary imports.
///
/// ```rust
/// use fin_model::request::{RequestError, RequestResult};
/// use fin_model::symbol::is_valid;
///
/// fn latest_price_only(&self, for_symbol: Symbol) -> RequestResult<f32> {
///     assert_is_valid!(for_symbol);
///     Ok(0.0)
/// }
/// ```
#[macro_export]
macro_rules! assert_is_valid {
    ($symbol:expr) => {
        match is_valid($symbol.to_string()) {
            true => (),
            false => return Err(RequestError::BadSymbolError($symbol))
        };
    };
}