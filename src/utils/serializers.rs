// ref: https://github.com/serde-rs/serde/issues/936#issuecomment-302281792
pub mod eggs {
   use crate::core::EggItem;
   use serde::de::{Deserialize, Deserializer};
   use serde::ser::Serializer;
   use std::collections::BTreeMap;

   pub fn serialize<S>(map: &BTreeMap<String, EggItem>, serializer: S) -> Result<S::Ok, S::Error>
   where
      S: Serializer,
   {
      serializer.collect_seq(map.values())
   }

   pub fn deserialize<'de, D>(deserializer: D) -> Result<BTreeMap<String, EggItem>, D::Error>
   where
      D: Deserializer<'de>,
   {
      let mut map = BTreeMap::new();
      for egg in Vec::<EggItem>::deserialize(deserializer)? {
         map.insert(egg.name.clone(), egg);
      }
      Ok(map)
   }
}

pub mod nest {
   use crate::core::NestItem;
   use serde::de::{Deserialize, Deserializer};
   use serde::ser::Serializer;
   use std::collections::BTreeMap;

   pub fn serialize<S>(map: &BTreeMap<String, NestItem>, serializer: S) -> Result<S::Ok, S::Error>
   where
      S: Serializer,
   {
      serializer.collect_seq(map.values())
   }

   pub fn deserialize<'de, D>(deserializer: D) -> Result<BTreeMap<String, NestItem>, D::Error>
   where
      D: Deserializer<'de>,
   {
      let mut map = BTreeMap::new();
      for item in Vec::<NestItem>::deserialize(deserializer)? {
         map.insert(item.name.clone(), item);
      }
      Ok(map)
   }
}

pub mod date_format {
   use chrono::{DateTime, TimeZone, Utc};
   use serde::{self, Deserialize, Deserializer, Serializer};

   const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

   pub fn serialize<S>(date: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
   where
      S: Serializer,
   {
      match date {
         Some(s) => {
            let d = format!("{}", s.format(FORMAT));
            serializer.serialize_str(&d)
         }
         None => serializer.serialize_none(),
      }
   }

   pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
   where
      D: Deserializer<'de>,
   {
      let o: Option<String> = Option::deserialize(deserializer)?;

      match o {
         Some(s) => {
            let d = match Utc.datetime_from_str(&s, FORMAT) {
               Ok(d) => d,
               Err(_) => return Ok(None),
            };
            Ok(Some(d))
         }
         None => Ok(None),
      }
   }
}
