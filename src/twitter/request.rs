use http::Method;
use reqwest::Client;

use crate::errors::{TwitterBuilderError, TwitterError};
use crate::twitter::endpoints::AuthenticationData;

use super::{
    query_filters::{filter::Filter, group::GroupList},
    Endpoint,
};

pub struct TwitterRequest {
    groups: GroupList,
    endpoint: Endpoint,
    method: Method,
    authentication: AuthenticationData,
}

impl TwitterRequest {
    pub fn builder() -> TwitterRequestBuilder {
        TwitterRequestBuilder {
            groups: GroupList::new_empty(),
            endpoint: None,
            method: None,
            authentication: None,
        }
    }

    /// Sends the request, consuming the `TwitterRequest` object and return the request Result
    pub async fn send_request(self, client: &Client) -> Result<reqwest::Response, TwitterError> {
        self.endpoint
            .send_request(client, self.method, self.groups, self.authentication)
            .await
    }
}

pub struct TwitterRequestBuilder {
    groups: GroupList,
    endpoint: Option<Endpoint>,
    method: Option<Method>,
    authentication: Option<AuthenticationData>,
}

impl TwitterRequestBuilder {
    pub fn set_endpoint(mut self, endpoint: Endpoint) -> Self {
        self.endpoint = Some(endpoint);
        self
    }

    /// Adds a filter that needs to be true in addition to the previous filters in the group
    pub fn add_and_filter(mut self, filter: Filter) -> Self {
        self.groups.push_filter_and(filter);
        self
    }

    /// Adds a filter that can be in true instead of the previous filters in the group
    pub fn add_or_filter(mut self, filter: Filter) -> Self {
        self.groups.push_filter_or(filter);
        self
    }

    /// Adds a new filter group that should evaluate to true in addition to previous groups.
    /// The group will initially be without any filters
    pub fn add_empty_and_group(mut self) -> Self {
        self.groups.new_empty_group_and();
        self
    }

    /// Adds a new filter group that can evaluate to true alternatively to previous groups.
    /// The group will initially be without any filters.
    pub fn add_empty_or_group(mut self) -> Self {
        self.groups.new_empty_group_or();
        self
    }

    /// Adds a new filter group that should evaluate to true in addition to previous groups.
    /// The group will be initiated with the passed filter
    pub fn add_and_group(mut self, init_filter: Filter) -> Self {
        self.groups.new_group_and(init_filter);
        self
    }

    /// Adds a set of filters where all filters should be true
    pub fn add_and_filters<I>(mut self, filters: I) -> Self
    where
        I: IntoIterator<Item = Filter>,
    {
        for filter in filters {
            self.groups.push_filter_and(filter)
        }
        self
    }

    /// Adds a set of filters where any one filter should be true
    pub fn add_or_filters<I>(mut self, filters: I) -> Self
    where
        I: IntoIterator<Item = Filter>,
    {
        for filter in filters {
            self.groups.push_filter_or(filter)
        }
        self
    }

    /// Adds a new filter group that can evaluate to true alternatively to previous groups.
    /// The group will be initiated with the passed filter
    pub fn add_or_group(mut self, init_filter: Filter) -> Self {
        self.groups.new_group_or(init_filter);
        self
    }

    /// Adds a bearer token to the request builder
    pub fn add_bearer_token(mut self, token: &str) -> Self {
        self.authentication = Some(AuthenticationData::new_bearer_auth(token));
        self
    }

    /// Sets the method of the request
    pub fn set_method(mut self, method: Method) -> Self {
        self.method = Some(method);
        self
    }

    pub fn build(self) -> Result<TwitterRequest, TwitterBuilderError> {
        if self.endpoint.is_none() {
            Err(TwitterBuilderError::NoEndpointError)
        } else if self.method.is_none() {
            Err(TwitterBuilderError::NoMethodError)
        } else if self.authentication.is_none() {
            Err(TwitterBuilderError::NoAuthError)
        } else {
            Ok(TwitterRequest {
                groups: self.groups,
                endpoint: self.endpoint.unwrap(),
                method: self.method.unwrap(),
                authentication: self.authentication.unwrap(),
            })
        }
    }
}
