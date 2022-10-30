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
pub enum AuthenticationType {
    BearerToken,
    OauthSignature,
}
impl Display for AuthenticationType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthenticationType::BearerToken => {
                write!(f, "BearerToken")
            }
            AuthenticationType::OauthSignature => {
                write!(f, "OAuth1.0")
            }
        }
    }
}

pub struct AuthenticationData {
    auth_token: String,
    auth_type: AuthenticationType,
}
impl AuthenticationData {
    pub fn new_bearer_auth(token: &str) -> AuthenticationData {
        AuthenticationData {
            auth_token: String::from(token),
            auth_type: AuthenticationType::BearerToken,
        }
    }
    pub fn get_oauth_data() -> ! {
        // TODO find out how to construct oauth data
        todo!()
    }
    /// Gets the authentication type to check that the correct type is being used
    pub fn get_type(&self) -> AuthenticationType {
        match self.auth_type {
            AuthenticationType::BearerToken => AuthenticationType::BearerToken,
            AuthenticationType::OauthSignature => AuthenticationType::OauthSignature,
        }
    }
    /// Get the auth token to put into the request header
    pub fn get_auth_token(&self) -> &str {
        &self.auth_token
    }
}
