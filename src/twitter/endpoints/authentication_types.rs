#[derive(PartialEq, Debug)]
pub enum AuthenticationType {
    BearerToken,
    OauthSignature,
}
