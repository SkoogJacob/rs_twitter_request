/*
 * The GPLv3 License (GPLv3)
 *
 * Copyright (c) 2022 Jacob Skoog
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use crate::{
    errors::{TwitterBuilderError, TwitterError},
    Filter,
};

use self::{group::GroupList, ids::IDFilter};

pub mod filter;
pub mod group;
pub mod ids;

/// A wrapper struct to contain the filter types used to make requests to the twitter API
/// Guaranteed to contain at least some data
pub struct QueryFilters {
    groups: Option<GroupList>,
    ids: Option<IDFilter>,
}

impl QueryFilters {
    /// Returns an empty builder
    pub fn new() -> QueryFiltersBuilder {
        QueryFiltersBuilder {
            groups: None,
            ids: None,
        }
    }
    /// Gets the tweet ids in the filter, if any exist
    pub fn ids(self) -> Option<IDFilter> {
        self.ids
    }
    /// Gets the filter groups of the query filter, if any exist
    pub fn groups(self) -> Option<GroupList> {
        self.groups
    }
}

/// A builder object for QueryFilters to separate the build stage where filter
/// information is added from the built phase where information is extracted
pub struct QueryFiltersBuilder {
    groups: Option<GroupList>,
    ids: Option<IDFilter>,
}

impl QueryFiltersBuilder {
    /// Adds all the groups and filters in the passed group list to the query filter
    pub fn add_group_list(mut self, group: group::GroupList) -> Self {
        match &mut self.groups {
            Some(group_list) => group_list.extend(group),
            None => self.groups = Some(group),
        };
        self
    }
    /// Adds an AND filter to the query filter group list
    pub fn add_and_filter(mut self, filter: Filter) -> Self {
        match &mut self.groups {
            Some(group_list) => group_list.push_filter_and(filter),
            None => {
                let group_list = Some(GroupList::new(filter));
                self.groups = group_list;
            }
        };
        self
    }
    /// Adds a OR filter to the query filter group list
    pub fn add_or_filter(mut self, filter: Filter) -> Self {
        match &mut self.groups {
            Some(group_list) => group_list.push_filter_or(filter),
            None => self.groups = Some(GroupList::new(filter)),
        };
        self
    }
    pub fn add_empty_and_filter_group(mut self) -> Self {
        match &mut self.groups {
            Some(group_list) => group_list.new_empty_group_and(),
            None => self.groups = Some(GroupList::new_empty()),
        };
        self
    }
    pub fn add_and_filter_group(mut self, filter: Filter) -> Self {
        match &mut self.groups {
            Some(group_list) => group_list.new_group_and(filter),
            None => self.groups = Some(GroupList::new(filter)),
        };
        self
    }
    pub fn add_empty_or_filter_group(mut self) -> Self {
        match &mut self.groups {
            Some(group_list) => group_list.new_empty_group_or(),
            None => self.groups = Some(GroupList::new_empty()),
        };
        self
    }
    pub fn add_or_filter_group(mut self, filter: Filter) -> Self {
        match &mut self.groups {
            Some(group_list) => group_list.new_group_or(filter),
            None => self.groups = Some(GroupList::new(filter)),
        };
        self
    }
    /// Adds all the ids in the IDFilter to the query filter
    pub fn add_id_filter(mut self, ids: IDFilter) -> Self {
        match &mut self.ids {
            Some(id_list) => {
                id_list.add_ids(ids);
            }
            None => self.ids = Some(ids),
        };
        self
    }
    /// Adds a single tweet ID to the filter
    pub fn add_id(mut self, id: u64) -> Self {
        match &mut self.ids {
            Some(id_list) => id_list.add_id(id),
            None => self.ids = Some(IDFilter::new(id)),
        };
        self
    }
    /// Consumes the QueryFiltersBuilder and returns an empty Err if
    /// the builder had not been provided with either ids or groups.
    /// Returns a QueryFilters struct if either was provided.
    pub fn build(self) -> Result<QueryFilters, TwitterBuilderError> {
        if self.ids.is_none() && self.groups.is_none() {
            return Err(TwitterBuilderError::BadQueryError(String::from(
                "No query filters provided!",
            )));
        }
        return Ok(QueryFilters {
            groups: self.groups,
            ids: self.ids,
        });
    }
}
