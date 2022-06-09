use crate::core::{BirdConfig, Egg, EggsWrapper, Nest};
use crate::utils::errors::BirdError;
use serde_json;
use std::collections::HashMap;
use std::fs;

pub fn read_file(path: &str) -> Result<String, BirdError> {
   let contents = match fs::read_to_string(path) {
      Ok(val) => val,
      Err(_) => return Err(BirdError::IoError(path.to_string())),
   };
   Ok(contents)
}

pub fn eggs_to_hashmap(config: &BirdConfig) -> Result<HashMap<String, Egg>, BirdError> {
   let json = read_file(&config.egg_file)?;

   let parsed_json: EggsWrapper = match serde_json::from_str(&json) {
      Ok(s) => s,
      Err(err) => {
         return Err(BirdError::JsonError((
            ".bird-egg.json".to_owned(),
            err.to_string(),
         )))
      }
   };

   Ok(parsed_json.eggs)
}

pub fn read_eggs_file(config: &BirdConfig) -> Result<Vec<Egg>, BirdError> {
   let json = read_file(&config.egg_file)?;

   let parsed_json: Vec<Egg> = match serde_json::from_str(&json) {
      Ok(s) => s,
      Err(err) => {
         return Err(BirdError::JsonError((
            ".bird-egg.json".to_owned(),
            err.to_string(),
         )))
      }
   };

   Ok(parsed_json)
}
