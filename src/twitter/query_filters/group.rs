use std::fmt::{Display, Formatter};
use std::fmt::Write as _;

use super::filter::Filter;

type FilterItem = QueryItem<Filter>;
type GroupItem = QueryItem<Group>;

#[derive(Eq, PartialEq, Debug, Hash)]
pub struct GroupList {
    groups: Vec<GroupItem>
}
impl GroupList {
    pub fn new(filter: Filter) -> GroupList {
        let mut groups:Vec<GroupItem> = Vec::with_capacity(8);
        groups.push(GroupItem{
            and_or: AndOr::And,
            item: Group::new(filter)
        });
        GroupList { groups }
    }
    pub fn push_filter_and(&mut self, filter: Filter) {
        match self.groups.last_mut() {
            None => self.new_group_and(filter),
            Some(g) => g.item.push_and(filter)
        }
    }
    pub fn push_filter_or(&mut self, filter: Filter) {
        match self.groups.last_mut() {
            None => self.new_group_and(filter),
            Some(g) => g.item.push_or(filter)
        }
    }
    pub fn new_group_and(&mut self, filter: Filter) {
        self.groups.push(GroupItem{
            and_or: AndOr::And,
            item: Group::new(filter)
        })
    }
    pub fn new_group_or(&mut self, filter: Filter) {
        self.groups.push(GroupItem {
            and_or: AndOr::Or,
            item: Group::new(filter)
        })
    }
    /// Removes the last Group in the GroupList
    pub fn remove_group(&mut self) {
        self.groups.pop();
    }
}

impl Display for GroupList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut res_string = String::with_capacity(2048);
        self.groups.iter()
            .for_each(
                |item| { write!(res_string, "{}{}", item.and_or, item.item
                ).expect("Could not write the filter into the String buffer"); });
        write!(f, "{}", res_string.trim())
    }
}

/// A group is a group of filters in the twitter search query.
/// Each filter is connected to another by an AND/OR connection.
#[derive(Eq, PartialEq, Debug, Hash)]
struct Group {
    list: Vec<QueryItem<Filter>>
}
impl Group {
    /// Creates a new `FilterList` instance
    pub fn new(filter: Filter) -> Group {
        let mut list: Vec<FilterItem> = Vec::with_capacity(8);
        list.push(QueryItem{
            and_or: AndOr::And,
            item: filter
        });
        Group {
            list
        }
    }

    /// Pushes a new filter to the list with an AND connector
    pub fn push_and(&mut self, filter: Filter) {
        if !self.contains_filter(&filter) {
            self.list.push(QueryItem{
                item: filter,
                and_or: AndOr::And
            })
        }
    }
    /// Pushes a new filter to the list with an OR connector
    pub fn push_or(&mut self, filter: Filter) {
        if !self.contains_filter(&filter){
            self.list.push(QueryItem {
                item: filter,
                and_or: AndOr::Or
            })
        }
    }

    /// Checks if any of the QueryItems contains the passed filter
    /// Running this check for each push is expensive, but requests
    /// are unlikely to have even 100 filters so this inefficiency
    /// should be acceptable.
    fn contains_filter(&self, filter: &Filter) -> bool {
        self.list.iter()
            .map(|item| &item.item)
            .any(|f| f.eq(filter))
    }
}

impl Display for Group {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut res_string = String::with_capacity(512);
        self.list.iter()
            .for_each(
                |item| { write!(res_string, "{}{}", item.and_or, item.item
                ).expect("Could not write the filter into the String buffer"); });
        write!(f, "({})", res_string.trim())
    }
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

#[derive(PartialEq, Eq, Debug, Hash)]
struct QueryItem<T> {
    and_or: AndOr,
    item: T
}
