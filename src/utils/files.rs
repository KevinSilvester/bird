use crate::core::{BirdConfig, EggItem, Eggs, Nest, NestItem};
use crate::utils::errors::BirdError;
use serde_json;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn create_file(path: &str) -> Result<(), BirdError> {
   File::create(path)?;
   Ok(())
}

pub fn write_file(path: &str, content: &str) -> Result<(), BirdError> {
   std::fs::write(path, content)?;
   Ok(())
}

pub fn read_file(path: &str) -> Result<String, BirdError> {
   let contents = match fs::read_to_string(path) {
      Ok(val) => val,
      Err(_) => return Err(BirdError::IoError(path.to_string())),
   };
   Ok(contents)
}

pub fn eggs_to_hashmap(config: &BirdConfig) -> Result<HashMap<String, EggItem>, BirdError> {
   let json = read_file(&config.eggs_file)?;

   let parsed_json: Eggs = match serde_json::from_str(&json) {
      Ok(s) => s,
      Err(err) => return Err(BirdError::JsonError((".bird-egg.json".to_owned(), err.to_string()))),
   };

   Ok(parsed_json.eggs)
}

// pub fn nest_to_hashmap(config: &BirdConfig) -> Result<HashMap<String, NestItem>, BirdError> {
//    // check if nest file already exists
//    match Path::new(&config.nest_file).exists() {
//       true => {
// let json = read_file(&config.nest_file)?;

//          let parsed_json: Nest = match serde_json::from_str(&json) {
//             Ok(s) => s,
//             Err(err) => {
//                return Err(BirdError::JsonError((
//                   ".bird-nest.json".to_owned(),
//                   err.to_string(),
//                )))
//             }
//          };

//          return Ok(parsed_json.nest);
//       }
//       false => {
//
//       }
//    }
// }
