use std::fmt::Write as _;
use std::fmt::{Display, Formatter};

use super::filter::Filter;

type FilterItem = QueryItem<Filter>;
type GroupItem = QueryItem<Group>;

#[derive(Eq, PartialEq, Debug, Hash)]
pub struct GroupList {
    groups: Vec<GroupItem>,
}
impl GroupList {
    pub fn new(filter: Filter) -> GroupList {
        let mut groups: Vec<GroupItem> = Vec::with_capacity(8);
        groups.push(GroupItem {
            and_or: AndOr::And,
            item: Group::new(filter),
        });
        GroupList { groups }
    }
    pub fn new_empty() -> GroupList {
        let groups: Vec<GroupItem> = Vec::with_capacity(4);
        GroupList { groups }
    }
    pub fn push_filter_and(&mut self, filter: Filter) {
        match self.groups.last_mut() {
            None => self.new_group_and(filter),
            Some(g) => g.item.push_and(filter),
        }
    }
    pub fn push_filter_or(&mut self, filter: Filter) {
        match self.groups.last_mut() {
            None => self.new_group_and(filter),
            Some(g) => g.item.push_or(filter),
        }
    }
    pub fn new_group_and(&mut self, filter: Filter) {
        match self.groups.last_mut() {
            None => self.add_group(AndOr::And, Some(filter)),
            Some(g) => {
                if g.item.len() == 0 {
                    g.and_or = AndOr::And
                } else {
                    self.add_group(AndOr::And, Some(filter))
                }
            }
        }
    }
    pub fn new_group_or(&mut self, filter: Filter) {
        let many_groups = self.groups.len() > 1;
        match self.groups.last_mut() {
            None => self.add_group(AndOr::And, Some(filter)),
            Some(g) => {
                if g.item.len() == 0 {
                    if many_groups {
                        g.and_or = AndOr::Or
                    }
                } else {
                    self.add_group(AndOr::Or, Some(filter))
                }
            }
        }
    }
    pub fn new_empty_group_and(&mut self) {
        if self.no_groups() || !self.empty_tail() {
            self.add_group(AndOr::And, None)
        } else {
            self.groups.last_mut().unwrap().and_or = AndOr::And
        }
    }
    pub fn new_empty_group_or(&mut self) {
        if self.no_groups() || !self.empty_tail() {
            let and_or = if self.no_groups() {
                AndOr::And
            } else {
                AndOr::Or
            };
            self.add_group(and_or, None)
        } else {
            self.groups.last_mut().unwrap().and_or = AndOr::Or;
        }
    }
    /// Removes the last Group in the GroupList
    pub fn remove_group(&mut self) {
        self.groups.pop();
    }

    /// Gets the number of groups in the GroupList
    pub fn len(&self) -> usize {
        return self.groups.len();
    }

    pub fn flatten(mut self) -> GroupList {
        if self.len() > 1 {
            let rest = self.groups.split_off(1);
            for g in rest {
                self.groups[0].item.extend(g.item)
            }
        }
        self
    }

    /// Returns true if the collection has no groups, not even empty ones
    pub fn no_groups(&self) -> bool {
        self.groups.is_empty()
    }

    /// Returns true if the `GroupList` contains no Filters
    pub fn no_filters(&self) -> bool {
        match self.groups.len() {
            0 => true,
            1 => self.groups.last().unwrap().item.len() == 0,
            _ => false,
        }
    }

    /// Returns true if tail is Some and contains no Filters
    fn empty_tail(&self) -> bool {
        match self.groups.last() {
            Some(g) => g.item.len() == 0,
            None => false,
        }
    }

    fn add_group(&mut self, and_or: AndOr, filter: Option<Filter>) {
        let group = match filter {
            Some(f) => Group::new(f),
            None => Group::new_empty(),
        };
        self.groups.push(GroupItem {
            and_or,
            item: group,
        })
    }
}

impl Display for GroupList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut res_string = String::with_capacity(1024);
        self.groups.iter().for_each(|item| {
            write!(res_string, "{}{}", item.and_or, item.item)
                .expect("Could not write the filter into the String buffer");
        });
        write!(f, "{}", res_string.trim())
    }
}

