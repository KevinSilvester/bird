use super::BirdConfig;
use crate::{
   colour, outln,
   utils::{
      errors::BirdError,
      files,
      serializers::{date_format, nest},
   },
};
use chrono::{DateTime, Utc};
use format_serde_error::SerdeError;
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

   pub fn exists(config: &BirdConfig) -> Result<bool, BirdError> {
      Ok(Path::new(&config.nest_file_path()?).exists())
   }

   pub fn init(config: &BirdConfig) -> Result<(), BirdError> {
      let path = config.nest_file_path()?;

      outln!(info, "Creating nest file at {}", colour!(amber, "{path}"));

      files::create_file(&path)?;
      files::write_file(&path, r#"{"nest": []}"#)?;
      Ok(())
   }

   pub fn btreemap_to_file(&mut self, config: &BirdConfig) -> Result<(), BirdError> {
      let json = serde_json::to_string_pretty(&self)?;
      let path = config.nest_file_path()?;

      files::create_file(&path)?;
      files::write_file(&path, &json)?;
      Ok(())
   }

   pub fn file_to_btreemap(config: &BirdConfig) -> Result<BTreeMap<String, NestItem>, BirdError> {
      let json = files::read_file(&config.nest_file_path()?)?;

      let parsed_json: Nest = serde_json::from_str(&json).map_err(|err| {
         BirdError::JsonError((
            ".bird-eggs.json".to_owned(),
            SerdeError::new(json.to_string(), err).to_string(),
         ))
      })?;

      Ok(parsed_json.nest)
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
}
