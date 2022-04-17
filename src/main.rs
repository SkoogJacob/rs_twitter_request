use dotenv::dotenv;
use reqwest;
use std::env;
use twitter_request::twitter;

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found!");
    let client = reqwest::Client::new();
    let endpoint = twitter::Endpoint::SearchTweetsRecent;
    let req = client
        .request(
            http::Method::GET,
            endpoint.to_string(),
        )
        .bearer_auth(env::var("BEARER_TOKEN").unwrap().as_str())
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .query(&[("query", "(from:Archival_Blob you)(letters)")]); // Twitter puts all its shit into query=(key:val)(key:val)
    println!("req_builder: {:?}", req);
    let request = req.try_clone().unwrap().build().unwrap();
    let url = request.url().as_str();
    println!("{}", url);
    println!("{:?}", request);
    let res = req.send().await;
    match res {
        Ok(r) => {
            println!("{:?}", r.text().await.unwrap())
        }
        Err(e) => {
            println!("{:?}", e)
        }
    }
}
