/*
 * This is free and unencumbered software released into the public domain.
 *
 * Anyone is free to copy, modify, publish, use, compile, sell, or
 * distribute this software, either in source code form or as a compiled
 * binary, for any purpose, commercial or non-commercial, and by any
 * means.
 *
 * In jurisdictions that recognize copyright laws, the author or authors
 * of this software dedicate any and all copyright interest in the
 * software to the public domain. We make this dedication for the benefit
 * of the public at large and to the detriment of our heirs and
 * successors. We intend this dedication to be an overt act of
 * relinquishment in perpetuity of all present and future rights to this
 * software under copyright law.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 * IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * For more information, please refer to <http://unlicense.org/>
 */

use std::fmt::Display;
use std::fmt::Write as _;

/// This struct represents a list of tweet ids to use in
/// a tweets GET request. Must not be empty to be used in a
/// request.
pub struct IDFilter {
    id_list: Vec<Id>,
}

impl IDFilter {
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
    /// Adds all the IDS in the passed iterable into the IDFilter
    pub fn add_ids(&mut self, ids: impl IntoIterator<Item = impl Into<Id>>) {
        let ids: Vec<Id> = ids.into_iter()
            .map(|f| f.into())
            .filter(|f| !self.id_list.contains(&f))
            .collect();
        self.id_list.extend(ids)
    }
    pub fn is_empty(&self) -> bool {
        self.id_list.is_empty()
    }
}

impl Display for IDFilter {
    /// Formats the IDFilter for use in a request. Will fail and return an
    /// error if the IDFilter is empty.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() { // An empty IDFilter is invalid
            return std::fmt::Result::Err(std::fmt::Error)
        };
        let mut s = String::from("ids=");
        self.id_list
            .iter()
            .for_each(|id| write!(s, "{},", id).expect("Could not write ID into string buffer"));
        write!(f, "{}", &s[0..(s.len() - 1)])
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct Id {
    id: u64
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
    type Error = <u64 as std::str::FromStr>::Err;

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
        assert_eq!(
            String::from("ids=123,32323,235235"),
            format!("{}", &sut));
        sut.add_ids(ids2);
        assert_eq!(
            String::from("ids=123,32323,235235"),
            format!("{}", &sut));
        sut.add_ids(ids3);
        assert_eq!(
            String::from("ids=123,32323,235235,1,2,3"),
            format!("{}", &sut));
    }
}
