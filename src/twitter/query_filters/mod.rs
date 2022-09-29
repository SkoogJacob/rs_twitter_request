use self::{group::GroupList, ids::IDFilter};

pub mod filter;
pub mod group;
mod ids;

pub struct QueryFilters {
    groups: GroupList,
    ids: IDFilter
}

impl QueryFilters {
    pub fn new() -> QueryFiltersBuilder {
        QueryFiltersBuilder {
            groups: None,
            ids: None
        }
    }
}

pub struct QueryFiltersBuilder {
    groups: Option<GroupList>,
    ids: Option<IDFilter>
}

