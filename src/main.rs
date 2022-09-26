use dotenv::dotenv;
use http::Method;
use reqwest;
use std::env;
use twitter_request::errors::{TwitterBuilderError, TwitterError};
use twitter_request::{twitter, Endpoint, Filter, TwitterRequest};

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found!");
    let client = reqwest::Client::new();
    let endpoint = twitter::Endpoint::SearchTweetsRecent;
    let req = client
        .request(http::Method::GET, endpoint.to_string())
        .bearer_auth(env::var("BEARER_TOKEN").unwrap().as_str())
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .query(&[("query", "(from:Archival_Blob)")]); // Twitter puts all its shit into query=(key:val)(key:val)
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

    let request = TwitterRequest::builder()
        .set_endpoint(Endpoint::SearchTweetsRecent)
        .add_and_filter(Filter::From("Archival_Blob".to_string(), true.into()))
        .add_bearer_token(&env::var("BEARER_TOKEN").unwrap())
        .set_method(Method::GET)
        .build();
    let request = match request {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{:?}", e);
            panic!("Error building request")
        }
    };
    let req = request.send_request(&client).await;
    match req {
        Ok(r) => {
            println!("{:?}", r)
        }
        Err(e) => {
            println!("{}", e)
        }
    }
}