impl<'group_life> IntoIterator for &'group_life GroupList {
    type Item = &'group_life Filter;
    type IntoIter = GroupListFlatten<'group_life>;

    fn into_iter(self) -> Self::IntoIter {
        GroupListFlatten {
            outer: self.groups.iter(),
            inner: None,
        }
    }
}

pub struct GroupListFlatten<'group_life> {
    outer: std::slice::Iter<'group_life, GroupItem>,
    inner: Option<std::slice::Iter<'group_life, FilterItem>>,
}
impl<'group_life> Iterator for GroupListFlatten<'group_life> {
    type Item = &'group_life Filter;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.inner {
            // Check if self.inner has an iterator to iterate over
            None => match self.outer.next() {
                None => return None,
                Some(i) => {
                    let group = &i.item;
                    self.inner = Some(group.into_iter());
                    self.next()
                }
            },
            Some(f) => {
                match f.next() {
                    None => {
                        // if the inner iterator is empty, set inner to None and go to next iteration
                        self.inner = None;
                        self.next()
                    }
                    Some(f) => Some(&f.item),
                }
            }
        }
    }
}

/// A group is a group of filters in the twitter search query.
/// Each filter is connected to another by an AND/OR connection.
#[derive(Eq, PartialEq, Debug, Hash)]
struct Group {
    list: Vec<FilterItem>,
}
impl Group {
    /// Creates a new `Group` instance
    pub fn new(filter: Filter) -> Group {
        let mut list: Vec<FilterItem> = Vec::with_capacity(8);
        list.push(QueryItem {
            and_or: AndOr::And,
            item: filter,
        });
        Group { list }
    }

    /// Creates a new empty `Group` instance
    pub fn new_empty() -> Group {
        Group {
            list: Vec::with_capacity(8),
        }
    }

    pub fn extend(&mut self, other: Group) {
        self.list.extend(other.list)
    }

    /// Pushes a new filter to the list with an AND connector
    pub fn push_and(&mut self, filter: Filter) {
        if !self.contains_filter(&filter) {
            self.list.push(QueryItem {
                item: filter,
                and_or: AndOr::And,
            })
        }
    }
    /// Pushes a new filter to the list with an OR connector
    pub fn push_or(&mut self, filter: Filter) {
        if !self.contains_filter(&filter) {
            self.list.push(QueryItem {
                item: filter,
                and_or: AndOr::Or,
            })
        }
    }

    /// Gets the number of filters in the group
    pub fn len(&self) -> usize {
        self.list.len()
    }

    /// Checks if any of the QueryItems contains the passed filter
    /// Running this check for each push is expensive, but requests
    /// are unlikely to have even 100 filters so this inefficiency
    /// should be acceptable.
    fn contains_filter(&self, filter: &Filter) -> bool {
        self.list
            .iter()
            .map(|item| &item.item)
            .any(|f| f.eq(filter))
    }
}

impl Display for Group {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut res_string = String::with_capacity(512);
        self.list.iter().for_each(|item| {
            write!(res_string, "{}{}", item.and_or, item.item)
                .expect("Could not write the filter into the String buffer");
        });
        write!(f, "({})", res_string.trim())
    }
}

impl<'group_life> IntoIterator for &'group_life Group {
    type Item = &'group_life FilterItem;
    type IntoIter = std::slice::Iter<'group_life, FilterItem>;

    fn into_iter(self) -> Self::IntoIter {
        self.list.iter()
    }
}

#[derive(PartialEq, Eq, Debug, Hash)]
enum AndOr {
    And,
    Or,
}

impl Display for AndOr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AndOr::And => {
                write!(f, " ")
            }
            AndOr::Or => {
                write!(f, " OR ")
            }
        }
    }
}

#[derive(PartialEq, Eq, Debug, Hash)]
struct QueryItem<T> {
    and_or: AndOr,
    item: T,
}

impl<T> QueryItem<T> {
    fn and_or(&self) -> &AndOr {
        &self.and_or
    }
}
