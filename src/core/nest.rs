use super::Egg;
use crate::utils::colour;
use crate::utils::errors::BirdError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NestItem {
   pub name: String,

   #[serde(with = "date_format")]
   pub timestamp: DateTime<Utc>,
}

mod date_format {
   use chrono::{DateTime, TimeZone, Utc};
   use serde::{self, Deserialize, Deserializer, Serializer};

   const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

   pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
   where
      S: Serializer,
   {
      let s = format!("{}", date.format(FORMAT));
      serializer.serialize_str(&s)
   }

   pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
   where
      D: Deserializer<'de>,
   {
      let s = String::deserialize(deserializer)?;
      Utc.datetime_from_str(&s, FORMAT)
         .map_err(serde::de::Error::custom)
   }
}


pub struct Nest {
   pub items: Vec<NestItem>,
}

