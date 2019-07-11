/*!
TBD
*/

use chrono::NaiveDate;

// ------------------------------------------------------------------------------------------------
// Public Traits
// ------------------------------------------------------------------------------------------------

pub trait Registry<C: std::fmt::Display, T> {

    fn new() -> Self where Self: Sized;

    fn name(&self) -> String;

    fn acronym(&self) -> String;

    fn source(&self) -> String;

    fn governing_body(&self) -> String;

    fn last_updated(&self) -> Option<NaiveDate>;

    fn next_publication(&self) -> Option<NaiveDate>;

    fn get(&self, code: C) -> Option<&T> where Self: Sized;

    fn get_children(&self, parent_code: C) -> Option<Vec<&T>> where Self: Sized;
}
