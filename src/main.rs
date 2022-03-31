use std::env;
use dotenv::dotenv;
use twitter_request::twitter;


#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found!");
    let client = reqwest::Client::new();
    let endpoint = twitter::Endpoint::SearchTweetsRecent;
    let req = client.request(endpoint.get_methods().get(0).unwrap().clone(), endpoint.get_url())
        .bearer_auth(env::var("BEARER_TOKEN").unwrap().as_str())
        .header("Content-Type", "application/json")
        .query(&[("query", "from:Archival_Blob")]); // Twitter puts all its shit into query=(key:val)&(key:val)
    println!("req_builder: {:?}", req);
    let request = req.try_clone().unwrap().build().unwrap();
    let url = request.url().as_str();
    println!("{}", url);
    println!("{:?}", request);
    let res = req.send().await;
    match res {
        Ok(r) => {println!("{:?}", r.text().await.unwrap())}
        Err(e) => {println!("{:?}", e)}
    }
}