/*!
A trait to act as a registry of data concerning some classification
scheme.

In it's simplest for the registry is a `HashMap` of identifier values to
some type that describes the identifier (see [classification](../classification/index.html)
and [market](../market/index.html) for examples). However, a registry has
specific value because there is some notion that the set of codes is standardized
and so has a source, a governing body and publication dates.

It is also common for such schemes to be hierarchical, and so there is direct
support for a [`get_children`](trait.Registry.html#tymethod.get_children) method
that may return `None` if unsupported by the current scheme.
*/

use chrono::NaiveDate as Date; /* match this to prelude */

// ------------------------------------------------------------------------------------------------
// Public Traits
// ------------------------------------------------------------------------------------------------

/// A trait to support the lookup of standardized codes.
pub trait Registry<C: std::fmt::Display, T> {

    /// Create a new instance.
    fn new() -> Self where Self: Sized;

    /// Name of the classification scheme itself.
    fn name(&self) -> String;

    /// A commonly used acronym for the scheme.
    fn acronym(&self) -> String;

    /// Source, either documentation, or data, from which the scheme
    /// was compiled.
    fn source(&self) -> String;

    /// The name of the governing body of the scheme.
    fn governing_body(&self) -> String;

    /// The date, if known, that the scheme was last updated or published.
    fn last_updated(&self) -> Option<Date>;

    /// The date, if known, that the scheme is expected to be updated.
    fn next_publication(&self) -> Option<Date>;

    /// Return a description of the code, or `None` if not present.
    fn get(&self, code: C) -> Option<&T> where Self: Sized;

    /// Return a list of child descriptions for the parent code, or
    /// `None` if either no children are found, the parent code is not
    /// valid, or the scheme does not support hierarchy.
    fn get_children(&self, parent_code: C) -> Option<Vec<&T>> where Self: Sized;
}
