use std::fmt::{Display, Formatter};

#[derive(PartialEq, Debug)]
pub enum AuthenticationType {
    BearerToken,
    OauthSignature,
}
impl Display for AuthenticationType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthenticationType::BearerToken => { write!(f, "BearerToken")}
            AuthenticationType::OauthSignature => { write!(f, "OAuth1.0")}
        }
    }
}
