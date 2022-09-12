use http::Method;
use reqwest::RequestBuilder;

use crate::errors::TwitterError;
use crate::twitter::endpoints::AuthenticationType;

use super::{
    query_filters::{filter::Filter, group::GroupList},
    Endpoint,
};

pub struct TwitterClient {
    client: reqwest::Client,
    request: Option<reqwest::RequestBuilder>,
    groups: GroupList,
    endpoint: Option<Endpoint>,
    method: Option<Method>,
    authentication: TwitterAuthentication
}

impl TwitterClient {
    pub fn new() -> Self {
        TwitterClient {
            client: reqwest::Client::new(),
            request: None,
            groups: GroupList::new_empty(),
            endpoint: None,
            method: None,
            authentication: TwitterAuthentication::new()
        }
    }

    pub fn initialised_new(
        endpoint: Endpoint,
        method: Method,
        filter: Filter,
    ) -> Result<Self, TwitterError> {
        if !endpoint.get_methods().contains(&method) {
            return Err(TwitterError::BadMethod(endpoint, method));
        }
        let client = reqwest::Client::new();
        let request = client.request(method.clone(), endpoint.to_string());
        Ok(TwitterClient {
            client,
            request: Some(request),
            groups: GroupList::new(filter),
            endpoint: Some(endpoint),
            method: Some(method),
            authentication: TwitterAuthentication::new()
        })
    }

    pub fn set_endpoint(&mut self, endpoint: Endpoint, method: Method) -> Result<(), TwitterError> {
        if !endpoint.get_methods().contains(&method) {
            return Err(TwitterError::BadMethod(endpoint, method));
        }
        self.request = Some(self.client.request(method.clone(), endpoint.to_string()));
        self.method = Some(method);
        self.endpoint = Some(endpoint);
        Ok(())
    }

    /// Sets the request to be to lookup a tweet with the passed ID
    pub fn set_lookup_tweet_endpoint(&mut self, tweet_id: &str) {
        self.request = Some(self.client.request(
            Method::GET,
            Endpoint::LookupTweet(String::from(tweet_id)).to_string(),
        ));
        self.endpoint = Some(Endpoint::LookupTweet(String::from(tweet_id)));
        self.method = Some(Method::GET);
    }

    /// Sets the request endpoint to look up tweets matching the
    pub fn set_lookup_tweets_endpoint(&mut self) {
        self.request = Some(
            self.client
                .request(Method::GET, Endpoint::LookupTweets.to_string()),
        );
        self.endpoint = Some(Endpoint::LookupTweets);
        self.method = Some(Method::GET)
    }

    /// Adds a filter that needs to be true in addition to the previous filters in the group
    pub fn add_and_filter(&mut self, filter: Filter) {
        self.groups.push_filter_and(filter);
    }

    /// Adds a filter that can be in true instead of the previous filters in the group
    pub fn add_or_filter(&mut self, filter: Filter) {
        self.groups.push_filter_or(filter);
    }

    /// Adds a new filter group that should evaluate to true in addition to previous groups.
    /// The group will initially be without any filters
    pub fn add_empty_and_group(&mut self) {
        self.groups.new_empty_group_and()
    }

    /// Adds a new filter group that can evaluate to true alternatively to previous groups.
    /// The group will initially be without any filters.
    pub fn add_empty_or_group(&mut self) {
        self.groups.new_empty_group_or()
    }

    /// Adds a new filter group that should evaluate to true in addition to previous groups.
    /// The group will be initiated with the passed filter
    pub fn add_and_group(&mut self, init_filter: Filter) {
        self.groups.new_group_and(init_filter)
    }

    /// Adds a new filter group that can evaluate to true alternatively to previous groups.
    /// The group will be initiated with the passed filter
    pub fn add_or_group(&mut self, init_filter: Filter) {
        self.groups.new_group_or(init_filter)
    }

    pub fn add_bearer_token(&mut self, token: &str) {
        self.authentication.add_bearer_token(token);
        let req = self.request.take();
        match req {
            None => {}
            Some(r) => {
                self.request = Some(r.bearer_auth(token))
            }
        }
    }

    pub async fn send_request(&mut self) -> Result<reqwest::Response, TwitterError> {
        if self.request.is_none() {
            return Err(TwitterError::NoEndpointSetError);
        }
        if self.authentication.api_token.is_none() && self.authentication.bearer_token.is_none() {
            return Err(TwitterError::NoAuthError)
        }
        let auth_type = &self.endpoint.as_ref().unwrap()
            .get_auth_type(self.method.clone().unwrap()).unwrap();
        match auth_type {
            AuthenticationType::BearerToken => { if self.authentication.bearer_token.is_none() {
                return Err(TwitterError::NoAuthError)
            }}
            AuthenticationType::OauthSignature => {unimplemented!()}
        }
        let req = self.request.take().unwrap();
        let req = req.query(&[("query", &self.groups.to_string())]);
        println!("{:?}", &req);
        match req.send().await {
            Ok(r) => {
                if r.status() == 401 {
                    return Err(TwitterError::BadAuthError(AuthenticationType::BearerToken))
                }
                Ok(r)
            },
            Err(e) => { Err(TwitterError::RequestError(e)) },
        }
    }
}

impl TwitterClient {
    /// Adds a set of filters where all filters should be true
    pub fn add_and_filters<I>(&mut self, filters: I)
    where
        I: IntoIterator<Item = Filter>,
    {
        for filter in filters {
            self.groups.push_filter_and(filter)
        }
    }

    /// Adds a set of filters where any one filter should be true
    pub fn add_or_filters<I>(&mut self, filters: I)
    where
        I: IntoIterator<Item = Filter>,
    {
        for filter in filters {
            self.groups.push_filter_or(filter)
        }
    }
}

pub struct TwitterAuthentication {
    bearer_token: Option<String>,
    api_token: Option<String>
}

impl TwitterAuthentication {
    pub fn new() -> Self {
        TwitterAuthentication {
            bearer_token: None,
            api_token: None,
        }
    }
    pub fn add_bearer_token(&mut self, token: &str) {
        self.bearer_token = Some(String::from(token))
    }
    pub fn add_api_token(&mut self, token: &str) {
        self.api_token = Some(String::from(token))
    }
}
