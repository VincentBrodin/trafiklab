mod request;
mod response;
mod response_raw;
use std::fmt::Display;

pub use request::*;
pub use response::*;
pub use response_raw::*;

#[derive(Default)]
pub enum Language {
    #[default]
    Swedish,
    English,
    German,
}
// Comment
impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Language::Swedish => f.write_str("sv"),
            Language::English => f.write_str("en"),
            Language::German => f.write_str("de"),
        }
    }
}

#[derive(Debug)]
pub struct Coordinate {
    pub latitude: f64,
    pub longitude: f64,
}

impl From<(f64, f64)> for Coordinate {
    fn from(value: (f64, f64)) -> Self {
        Self {
            latitude: value.0,
            longitude: value.1,
        }
    }
}

pub enum Location {
    Id(String),
    Coordinate(Coordinate),
}

impl Location {
    pub(crate) fn as_query_params(&self, prefix: &str) -> Vec<(String, String)> {
        match self {
            Location::Id(id) => vec![(format!("{prefix}Id"), id.clone())],
            Location::Coordinate(coordinate) => {
                vec![
                    (
                        format!("{prefix}CoordLat"),
                        format!("{:.6}", coordinate.latitude),
                    ),
                    (
                        format!("{prefix}CoordLong"),
                        format!("{:.6}", coordinate.longitude),
                    ),
                ]
            }
        }
    }
}
