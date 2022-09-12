mod twitter_auth;

use std::fmt::Formatter;

use http::Method;
use reqwest::{Client, Response};

pub use twitter_auth::{AuthenticationType, AuthenticationData};

use crate::{errors::TwitterError};
use crate::twitter::query_filters::group::GroupList;

pub const TWITTER_URL: &str = "https://api.twitter.com";

/// This enum represents the twitter API endpoints. The enum
/// has methods to get the https URI of the API endpoint and the http
/// methods the endpoint accepts.
///
/// # Endpoints
///
/// ## LookupTweets
/// Allows to search for multiple tweets or posting a tweet.
///
/// ### HTTP Methods
/// - **GET** Can get information on multiple tweets by passing comma-separated tweets in the query.
/// - **POST** Allows to post a new tweet.
///
/// ## LookupTweet(tweet_id: String)
/// Returns a path that needs to be formatted to replace a placeholder with a single tweet ID.
/// Supports both GET and POST.
///
/// ### HTTP Methods
/// This endpoint supports GET and DELETE.
///
/// - **GET** fetches the tweet, allowing for more detailed information by passing additional
///   query parameters.
/// - **DELETE** deletes the tweet.
///
/// ## LookupTweetQuoteTweets(tweet_id: String)
/// Returns a path that needs to be formatted to replace a placeholder with a single tweet ID.
/// Gets the quote tweet of the tweet id in the endpoint path.
///
/// ### HTTP Methods
/// This method only supports the GET method.
/// - **GET** gets the quote tweets
///
/// ## LookupTweetRetweetedBy(tweet_id: String)
/// Returns a path that needs to be formatted to replace a placeholder with a single tweet ID.
/// Gets the users who retweeted the specified tweet.
///
/// ### HTTP Methods
/// This method only supports the GET method.
///  - **GET** gets the users that have retweeted this tweet.
///
/// ## LookupTweetsCountRecent
/// This method only supports the GET method.
///  - **GET** Gets tweets from the last 7 days that match the query filters.
///
/// ## LookupTweetsCountAll
/// This endpoint only supports the GET method.
///  - **GET** Gets all tweets from all time that match the query filters. Only available if
///    the token used has "Academic Access".
///
/// ### HTTP Methods
/// This endpoint only supports the GET method.
///  - **GET** Gets all tweets made that match the query conditions.
///
/// ## SearchTweetsRecent
/// ### HTTP Methods
///
/// ## SearchTweetsAll
/// ### HTTP Methods
///
/// ## TimelineUserTweets(user_id: String)
/// ### HTTP Methods
///
/// ## TimelineUserMentions(user_id: String)
/// ### HTTP Methods
///
/// ## StreamTweets
/// ### HTTP Methods
///
/// ## StreamRules
/// ### HTTP Methods
///
/// ## UsersByUsernames
/// ### HTTP Methods
///
/// # Methods
/// The Endpoints enum has methods for getting the supported HTTP methods as well as getting
/// what kind of authentication is required for a specific Endpoint + HTTPMethod combination.
///
/// ## `get_methods() -> Vec<http::Method>`
/// Gets a vector of the endpoint's supported HTTP methods.
///
/// ## `get_auth_type(method: http::Method) -> Option<String>`
/// Gets an option containing either a Some(String) with the name of the authentication type,
/// or None if the http method was not supported by the endpoint.
#[derive(Debug, Clone)]
pub enum Endpoint {
    /// Used to look up tweets by passing a comma-separated list of tweet ids
    /// Currently unsupported, probably need to switch to an endpoint-focused architecture
    LookupTweets,
    /// Used to look up a tweet with a specific ID
    /// The ID is given as the String arg going with this variant
    LookupTweet(String),
    /// Used to look up tweets quoting the tweet with the passed ID
    /// The ID is given as the String arg going with this variant
    LookupTweetQuoteTweets(String),
    /// Used to look up who have retweeted the tweet with the passed ID
    /// The ID is given as the String arg going with this variant
    LookupTweetRetweetedBy(String),
    /// Used to look up how many tweets in the last X days match the accompanying query parameters
    LookupTweetsCountRecent,
    /// Used to look up how many tweets match the accompanying query parameters in the entire
    /// history of twitter. Can only be used with Academic access.
    LookupTweetsCountAll,
    /// Used to look up all tweets matching the accompanying query parameters in the last X days
    SearchTweetsRecent,
    /// Used to look up all tweets that match the accompanying query parameters in the entire
    /// history of twitter. Can only be used with Academic access.
    SearchTweetsAll,
    TimelineUserTweets(String),
    TimelineUserMentions(String),
    StreamTweets,
    StreamRules,
    UsersByUsernames, // TODO expand to include more endpoints
}

