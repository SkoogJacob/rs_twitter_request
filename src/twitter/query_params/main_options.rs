/*
  The GPLv3 License (GPLv3)

  Copyright (c) 2022 Jacob Skoog

  This program is free software: you can redistribute it and/or modify
  it under the terms of the GNU General Public License as published by
  the Free Software Foundation, either version 3 of the License, or
  (at your option) any later version.

  This program is distributed in the hope that it will be useful,
  but WITHOUT ANY WARRANTY; without even the implied warranty of
  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
  GNU General Public License for more details.

  You should have received a copy of the GNU General Public License
  along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

use std::fmt::{Display, Formatter};

use chrono::{DateTime, Utc};

use crate::twitter::query_filters::{group::GroupList, ids::Id};

#[derive(PartialEq, Eq, Debug)]
pub enum MainOptions {
    Query(GroupList),
    StartTime(DateTime<Utc>),
    EndTime(DateTime<Utc>),
    SinceId(Id),
    UntilId(Id),
    MaxResults(Max),
    SortOrder(Order),
    NextToken,       // WIP
    PaginationToken, // WIP
}

impl MainOptions {
    pub fn get_query_tuple<'s>(&'s self) -> (String, String) {
        match self {
            MainOptions::Query(groups) => (String::from("query"), groups.to_string()),
            MainOptions::StartTime(start) => (String::from("start_time"), start.to_string()),
            MainOptions::EndTime(end) => (String::from("end_time"), end.to_string()),
            MainOptions::SinceId(since_id) => (String::from("since_id"), since_id.to_string()),
            MainOptions::UntilId(until_id) => (String::from("until_id"), until_id.to_string()),
            MainOptions::MaxResults(max_results) => {
                (String::from("max_results"), max_results.to_string())
            }
            MainOptions::SortOrder(sort_order) => {
                (String::from("sort_order"), sort_order.to_string())
            }
            MainOptions::NextToken => (String::from("next_token"), todo!()),
            MainOptions::PaginationToken => (String::from("pagination_token"), todo!()),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum Order {
    Recency,
    Relevancy,
}
impl Display for Order {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Order::Recency => {
                write!(f, "recency")
            }
            Order::Relevancy => {
                write!(f, "relevancy")
            }
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Max {
    max: u8,
}
impl Max {
    pub fn new(max: u8) -> Max {
        assert!(
            (10..=100).contains(&max),
            "Max must be between 10 and 100 (inclusive both ends)"
        );
        Max { max }
    }
}
impl From<u8> for Max {
    fn from(max: u8) -> Self {
        Max { max }
    }
}
impl Display for Max {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.max)
    }
}
