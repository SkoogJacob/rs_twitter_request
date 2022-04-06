use super::{endpoints::Endpoint, query_params::filter::*};
use http::Method;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fmt::{format, Display, Formatter, Write};
use std::ops::Deref;

pub struct TwitterClient<'a> {
    client: reqwest::Client,
    request: Option<reqwest::RequestBuilder>,
    groups: HashSet<Group<'a>>,
}

#[derive(PartialEq)]
struct Group<'a> {
    filters: Vec<Filter<'a>>, // TODO fix groups to work somewhat properly next
    locked: bool,
}

impl<'a> Group<'a> {
    /// Lock the group, preventing any alterations to self.filters until unlocked.
    pub fn lock(&mut self) {
        self.locked = true;
    }
    /// Unlock the group, allowing alterations to self.filters until locked.
    pub fn unlock(&mut self) {
        self.locked = false;
    }
}
