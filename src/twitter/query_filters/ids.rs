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

use std::fmt::Display;
use std::fmt::Write as _;
use std::num::ParseIntError;

/// This struct represents a list of tweet ids to use in
/// a tweets GET request. Must not be empty to be used in a
/// request.
pub struct IDFilter {
    id_list: Vec<Id>,
}

impl IDFilter {
    pub fn new(id: impl Into<Id>) -> IDFilter {
        IDFilter {
            id_list: vec![id.into()],
        }
    }
    /// Creates a new IDFilter from an Iterable
    pub fn from_iterator(ids: impl IntoIterator<Item = impl Into<Id>>) -> IDFilter {
        IDFilter {
            id_list: Vec::from_iter(ids.into_iter().map(|f| f.into())),
        }
    }
    /// Adds an ID to the IDFilter
    pub fn add_id(&mut self, id: impl Into<Id>) {
        let id = id.into();
        if !self.id_list.contains(&id) {
            self.id_list.push(id)
        }
    }
    pub fn try_add_id(
        &mut self,
        try_id: impl TryInto<Id, Error = ParseIntError>,
    ) -> Result<(), ParseIntError> {
        let try_id = try_id.try_into();
        match try_id {
            Ok(id) => {
                if !self.id_list.contains(&id) {
                    self.id_list.push(id)
                }
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
    /// Adds all the IDS in the passed iterable into the IDFilter
    pub fn add_ids(&mut self, ids: impl IntoIterator<Item = impl Into<Id>>) {
        let ids: Vec<Id> = ids
            .into_iter()
            .map(|f| f.into())
            .filter(|f| !self.id_list.contains(&f))
            .collect();
        self.id_list.extend(ids)
    }
    pub fn is_empty(&self) -> bool {
        self.id_list.is_empty()
    }
}

impl IntoIterator for IDFilter {
    type Item = Id;

    type IntoIter = <Vec<Id> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.id_list.into_iter()
    }
}

impl Display for IDFilter {
    /// Formats the IDFilter for use in a request. Will fail and return an
    /// error if the IDFilter is empty.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            // An empty IDFilter is invalid
            return std::fmt::Result::Err(std::fmt::Error);
        };
        let mut s = String::from("ids=");
        self.id_list
            .iter()
            .for_each(|id| write!(s, "{},", id).expect("Could not write ID into string buffer"));
        write!(f, "{}", &s[0..(s.len() - 1)])
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Id {
    id: u64,
}
impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}
impl From<u64> for Id {
    fn from(id: u64) -> Self {
        Self { id }
    }
}
impl TryFrom<&str> for Id {
    type Error = ParseIntError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let id = value.parse::<u64>()?;
        Ok(Id { id })
    }
}

#[cfg(test)]
mod tests {
    use super::IDFilter;

    #[test]
    fn test_from_it() {
        let ids: Vec<u64> = vec![123, 32323, 235235];
        let ids2 = ids.clone();
        let ids3: Vec<u64> = vec![1, 2, 3];
        let mut sut = IDFilter::from_iterator(ids);
        assert!(!sut.is_empty());
        assert_eq!(String::from("ids=123,32323,235235"), format!("{}", &sut));
        sut.add_ids(ids2);
        assert_eq!(String::from("ids=123,32323,235235"), format!("{}", &sut));
        sut.add_ids(ids3);
        assert_eq!(
            String::from("ids=123,32323,235235,1,2,3"),
            format!("{}", &sut)
        );
    }
}
