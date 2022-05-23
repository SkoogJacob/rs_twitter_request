use super::filter::Filter;

/// A group is a group of filters in the twitter search query.
/// Each filter is connected to another by an AND/OR connection.
#[derive(Eq, PartialEq, Debug, Hash)]
pub struct Group {
    filters: FilterList,
}
impl Group {}

#[derive(Eq, PartialEq, Debug, Hash)]
struct FilterList {
    list: Vec<ListItem>
}
impl FilterList {
    /// Creates a new `FilterList` instance
    pub fn new(filter: Filter) -> FilterList {
        FilterList {
            list: {
                vec![ListItem {
                    filter,
                    and_or: AndOr::And
                }]
            }
        }
    }
    /// Pushes a new filter to the list with an AND connector
    pub fn push_and(&mut self, filter: Filter) {
        self.list.push(ListItem {
            filter,
            and_or: AndOr::And
        })
    }
    /// Pushes a new filter to the list with an OR connector
    pub fn push_or(&mut self, filter: Filter) {
        self.list.push(ListItem {
            filter,
            and_or: AndOr::Or
        })
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
