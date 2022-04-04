use super::filter::Filter;
use std::collections::HashSet;

pub struct Group<'a> {
    filters: HashSet<Filter<'a>>,
}

impl<'a> Group<'a> {}