impl Endpoint {
    /// Gets the supported HTTP methods for this twitter API endpoint.
    ///
    /// # Returns
    /// `Vec<http::Method>` a vector containing the supported HTTP methods.
    pub fn get_methods(&self) -> Vec<Method> {
        match self {
            Endpoint::LookupTweets | Endpoint::StreamRules => {
                vec![Method::GET, Method::POST]
            }
            Endpoint::LookupTweet(_) => {
                vec![Method::GET, Method::DELETE]
            }
            Endpoint::LookupTweetQuoteTweets(_)
            | Endpoint::LookupTweetRetweetedBy(_)
            | Endpoint::LookupTweetsCountRecent
            | Endpoint::LookupTweetsCountAll
            | Endpoint::SearchTweetsRecent
            | Endpoint::SearchTweetsAll
            | Endpoint::TimelineUserTweets(_)
            | Endpoint::TimelineUserMentions(_)
            | Endpoint::UsersByUsernames
            | Endpoint::StreamTweets => {
                vec![Method::GET]
            }
        }
    }

    /// This method returns the auth type the endpoint needs using the passed HTTP method.
    /// The method returns None if the HTTP method is not supported by the endpoint.
    ///
    /// # Parameters
    /// ## `method: http::Method`
    /// The method you wish to use with the API endpoint.
    ///
    /// # Returns
    /// `Some(AuthenticationType)` if the endpoint supports the method
    /// `None` if the endpoint does not support the method
    pub fn get_auth_type(&self, method: &Method) -> Result<AuthenticationType, ()> {
        if !self.get_methods().contains(method) {
            // This if-check makes exhaustive checks of method unnecessary
            // and it prevents things like LookupTweets with the DELETE method to get through
            return Err(());
        }
        match self {
            Endpoint::LookupTweets | Endpoint::LookupTweet(_) => {
                match *method {
                    Method::GET => { Ok(AuthenticationType::BearerToken) },
                    Method::DELETE | Method::POST => { Ok(AuthenticationType::OauthSignature) },
                    _ => unreachable!()
                }
            }
            Endpoint::LookupTweetQuoteTweets(_)
            | Endpoint::LookupTweetRetweetedBy(_)
            | Endpoint::LookupTweetsCountRecent
            | Endpoint::LookupTweetsCountAll
            | Endpoint::SearchTweetsRecent
            | Endpoint::SearchTweetsAll
            | Endpoint::TimelineUserTweets(_)
            | Endpoint::TimelineUserMentions(_)
            | Endpoint::UsersByUsernames
            | Endpoint::StreamTweets
            | Endpoint::StreamRules => Ok(AuthenticationType::BearerToken),
        }
    }

    pub async fn send_request(&self, client: &Client, method: Method, groups: GroupList, auth: AuthenticationData)
                              -> Result<Response, TwitterError> {
        if auth.get_type() != self.get_auth_type(&method).unwrap() {
            return Err(TwitterError::WrongAuthError(self.clone().to_owned(), auth.get_type(), method))
        }
        let req = client.request(method, &self.to_string());
        let req = match auth.get_type() {
            AuthenticationType::BearerToken => { req.bearer_auth(auth.get_auth_token())}
            AuthenticationType::OauthSignature => { todo!() }
        }.query(&[("query", &groups.to_string())])
            .send()
            .await;
        match req {
            Ok(r) => { Ok(r) }
            Err(e) => { Err(TwitterError::RequestError(e)) }
        }
    }
}

