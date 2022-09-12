use dotenv::dotenv;
use reqwest;
use std::env;
use std::process::exit;
use http::Method;
use reqwest::Response;
use twitter_request::{Endpoint, Filter, twitter};
use twitter_request::errors::TwitterError;

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found!");
    let client = reqwest::Client::new();
    let endpoint = twitter::Endpoint::SearchTweetsRecent;
    let req = client
        .request(http::Method::GET, endpoint.to_string())
        .bearer_auth(env::var("BEARER_TOKEN").unwrap().as_str())
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .query(&[("query", "(from:Archival_Blob you)(letters)")]); // Twitter puts all its shit into query=(key:val)(key:val)
    println!("req_builder: {:?}", req);
    let request = req.try_clone().unwrap().build().unwrap();
    let url = request.url().as_str();
    println!("url: {}", url);
    println!("req: {:?}", request);
    let res = req.send().await;
    match res {
        Ok(r) => {
            println!("{:?}", r)
        }
        Err(e) => {
            println!("{:?}", e)
        }
    };
    twitter_request::hello();

    let mut client = twitter::request::TwitterClient::new();
    match client.set_endpoint(Endpoint::SearchTweetsRecent, Method::GET) {
        Ok(_) => {}
        Err(e) => { println!("{:?}", e); exit(1) }
    };
    client.add_and_filter(Filter::From("Archival_Blob".to_string(), true.into()));
    client.add_bearer_token(&env::var("BEARER_TOKEN").unwrap());
    let req = client.send_request().await;
    match req {
        Ok(r) => { println!("{:?}", r) }
        Err(e) => { println!("{}", e) }
    }
}
