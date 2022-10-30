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

use crate::twitter::query_filters::group::GroupList;

#[derive(PartialEq, Eq, Debug)]
pub enum MainOptions {
    Query(GroupList),
    StartTime(DateTime<Utc>),
    EndTime(DateTime<Utc>),
    SinceId(u64),
    UntilId(u64),
    MaxResults(Max),
    SortOrder(Order),
    NextToken,       // WIP
    PaginationToken, // WIP
}

impl Display for MainOptions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let opt = match self {
            MainOptions::Query(groups) => {
                format!("query={}", groups)
            }
            MainOptions::StartTime(d) => {
                format!("start_time={}", d)
            }
            MainOptions::EndTime(d) => {
                format!("end_time={}", d)
            }
            MainOptions::SinceId(id) => {
                format!("since_id={}", id)
            }
            MainOptions::UntilId(id) => {
                format!("until_id={}", id)
            }
            MainOptions::MaxResults(max) => {
                format!("max_results={}", max)
            }
            MainOptions::SortOrder(order) => {
                format!("sort_order={}", order)
            }
            MainOptions::NextToken => "next_token".to_string(),
            MainOptions::PaginationToken => "pagination_token".to_string(),
        };
        write!(f, "{}", opt)
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
impl Display for Max {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.max)
    }
}
