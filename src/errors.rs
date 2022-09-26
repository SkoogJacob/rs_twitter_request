use std::fmt::Display;

use http::Method;
use thiserror::Error;

use crate::twitter::endpoints::AuthenticationType;
use crate::twitter::Endpoint;

#[derive(Error, Debug)]
pub enum TwitterError {
    BadMethod(Endpoint, Method),
    RequestError(reqwest::Error),
    FromStrError(String),
    NoEndpointSetError,
    NoAuthError,
    WrongAuthError(Endpoint, AuthenticationType, Method),
    BadAuthError(AuthenticationType),
}

impl Display for TwitterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TwitterError::BadMethod(endpoint, method) => {
                write!(
                    f,
                    "Method {} is not supported by {}, use one the following methods instead: {:?}",
                    method,
                    endpoint,
                    endpoint.get_methods()
                )
            }
            TwitterError::RequestError(e) => write!(f, "{}", e),
            TwitterError::FromStrError(e) => {
                write!(f, "{} could not be turned into an endpoint", e)
            }
            TwitterError::NoEndpointSetError => {
                write!(
                    f,
                    "The TwitterClient has not been provided an endpoint to make a request to"
                )
            }
            TwitterError::NoAuthError => {
                write!(
                    f,
                    "No authentication information has been given to make this request"
                )
            }
            TwitterError::BadAuthError(t) => {
                write!(f, "The {} did not grant proper access", t)
            }
            TwitterError::WrongAuthError(endpoint, auth_type, method) => {
                write!(
                    f,
                    "{} is not an accepted auth type for {}, use {} instead",
                    auth_type,
                    endpoint,
                    endpoint.get_auth_type(method).unwrap()
                )
            }
        }
    }
}

#[derive(Error, Debug)]
pub enum TwitterBuilderError {
    #[error("no authentication provided, cannot build TwitterRequest")]
    NoAuthError,
    #[error("no endpoint provided, cannot build TwitterRequest")]
    NoEndpointError,
    #[error("no method provided, cannot build a TwitterRequest")]
    NoMethodError,
}
