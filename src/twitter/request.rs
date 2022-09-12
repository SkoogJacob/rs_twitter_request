use http::Method;
use reqwest::{Client, ClientBuilder, RequestBuilder};

use crate::errors::{TwitterBuilderError, TwitterError};
use crate::twitter::endpoints::{AuthenticationType, AuthenticationData};

use super::{
    query_filters::{filter::Filter, group::GroupList},
    Endpoint,
};

pub struct TwitterRequest {
    request: RequestBuilder,
    groups: GroupList,
    endpoint: Endpoint,
    method: Method,
    authentication: AuthenticationData,
}

impl TwitterClient {
    pub fn builder() -> TwitterRequestBuilder {
        TwitterRequestBuilder {
            request: None,
            groups: GroupList::new_empty(),
            endpoint: None,
            method: None,
            authentication: None
        }
    }


    pub async fn send_request(&mut self, client: &Client) -> Result<reqwest::Response, TwitterError> {
        self.endpoint.send_request(client, &self.method, &self.groups, &self.authentication)
    }
}

pub struct TwitterRequestBuilder {
    request: Option<RequestBuilder>,
    groups: GroupList,
    endpoint: Option<Endpoint>,
    method: Option<Method>,
    authentication: Option<AuthenticationData>
}

impl TwitterRequestBuilder {
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

    /// Adds a new filter group that can evaluate to true alternatively to previous groups.
    /// The group will be initiated with the passed filter
    pub fn add_or_group(&mut self, init_filter: Filter) {
        self.groups.new_group_or(init_filter)
    }

    pub fn build(self) -> Result<TwitterClient, TwitterBuilderError> {
        if self.request.is_none() {
            Err(TwitterBuilderError::NoRequestError)
        } else if self.endpoint.is_none() {
            Err(TwitterBuilderError::NoEndpointError)
        } else if self.method.is_none() {
            Err(TwitterBuilderError::NoMethodError)
        } else if self.authentication.is_none() {
            Err(TwitterBuilderError::NoAuthError)
        } else {
            Ok(TwitterClient {
                request: self.request.unwrap(),
                groups: self.groups,
                endpoint: self.endpoint.unwrap(),
                method: self.method.unwrap(),
                authentication: self.authentication.unwrap()
            })
        }
    }
}