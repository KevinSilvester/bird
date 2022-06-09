use crate::core::BirdConfig;
use crate::utils::colour;
use crate::utils::errors::BirdError;
use crate::core::Egg;
use crate::utils::files;
// use anyhow::Result;

pub trait Command: Sized {
   fn call(self, config: &BirdConfig) -> Result<(), BirdError>;

   fn read_eggs_file(config: &BirdConfig) -> Result<Vec<Egg>, BirdError> {
      let json = files::read_file(&config.egg_file)?;

      let parsed_json: Vec<Egg> = match serde_json::from_str(&json) {
         Ok(s) => s,
         Err(err) => {
            return Err(BirdError::JsonError((
               ".bird-egg.json".to_owned(),
               err.to_string(),
            )))
         }
      };

      if parsed_json.len() == 0 {
         colour::warn("No programs found in '.birds-egg.json'");
         std::process::exit(0);
      }

      Ok(parsed_json)
   }
}
