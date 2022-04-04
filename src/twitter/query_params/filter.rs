use std::fmt::{Display, Formatter};

#[derive(PartialEq)]
pub enum Filter<'a> {
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
                let string = if exact.eval() {
                    // If `exact`, wrap with `"`
                    format!("\"{}\"", val)
                } else {
                    val.to_string()
                };
                (string, is.eval())
            }
            Filter::From(val, is) => (format!("from:{}", val), is.eval()),
            Filter::RetweetsOf(val, is) => (format!("retweets_of:{}", val), is.eval()),
            Filter::Context(val, is) => (format!("{}{}", "context:", val), is.eval()),
            Filter::Entity(val, is) => (format!("{}{}{}", "entity:\"", val, "\""), is.eval()),
            Filter::Url(val, is) => (format!("{}{}{}", "url:\"", val, "\""), is.eval()),
            Filter::To(val, is) => (format!("{}{}", "to:", val), is.eval()),
            Filter::IsRetweet(is) => (String::from("is:retweet"), is.eval()),
            Filter::IsReply(is) => (String::from("is:reply"), is.eval()),
            Filter::IsQuote(is) => (String::from("is:quote"), is.eval()),
            Filter::IsVerified(is) => (String::from("is:verified"), is.eval()),
            Filter::IsNullcast(is) => (String::from("is:nullcast"), is.eval()),
            Filter::HasHashtags(is) => (String::from("has:hashtags"), is.eval()),
            Filter::HasCashtags(is) => (String::from("has:cashtags"), is.eval()),
            Filter::HasLinks(is) => (String::from("has:links"), is.eval()),
            Filter::HasMentions(is) => (String::from("has:mentions"), is.eval()),
            Filter::HasMedia(is) => (String::from("has:media"), is.eval()),
            Filter::HasImages(is) => (String::from("has:images"), is.eval()),
            Filter::HasVideos(is) => (String::from("has:videos"), is.eval()),
            Filter::HasGeo(is) => (String::from("has:geo"), is.eval()),
            Filter::LocPlaceCountry(val, is) => {(format!("place_country:{}", val), is.eval())}
            Filter::LocPlace(val, is) => {(format!("place:\"{}\"", val), is.eval())}
            Filter::LocBoundingBox(bound_box, is) => {(format!("bounding_box:{}", bound_box), is.eval())}
            Filter::LocPointRadius(point_radius, is) => {(format!("point_radius:{}", point_radius), is.eval())}
            Filter::LangLang(lang, is) => {(format!("lang:{}", lang), is.eval())}
            Filter::ConvConversationId(id, is) => {(format!("conversation_id:{}", id), is.eval())}
        };
        if !is {
            filter_string = format!("{}{}", "-", filter_string);
        }
        write!(f, "{}", filter_string)
    }
}

trait Val: Sized {
    fn eval(&self) -> bool;
}
/// Simple named wrapper for a boolean that indicates if select search terms are to be "exact".
#[derive(PartialEq, Eq)]
pub struct Exact(bool);
impl Val for Exact {
    fn eval(&self) -> bool {
        self.0
    }
}
/// Simple named wrapper for a boolean that says whether a filter is positive (shall match) or negative (shall not match)
#[derive(PartialEq, Eq)]
pub struct Is(bool);
impl Val for Is {
    fn eval(&self) -> bool {
        self.0
    }
}
/// A struct for a bounding box location search, having two coordinate pairs
#[derive(PartialEq, Debug)]
pub struct BoundingBox(pub f32, pub f32, pub f32, pub f32);
impl Display for BoundingBox {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:.6} {:.6} {:.6} {:.6}]", self.0, self.1, self.2, self.3)
    }
}
/// A struct for a point+radius location search, having a coordinate pair for the
/// circle center, and a radius expressed in km.
#[derive(PartialEq, Debug)]
pub struct PointRadius {
    longitude: f32,
    latitude: f32,
    radius_km: u32
}
impl PointRadius {
    pub fn new(longitude: f32, latitude: f32, radius_km: u32) -> PointRadius {
        PointRadius{longitude, latitude, radius_km}
    }
}
impl Display for PointRadius{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:.6} {:.6} {}km]", self.longitude, self.latitude, self.radius_km)
    }
}
