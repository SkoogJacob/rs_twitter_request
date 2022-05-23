use std::fmt::{Display, Formatter};
use std::fmt::Write as _;

use super::filter::Filter;

/// A group is a group of filters in the twitter search query.
/// Each filter is connected to another by an AND/OR connection.
#[derive(Eq, PartialEq, Debug, Hash)]
pub struct Group {
    list: Vec<ListItem>
}
impl Group {
    /// Creates a new `FilterList` instance
    pub fn new(filter: Filter) -> Group {
        let mut list: Vec<ListItem> = Vec::with_capacity(8);
        list.push(ListItem{
            filter, and_or: AndOr::And
        });
        Group {
            list
        }
    }

    /// Pushes a new filter to the list with an AND connector
    pub fn push_and(&mut self, filter: Filter) {
        if !self.contains_filter(&filter) {
            self.list.push(ListItem{
                filter,
                and_or: AndOr::And
            })
        }
    }
    /// Pushes a new filter to the list with an OR connector
    pub fn push_or(&mut self, filter: Filter) {
        if !self.contains_filter(&filter){
            self.list.push(ListItem {
                filter,
                and_or: AndOr::Or
            })
        }
    }

    /// Checks if any of the ListItems contains the passed filter
    /// Running this check for each push is expensive, but requests
    /// are unlikely to have even 100 filters so this inefficiency
    /// should be acceptable.
    fn contains_filter(&self, filter: &Filter) -> bool {
        self.list.iter()
            .map(|item| &item.filter)
            .any(|f| f.eq(filter))
    }
}

impl Display for Group {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut res_string = String::with_capacity(512);
        self.list.iter()
            .for_each(
                |item| { write!(res_string, "{}{}", item.and_or, item.filter
                ).expect("Could not write the filter into the String buffer"); });
        write!(f, "({})", res_string.trim())
    }
}

#[derive(Eq, PartialEq, Debug, Hash)]
struct ListItem {
    filter: Filter,
    and_or: AndOr
}

#[derive(PartialEq, Eq, Debug, Hash)]
enum AndOr {
    And,
    Or
}

impl Display for AndOr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AndOr::And => {write!(f, " ")}
            AndOr::Or => {write!(f, " OR ")}
        }
    }
}
