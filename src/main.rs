/*
The GPLv3 License (GPLv3)

Copyright (c) 2022 Jacob Skoog

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

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
    // let req = client
    //     .request(http::Method::GET, endpoint.to_string())
    //     .bearer_auth(env::var("BEARER_TOKEN").unwrap().as_str())
    //     .header(reqwest::header::CONTENT_TYPE, "application/json")
    //     .query(&[("query", "(from:Archival_Blob)")]); // Twitter puts all its shit into query=(key:val)(key:val)
    // println!("req_builder: {:?}", req);
    // let request = req.try_clone().unwrap().build().unwrap();
    // let url = request.url().as_str();
    // println!("url: {}", url);
    // println!("req: {:?}", request);
    // let res = req.send().await;
    // match res {
    //     Ok(r) => {
    //         println!("{:?}", r)
    //     }
    //     Err(e) => {
    //         println!("{:?}", e)
    //     }
    // };
    //
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
            println!("ok: {:?}", &r);
            println!("resonse text: {:?}", r.text().await.unwrap())
        }
        Err(e) => {
            println!("error: {}", e)
        }
    }
    let request = TwitterRequest::builder()
        .set_endpoint(Endpoint::LookupTweets)
        .add_id(1585200978755665925)
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
            println!("ok: {:?}", &r);
            println!("response text: {:?}", r.text().await.unwrap())
        }
        Err(e) => eprintln!("error: {}", e),
    }
}
