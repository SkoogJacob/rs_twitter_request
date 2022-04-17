use std::fmt::{Display, Formatter};

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
    PublicMetrics,
}

impl Display for UserField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            UserField::Id => "id",
            UserField::CreatedAt => "created_at",
            UserField::Name => "name",
            UserField::Username => "username",
            UserField::Protected => "protected",
            UserField::Verified => "verified",
            UserField::Withheld => "withheld",
            UserField::ProfileImageUrl => "profile_image_url",
            UserField::Location => "location",
            UserField::Url => "url",
            UserField::Description => "description",
            UserField::Entities => "twitter_objects",
            UserField::PinnedTweetId => "pinned_tweet_id",
            UserField::PublicMetrics => "public_metrics",
        };

        write!(f, "{}", s)
    }
}
