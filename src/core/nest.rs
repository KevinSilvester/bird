use super::BirdConfig;
use crate::utils::{
   errors::BirdError,
   files,
   serializers::{date_format, nest},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct NestItem {
   pub name: String,
   #[serde(with = "date_format")]
   pub timestamp: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Nest {
   #[serde(with = "nest")]
   pub nest: BTreeMap<String, NestItem>,
}

impl Nest {
   pub fn new(config: &BirdConfig) -> Result<Self, BirdError> {
      Ok(Self {
         nest: Self::file_to_btreemap(&config)?,
      })
   }

   pub fn exists(config: &BirdConfig) -> bool {
      Path::new(&config.nest_file).exists()
   }

   pub fn init(config: &BirdConfig) -> Result<(), BirdError> {
      files::create_file(&config.nest_file)?;
      files::write_file(&config.nest_file, r#"{"nest": []}"#)?;
      Ok(())
   }

   pub fn btreemap_to_file(&mut self, config: &BirdConfig) -> Result<(), BirdError> {
      let json = serde_json::to_string_pretty(&self)?;
      files::create_file(&config.nest_file)?;
      files::write_file(&config.nest_file, &json)?;
      Ok(())
   }

   pub fn append(&mut self, p_name: &String, config: &BirdConfig) -> Result<(), BirdError> {
      let p = NestItem {
         name: p_name.to_owned(),
         timestamp: Some(Utc::now()),
      };

      self.nest.insert(p_name.to_owned(), p);
      self.btreemap_to_file(&config)?;
      Ok(())
   }

   pub fn remove(&mut self, p_name: &String, config: &BirdConfig) -> Result<(), BirdError> {
      self.nest.remove(&p_name.to_owned());
      self.btreemap_to_file(&config)?;
      Ok(())
   }

   pub fn file_to_btreemap(config: &BirdConfig) -> Result<BTreeMap<String, NestItem>, BirdError> {
      let json = files::read_file(&config.nest_file)?;

      let parsed_json: Nest = match serde_json::from_str(&json) {
         Ok(s) => s,
         Err(err) => return Err(BirdError::JsonError((".bird-nest.json".to_owned(), err.to_string()))),
      };

      Ok(parsed_json.nest)
   }
}
