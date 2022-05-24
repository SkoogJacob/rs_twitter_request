use super::query_filters::group::GroupList;

pub struct TwitterClient {
    client: reqwest::Client,
    request: Option<reqwest::RequestBuilder>,
    groups: GroupList,
}
