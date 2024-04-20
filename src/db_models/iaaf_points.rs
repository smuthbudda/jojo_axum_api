#![allow(dead_code)]

use std::{fmt, str::FromStr};

use serde::{de, Deserialize, Deserializer, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "PascalCase")]
pub struct Points {
    pub points: i32,
    pub gender: String,
    pub category: String,
    pub event: String,
    pub mark: f64,
}

impl Points {
    pub fn new(points: i32, gender: String, category: String, event: String, mark: f64) -> Self {
        Self {
            points,
            gender,
            category,
            event,
            mark,
        }
    }
}
impl fmt::Display for Points {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PointsSearchQueryParams {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub points: Option<i32>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub mark: Option<f64>,
}

fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: fmt::Display,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Gender {
    Male,
    Female,
}
impl fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Category {
    Indoor,
    Outdoor,
}
impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
