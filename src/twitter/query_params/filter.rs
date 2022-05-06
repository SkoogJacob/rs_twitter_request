use std::fmt::{Display, Formatter};

#[derive(PartialEq, Debug)]
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
            Filter::Keyword(_, _, _)
            | Filter::From(_, _)
            | Filter::RetweetsOf(_, _)
            | Filter::Context(_, _)
            | Filter::Entity(_, _)
            | Filter::Url(_, _)
            | Filter::To(_, _) => true,
            _ => false,
        };
    }
}

impl<'a> Display for Filter<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (filter_string, is) = match self {
            Filter::Keyword(val, exact, is) => {
                let string = match exact {
                    // If `exact`, wrap with `"`
                    Exact::Is => {format!("\"{}\"", val)}
                    Exact::Not => {val.to_string()}
                };
                (string, is)
            }
            Filter::From(val, is) => (format!("from:{}", val), is),
            Filter::RetweetsOf(val, is) => (format!("retweets_of:{}", val), is),
            Filter::Context(val, is) => (format!("context:{}", val), is),
            Filter::Entity(val, is) => (format!("entity:\"{}\"", val), is),
            Filter::Url(val, is) => (format!("url:\"{}\"", val), is),
            Filter::To(val, is) => (format!("to:{}", val), is),
            Filter::IsRetweet(is) => (String::from("is:retweet"), is),
            Filter::IsReply(is) => (String::from("is:reply"), is),
            Filter::IsQuote(is) => (String::from("is:quote"), is),
            Filter::IsVerified(is) => (String::from("is:verified"), is),
            Filter::IsNullcast(is) => (String::from("is:nullcast"), is),
            Filter::HasHashtags(is) => (String::from("has:hashtags"), is),
            Filter::HasCashtags(is) => (String::from("has:cashtags"), is),
            Filter::HasLinks(is) => (String::from("has:links"), is),
            Filter::HasMentions(is) => (String::from("has:mentions"), is),
            Filter::HasMedia(is) => (String::from("has:media"), is),
            Filter::HasImages(is) => (String::from("has:images"), is),
            Filter::HasVideos(is) => (String::from("has:videos"), is),
            Filter::HasGeo(is) => (String::from("has:geo"), is),
            Filter::LocPlaceCountry(val, is) => (format!("place_country:{}", val), is),
            Filter::LocPlace(val, is) => (format!("place:\"{}\"", val), is),
            Filter::LocBoundingBox(bound_box, is) => {
                (format!("bounding_box:{}", bound_box), is)
            }
            Filter::LocPointRadius(point_radius, is) => {
                (format!("point_radius:{}", point_radius), is)
            }
            Filter::LangLang(lang, is) => (format!("lang:{}", lang), is),
            Filter::ConvConversationId(id, is) => (format!("conversation_id:{}", id), is),
        };
        match is {
            Is::Is => {
                write!(f, "{}", filter_string)
            }
            Is::Not => {
                write!(f, "-{}", filter_string)
            }
        }
    }
}

trait Val: Sized {
    fn eval(&self) -> bool;
}
/// This enum is used to indicate if a filter is meant to be exact or not
#[derive(PartialEq, Eq, Debug)]
pub enum Exact {
    Is,
    Not
}
/// This enum is used to indicate if a filter tests for existence or absence
#[derive(PartialEq, Eq, Debug)]
pub enum Is {
    Is,
    Not
}
/// A struct for a bounding box location search, having two coordinate pairs
#[derive(PartialEq, Debug)]
pub struct BoundingBox(pub f32, pub f32, pub f32, pub f32);
impl Display for BoundingBox {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{:.6} {:.6} {:.6} {:.6}]",
            self.0, self.1, self.2, self.3
        )
    }
}
/// A struct for a point+radius location search, having a coordinate pair for the
/// circle center, and a radius expressed in km.
#[derive(PartialEq, Debug)]
pub struct PointRadius {
    longitude: f32,
    latitude: f32,
    radius_km: u32,
}
impl PointRadius {
    pub fn new(longitude: f32, latitude: f32, radius_km: u32) -> PointRadius {
        PointRadius {
            longitude,
            latitude,
            radius_km,
        }
    }
}
impl Display for PointRadius {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{:.6} {:.6} {}km]",
            self.longitude, self.latitude, self.radius_km
        )
    }
}
