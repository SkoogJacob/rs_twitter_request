use std::fmt::Display;

use http::Method;
use thiserror::Error;

use crate::twitter::Endpoint;
use crate::twitter::endpoints::AuthenticationType;

#[derive(Error, Debug)]
pub enum TwitterError {
    BadMethod(Endpoint, Method),
    RequestError(reqwest::Error),
    FromStrError(String),
    NoEndpointSetError,
    NoAuthError,
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
                    f, "The TwitterClient has not been provided an endpoint to make a request to"
                )
            }
            TwitterError::NoAuthError => {
                write!(
                    f, "No authentication information has been given to make this request"
                )
            }
            TwitterError::BadAuthError(t) => {
                write!(
                    f, "The {} did not grant proper access", t
                )
            }
        }
    }
}
