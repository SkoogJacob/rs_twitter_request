use std::collections::HashSet;

use super::query_filters::group::Group;

pub struct TwitterClient {
    client: reqwest::Client,
    request: Option<reqwest::RequestBuilder>,
    groups: HashSet<Group>,
}
