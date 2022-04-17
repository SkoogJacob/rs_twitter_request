use super::filter::Filter;
use std::collections::HashSet;

#[derive(PartialEq, Debug)]
pub struct Group<'a> {
    filters: HashSet<Filter<'a>>,
}

impl<'a> Group<'a> {}
