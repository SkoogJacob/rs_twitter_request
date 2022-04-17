use std::fmt::{Display, Formatter};

#[derive(PartialEq, Debug)]
pub enum TweetField {
    Id,
    CreatedAt,
    Text,
    AuthorId,
    InReplyToUserId,
    ReferencedTweets,
    Attachments,
    Withheld,
    Geo,
    Entities,
    PublicMetrics,
    PossiblySensitive,
    Source,
    Lang,
    ContextAnnotations,
    ConversationId,
    ReplySettings,
}

impl Display for TweetField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            TweetField::Id => "id",
            TweetField::CreatedAt => "created_at",
            TweetField::Text => "text",
            TweetField::AuthorId => "author_id",
            TweetField::InReplyToUserId => "in_reply_to_user_id",
            TweetField::ReferencedTweets => "referenced_tweets",
            TweetField::Attachments => "attachments",
            TweetField::Withheld => "withheld",
            TweetField::Geo => "geo",
            TweetField::Entities => "twitter_objects",
            TweetField::PublicMetrics => "public_metrics",
            TweetField::PossiblySensitive => "possibly_sensitive",
            TweetField::Source => "source",
            TweetField::Lang => "lang",
            TweetField::ContextAnnotations => "context_annotations",
            TweetField::ConversationId => "conversation_id",
            TweetField::ReplySettings => "reply_settings",
        };

        write!(f, "{}", s)
    }
}
