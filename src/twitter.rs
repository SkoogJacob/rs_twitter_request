pub mod entities;

mod endpoints;

use http::Method;
pub use endpoints::Endpoint;

mod query_params;

/**
This module will have to contain the requests, and then you can break up the filters and such
to other files to keep things neat.
*/
pub struct TwitterRequest{
    endpoint: Endpoint,
    method: Method,
}
