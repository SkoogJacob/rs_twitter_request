use super::endpoints::Endpoint;
use http::Method;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fmt::{Display, format, Formatter, Write};
use std::ops::Deref;

pub struct TwitterClient<'a> {
    client: reqwest::Client,
    request: Option<reqwest::RequestBuilder>,
    groups: HashSet<Group<'a>>,
}

#[derive(PartialEq)]
struct Group<'a> {
    filters: Vec<Filter<'a>>, // TODO fix groups to work somewhat properly next
    locked: bool,
}

impl<'a> Group<'a> {
    /// Lock the group, preventing any alterations to self.filters until unlocked.
    pub fn lock(&mut self) {
        self.locked = true;
    }
    /// Unlock the group, allowing alterations to self.filters until locked.
    pub fn unlock(&mut self) {
        self.locked = false;
    }
}

#[derive(PartialEq)]
enum Filter<'a> {
    Keyword(&'a str, Exact, Is),
    From(&'a str, Is),
    RetweetsOf(&'a str, Is),
    Context(&'a str, Is),
    Entity(&'a str, Is),
    Url(&'a str, Is),
    To(&'a str, Is),
    IsRetweet(Is),
    IsReply(Is),
    IsQuote(Is),
    IsVerified(Is),
    IsNullcast(Is),
    HasHashtags(Is),
    HasCashtags(Is),
    HasLinks(Is),
    HasMentions(Is),
    HasMedia(Is),
    HasImages(Is),
    HasVideos(Is),
    HasGeo(Is),
    LocPlaceCountry(String, Is), // TODO Make an enum of the countries
    LocPlace(String, Is),
    LocBoundingBox(BoundingBox, Is),
    LocPointRadius(PointRadius, Is),
    LangLang(String, Is), // TODO Make a language enum
    ConvConversationId(String, Is),
}

impl<'a> Filter<'a> {
    pub fn is_main(&self) -> bool {
        return match self {
            Filter::Keyword(_, _, _) |
            Filter::From(_, _) |
            Filter::RetweetsOf(_, _) |
            Filter::Context(_, _) |
            Filter::Entity(_, _) |
            Filter::Url(_, _) |
            Filter::To(_, _) => {true}
            _ => {false}
        }
    }
}

impl<'a> Display for Filter<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (mut filter_string, is) = match self {
            Filter::Keyword(val, exact, is) => {
                let string = if exact.val() {
                    // If `exact`, wrap with `"`
                    format!("\"{}\"", val)
                } else {
                    val.to_string()
                };
                (string, is.val())
            }
            Filter::From(val, is) => (format!("from:{}", val), is.val()),
            Filter::RetweetsOf(val, is) => (format!("retweets_of:{}", val), is.val()),
            Filter::Context(val, is) => (format!("{}{}", "context:", val), is.val()),
            Filter::Entity(val, is) => (format!("{}{}{}", "entity:\"", val, "\""), is.val()),
            Filter::Url(val, is) => (format!("{}{}{}", "url:\"", val, "\""), is.val()),
            Filter::To(val, is) => (format!("{}{}", "to:", val), is.val()),
            Filter::IsRetweet(is) => (String::from("is:retweet"), is.val()),
            Filter::IsReply(is) => (String::from("is:reply"), is.val()),
            Filter::IsQuote(is) => (String::from("is:quote"), is.val()),
            Filter::IsVerified(is) => (String::from("is:verified"), is.val()),
            Filter::IsNullcast(is) => (String::from("is:nullcast"), is.val()),
            Filter::HasHashtags(is) => (String::from("has:hashtags"), is.val()),
            Filter::HasCashtags(is) => (String::from("has:cashtags"), is.val()),
            Filter::HasLinks(is) => (String::from("has:links"), is.val()),
            Filter::HasMentions(is) => (String::from("has:mentions"), is.val()),
            Filter::HasMedia(is) => (String::from("has:media"), is.val()),
            Filter::HasImages(is) => (String::from("has:images"), is.val()),
            Filter::HasVideos(is) => (String::from("has:videos"), is.val()),
            Filter::HasGeo(is) => (String::from("has:geo"), is.val()),
            Filter::LocPlaceCountry(val, is) => {(format!("place_country:{}", val), is.val())}
            Filter::LocPlace(val, is) => {(format!("place:\"{}\"", val), is.val())}
            Filter::LocBoundingBox(bound_box, is) => {(format!("bounding_box:{}", bound_box), is.val())}
            Filter::LocPointRadius(point_radius, is) => {(format!("point_radius:{}", point_radius), is.val())}
            Filter::LangLang(lang, is) => {(format!("lang:{}", lang), is.val())}
            Filter::ConvConversationId(id, is) => {(format!("conversation_id:{}", id), is.val())}
        };
        if !is {
            filter_string = format!("{}{}", "-", filter_string);
        }
        write!(f, "{}", filter_string)
    }
}

trait Val: Sized {
    fn val(&self) -> bool;
}
#[derive(PartialEq, Eq)]
pub struct Exact(bool);
impl Val for Exact {
    fn val(&self) -> bool {
        self.0
    }
}
#[derive(PartialEq, Eq)]
pub struct Is(bool);
impl Val for Is {
    fn val(&self) -> bool {
        self.0
    }
}
#[derive(PartialEq, Debug)]
pub struct BoundingBox(pub f32, pub f32, pub f32, pub f32);
impl Display for BoundingBox {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:.6} {:.6} {:.6} {:.6}]", self.0, self.1, self.2, self.3)
    }
}
#[derive(PartialEq, Debug)]
pub struct PointRadius {
    pub longitude: f32,
    pub latitude: f32,
    pub radius: u32
}
impl Display for PointRadius{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:.6} {:.6} {}]", self.longitude, self.latitude, self.radius)
    }
}
