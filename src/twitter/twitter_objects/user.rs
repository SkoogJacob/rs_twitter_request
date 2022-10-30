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
