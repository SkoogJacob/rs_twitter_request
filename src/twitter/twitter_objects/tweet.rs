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
