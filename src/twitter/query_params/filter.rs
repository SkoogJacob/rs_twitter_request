use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};

#[derive(PartialEq, Eq, Debug, Hash)]
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

/// This enum is used to indicate if a filter is meant to be exact or not
#[derive(PartialEq, Eq, Debug, Hash)]
pub enum Exact {
    Is,
    Not
}
/// This enum is used to indicate if a filter tests for existence or absence
#[derive(PartialEq, Eq, Debug, Hash)]
pub enum Is {
    Is,
    Not
}
/// A struct for a bounding box location search, having two coordinate pairs
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct BoundingBox {
    x1: Real,
    y1: Real,
    x2: Real,
    y2: Real
}
impl BoundingBox {
    pub fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> BoundingBox {
        let (x1, y1, x2, y2) = (Real::from(x1), Real::from(y1),
                                                          Real::from(x2), Real::from(y2));
        BoundingBox {
            x1, y1, x2, y2
        }
    }

}
impl Display for BoundingBox {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{} {} {} {}]",
            self.x1, self.y1, self.x2, self.y2
        )
    }
}
/// A struct for a point+radius location search, having a coordinate pair for the
/// circle center, and a radius expressed in km.
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct PointRadius {
    longitude: Real,
    latitude: Real,
    radius_km: u32,
}
impl PointRadius {
    pub fn new(longitude: f32, latitude: f32, radius_km: u32) -> PointRadius {
        PointRadius {
            longitude: Real::new(longitude),
            latitude: Real::new(latitude),
            radius_km,
        }
    }
}
impl Display for PointRadius {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{} {} {}km]",
            self.longitude, self.latitude, self.radius_km
        )
    }
}

#[derive(Debug)]
struct Real {
    r: f32,
}
impl Real {
    /// Constructs a `Real` which is a struct that wraps a float in a `Result`. It panics on NAN,
    /// guaranteeing that the float is always comparable.
    pub fn new(real: f32) -> Real {
        if real.is_nan() {
            panic!("Real can only be constructed from non-NAN floats")
        }
        Real { r: real }
    }
    /// Get an immutable reference to the internal Real struct
    pub fn r(&self) -> f32 {
        self.r
    }
}
impl PartialEq for Real {
    fn eq(&self, other: &Self) -> bool {
        // This comparison will always work as r is guaranteed to never be NAN
        self.r == other.r
    }
}
/// Can implement Eq for Real as the Ok value is never NAN
impl Eq for Real {
}
impl Hash for Real {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Using native endian-ness as this is never intended to be sent over network.
        let f_bytes = self.r.to_le_bytes();
        f_bytes.iter().for_each(|r| state.write_u8(*r));
    }
}
impl Display for Real {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.6}", self.r)
    }
}