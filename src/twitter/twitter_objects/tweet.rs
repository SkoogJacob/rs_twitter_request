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
    ReplySettings
}

impl TweetField {
    pub fn to_string(&self) -> String {
        return match self {
            TweetField::Id => {String::from("id")}
            TweetField::CreatedAt => {String::from("created_at")}
            TweetField::Text => {String::from("text")}
            TweetField::AuthorId => {String::from("author_id")}
            TweetField::InReplyToUserId => {String::from("in_reply_to_user_id")}
            TweetField::ReferencedTweets => {String::from("referenced_tweets")}
            TweetField::Attachments => {String::from("attachments")}
            TweetField::Withheld => {String::from("withheld")}
            TweetField::Geo => {String::from("geo")}
            TweetField::Entities => {String::from("twitter_objects")}
            TweetField::PublicMetrics => {String::from("public_metrics")}
            TweetField::PossiblySensitive => {String::from("possibly_sensitive")}
            TweetField::Source => {String::from("source")}
            TweetField::Lang => {String::from("lang")}
            TweetField::ContextAnnotations => {String::from("context_annotations")}
            TweetField::ConversationId => {String::from("conversation_id")}
            TweetField::ReplySettings => {String::from("reply_settings")}
        }
    }
}