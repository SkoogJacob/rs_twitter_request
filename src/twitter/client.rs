use std::collections::BTreeSet;
use super::endpoints::Endpoint;
use http::Method;
/**
This module will have to contain the requests, and then you can break up the filters and such
to other files to keep things neat.
 */
pub struct TwitterClient{
    client: reqwest::Client,
    groups: BTreeSet<Group>
}

struct Group {
}

struct Filter<'a> {
    f_type: FilterType,
    key: Option<&'a str>,
    value: &'a str
}

impl Filter {
    fn to_string(&self) -> String {
        let mut s = String::new();
        match self.key {
            None => { s.insert_str(0, self.value); }
            Some(k) => {
                s.insert_str(0, k);
                s.insert_str(s.len(), ":");
                s.insert_str(s.len(), self.value);
            }
        }
        s
    }
}

enum FilterType{
    Main,
    Secondary
}