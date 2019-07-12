/*!
Structures for a registry of classification codes.

The `Code` structure supports modeling of simple hierarchies of classification
codes, or classification schemes. For example, US and UK SIC codes, the North
American NAICS scheme, etc.

A classification scheme is an implementation of the
[`Registry`](../registry/trait.Registry.html) trait for `Code<T>` supporting the
lookup of code identifiers.
*/

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// Data about a single code, note that we parameterize the type
/// of the code identifier.
#[derive(Debug, Clone)]
pub struct Code<T> {
    /// the code identifier
    pub code: T,
    /// the parent code, allowing for simple hierarchy
    pub parent_code: Option<T>,
    /// a description of this code
    pub description: String
}

