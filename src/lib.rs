pub mod errors;
pub mod twitter;

pub use http::Method;
pub use twitter::{client::TwitterClient, endpoints::Endpoint, query_filters::filter::*};

pub fn hello() {
    let mut client = TwitterClient::new();
    client.set_endpoint(Endpoint::SearchTweetsRecent, Method::GET).expect("whoopsie");
}