impl std::fmt::Display for Endpoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let url = match self {
            Endpoint::LookupTweets => format!("{}/2/tweets", TWITTER_URL),
            Endpoint::LookupTweet(tweet_id) => format!("{}/2/tweets/{}", TWITTER_URL, tweet_id),
            Endpoint::LookupTweetQuoteTweets(tweet_id) => {
                format!("{}/2/tweets/{}/quote_tweets", TWITTER_URL, tweet_id)
            }
            Endpoint::LookupTweetRetweetedBy(tweet_id) => {
                format!("{}/2/tweets/{}/retweeted_by", TWITTER_URL, tweet_id)
            }
            Endpoint::LookupTweetsCountRecent => format!("{}/2/tweets/counts/recent", TWITTER_URL),
            Endpoint::LookupTweetsCountAll => format!("{}/2/tweets/counts/all", TWITTER_URL),
            Endpoint::SearchTweetsRecent => format!("{}/2/tweets/search/recent", TWITTER_URL),
            Endpoint::SearchTweetsAll => format!("{}/2/tweets/search/all", TWITTER_URL),
            Endpoint::TimelineUserTweets(user_id) => {
                format!("{}/2/users/{}/tweets", TWITTER_URL, user_id)
            }
            Endpoint::TimelineUserMentions(user_id) => {
                format!("{}/2/users/{}/mentions", TWITTER_URL, user_id)
            }
            Endpoint::StreamTweets => format!("{}/2/tweets/search/stream", TWITTER_URL),
            Endpoint::StreamRules => format!("{}/2/tweets/search/stream/rules", TWITTER_URL),
            Endpoint::UsersByUsernames => format!("{}/2/users/by", TWITTER_URL),
        };
        write!(f, "{}", url)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lookup_tweets_test() {
        let endpoint = Endpoint::LookupTweets;
        let expected_methods = vec![Method::GET, Method::POST];
        assert_eq!(
            endpoint.to_string(),
            String::from("https://api.twitter.com/2/tweets")
        );

        check_methods(&endpoint, &expected_methods);
        match endpoint.get_auth_type(&Method::DELETE) {
            None => {}
            Some(_) => {
                panic!("This endpoint doesn't support DELETE and should return None")
            }
        }
        match endpoint.get_auth_type(&Method::GET) {
            None => {
                panic!("Should return a Some(AuthenticationType::BearerToken)");
            }
            Some(auth) => match auth {
                AuthenticationType::BearerToken => {}
                AuthenticationType::OauthSignature => {
                    panic!("Should return Some(BearerToken), not OAuthSignature");
                }
            },
        }
        match endpoint.get_auth_type(&Method::POST) {
            None => {
                panic!("Should return a Some(AuthenticationType::OAuthSignature)");
            }
            Some(auth) => match auth {
                AuthenticationType::BearerToken => {
                    panic!("Should have returned a OAuthSignature, not a BearerToken.")
                }
                AuthenticationType::OauthSignature => {}
            },
        }
    }

    #[test]
    fn lookup_tweet_test() {
        let endpoint = Endpoint::LookupTweet(String::from("test"));
        let expected_methods = vec![Method::GET, Method::DELETE];
        assert_eq!(
            endpoint.to_string(),
            String::from("https://api.twitter.com/2/tweets/test")
        );

        check_methods(&endpoint, &expected_methods);
    }

    #[test]
    fn lookup_quote_tweets_test() {
        let endpoint = Endpoint::LookupTweetQuoteTweets(String::from("test"));
        let expected_paths = vec![Method::GET];
        check_methods(&endpoint, &expected_paths);

        assert_eq!(
            endpoint.get_auth_type(&Method::GET).unwrap(),
            AuthenticationType::BearerToken
        );

        endpoint.to_string();
    }
    // TODO fix tests below
    #[test]
    fn lookup_retweeted_by_test() {
        let endpoint = Endpoint::LookupTweetRetweetedBy(String::from("test"));
    }

    #[test]
    fn lookup_tweets_count_recent_test() {
        let endpoint = Endpoint::LookupTweetsCountRecent;
    }

    #[test]
    fn lookup_tweets_count_all_test() {
        let endpoint = Endpoint::LookupTweetsCountAll;
    }

    #[test]
    fn search_tweets_recent_test() {
        let endpoint = Endpoint::SearchTweetsRecent;
    }

    #[test]
    fn search_tweets_all_test() {
        let endpoint = Endpoint::SearchTweetsAll;
    }

    #[test]
    fn timeline_user_tweets_test() {
        let endpoint = Endpoint::TimelineUserTweets(String::from("test_user"));
    }

    #[test]
    fn timeline_user_mentions_test() {
        let endpoint = Endpoint::TimelineUserMentions(String::from("test_user"));
    }

    #[test]
    fn stream_tweets_test() {
        let endpoint = Endpoint::StreamTweets;
    }

    #[test]
    fn stream_tweets_rules_test() {
        let endpoint = Endpoint::StreamRules;
    }

    #[test]
    fn users_by_usernames_test() {
        let endpoint = Endpoint::UsersByUsernames;
    }

    fn check_methods(endpoint: &Endpoint, expected_methods: &Vec<Method>) {
        let other_methods = vec![Method::GET, Method::DELETE, Method::PUT, Method::HEAD];
        let other_methods: Vec<&Method> = other_methods
            .iter()
            .filter(|element| !expected_methods.contains(element))
            .collect();

        expected_methods
            .iter()
            .for_each(|el| assert!(endpoint.get_methods().contains(el)));
        other_methods
            .iter()
            .for_each(|el| assert!(!endpoint.get_methods().contains(el)))
    }
}
