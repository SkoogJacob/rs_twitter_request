pub struct User {
    pub id: u64,
    pub verified: bool,
    pub profile_img_url: String,
    pub url: String,
    pub created_at: String,
    pub description: String,
    pub name: String,
    pub username: String,
    pub public_metrics: PublicMetrics,
    pub protected: bool
}

impl PartialEq<Self> for User {
    fn eq(&self, other: &Self) -> bool {
        return self.id.eq(&other.id);
    }
}

#[derive(PartialEq, Eq)]
pub struct PublicMetrics {
    followers_count: u32,
    following_count: u32,
    tweet_count: u32,
    listed_count: u32
}

pub enum UserField {
    Id,
    CreatedAt,
    Name,
    Username,
    Protected,
    Verified,
    Withheld,
    ProfileImageUrl,
    Location,
    Url,
    Description,
    Entities,
    PinnedTweetId,
    PublicMetrics
}

impl UserField {
    pub fn to_string(&self) -> String {
        return match self {
            UserField::Id => {String::from("id")}
            UserField::CreatedAt => {String::from("created_at")}
            UserField::Name => {String::from("name")}
            UserField::Username => {String::from("username")}
            UserField::Protected => {String::from("protected")}
            UserField::Verified => {String::from("verified")}
            UserField::Withheld => {String::from("withheld")}
            UserField::ProfileImageUrl => {String::from("profile_image_url")}
            UserField::Location => {String::from("location")}
            UserField::Url => {String::from("url")}
            UserField::Description => {String::from("description")}
            UserField::Entities => {String::from("entities")}
            UserField::PinnedTweetId => {String::from("pinned_tweet_id")}
            UserField::PublicMetrics => {String::from("public_metrics")}
        }
    }
}