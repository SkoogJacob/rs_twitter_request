use http::Method;
use std::fmt::Formatter;
mod authentication_types;
pub use authentication_types::AuthenticationType;

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
/// ### Methods
/// - **GET** Can get information on multiple tweets by passing comma-separated tweets in the query.
/// - **POST** Allows to post a new tweet.
///
/// ## LookupTweet(tweet_id: String)
/// Returns a path that needs to be formatted to replace a placeholder with a single tweet ID.
/// Supports both GET and POST.
///
/// ### Methods
/// This endpoint supports GET and DELETE.
///
/// - **GET** fetches the tweet, allowing for more detailed information by passing additional query parameters.
/// - **DELETE** deletes the tweet.
///
/// ## LookupTweetQuoteTweets(tweet_id: String)
/// Returns a path that needs to be formatted to replace a placeholder with a single tweet ID.
/// Gets the quote tweet of the tweet id in the endpoint path.
///
/// ### Methods
/// This method only supports the GET method.
/// - **GET** gets the quote tweets
///
/// ## LookupTweetRetweetedBy(tweet_id: String)
/// Returns a path that needs to be formatted to replace a placeholder with a single tweet ID.
/// Gets the users who retweeted the specified tweet.
///
/// ### Methods
/// This method only supports the GET method.
///  - **GET** gets the users that have retweeted this tweet.
///
/// ## LookupTweetsCountRecent
/// TODO write proper description
///
/// ### Methods
/// This endpoint only supports the GET method.
///  - **GET** Gets the number of tweets that fulfill the query parameters in the last TODO days.
///
/// ## LookupTweetsCountAll
/// TODO make a proper description
///
/// ### Methods
/// This endpoint only supports the GET method.
///  - **GET** Gets all tweets made that match the query conditions.
///
/// ## SearchTweetsRecent
/// ### Methods
///
/// ## SearchTweetsAll
/// ### Methods
///
/// ## TimelineUserTweets(user_id: String)
/// ### Methods
///
/// ## TimelineUserMentions(user_id: String)
/// ### Methods
///
/// ## StreamTweets
/// ### Methods
///
/// ## StreamRules
/// ### Methods
///
/// ## UsersByUsernames
/// ### Methods
///
/// # Methods
/// Endpoints has methods for getting supported HTTP methods for endpoints as well as getting
/// the required authentication type for an endpoint+method combination.
///
/// ## `get_methods() -> Vec<http::Method>`
/// Gets a vector of the endpoint's supported HTTP methods.
///
/// ## `get_auth_type(method: http::Method) -> Option<String>`
/// Gets an option containing either a Some(String) with the name of the authentication type,
/// or None if the http method was not supported by the endpoint.
///
pub enum Endpoint {
    LookupTweets,
    LookupTweet(String),
    LookupTweetQuoteTweets(String),
    LookupTweetRetweetedBy(String),
    LookupTweetsCountRecent,
    LookupTweetsCountAll,
    SearchTweetsRecent,
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
    pub fn get_auth_type(&self, method: Method) -> Option<AuthenticationType> {
        if !self.get_methods().contains(&method) {
            // This if-check makes exhaustive checks of method unnecessary
            // and it prevents things like LookupTweets with the DELETE method to get through
            return None;
        }
        return match self {
            Endpoint::LookupTweets | Endpoint::LookupTweet(_) => match method {
                Method::GET => Some(AuthenticationType::BearerToken),
                Method::DELETE | Method::POST => Some(AuthenticationType::OauthSignature),
                _ => None,
            },
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
            | Endpoint::StreamRules => Some(AuthenticationType::BearerToken),
        };
    }
}

impl std::fmt::Display for Endpoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let url = match self {
            Endpoint::LookupTweets => {
                format!("{}/2/tweets", TWITTER_URL)
            }
            Endpoint::LookupTweet(tweet_id) => {
                format!("{}/2/tweets/{}", TWITTER_URL, tweet_id)
            }
            Endpoint::LookupTweetQuoteTweets(tweet_id) => {
                format!("{}/2/tweets/{}/quote_tweets", TWITTER_URL, tweet_id)
            }
            Endpoint::LookupTweetRetweetedBy(tweet_id) => {
                format!("{}/2/tweets/{}/retweeted_by", TWITTER_URL, tweet_id)
            }
            Endpoint::LookupTweetsCountRecent => {
                format!("{}/2/tweets/counts/recent", TWITTER_URL)
            }
            Endpoint::LookupTweetsCountAll => {
                format!("{}/2/tweets/counts/all", TWITTER_URL)
            }
            Endpoint::SearchTweetsRecent => {
                format!("{}/2/tweets/search/recent", TWITTER_URL)
            }
            Endpoint::SearchTweetsAll => {
                format!("{}/2/tweets/search/all", TWITTER_URL)
            }
            Endpoint::TimelineUserTweets(user_id) => {
                format!("{}/2/users/{}/tweets", TWITTER_URL, user_id)
            }
            Endpoint::TimelineUserMentions(user_id) => {
                format!("{}/2/users/{}/mentions", TWITTER_URL, user_id)
            }
            Endpoint::StreamTweets => {
                format!("{}/2/tweets/search/stream", TWITTER_URL)
            }
            Endpoint::StreamRules => {
                format!("{}/2/tweets/search/stream/rules", TWITTER_URL)
            }
            Endpoint::UsersByUsernames => {
                format!("{}/2/users/by", TWITTER_URL)
            }
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
        assert_eq!(
            endpoint.to_string(),
            String::from("https://api.twitter.com/2/tweets")
        );

        assert_eq!(endpoint.get_methods(), vec![Method::GET, Method::POST]);
        match endpoint.get_auth_type(Method::DELETE) {
            None => {}
            Some(_) => {
                panic!("This endpoint doesn't support DELETE and should return None")
            }
        }
        match endpoint.get_auth_type(Method::GET) {
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
        match endpoint.get_auth_type(Method::POST) {
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
        assert_eq!(endpoint.to_string(), String::from("https://api.twitter.com/2/tweets/test"));

        vec![Method::GET, Method::DELETE]
            .iter()
            .for_each(|method| assert_eq!(endpoint.get_methods().contains(method), true));
        assert_eq!(endpoint.get_methods().contains(&Method::PATCH), false);
    }
}
