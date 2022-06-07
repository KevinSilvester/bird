use anyhow::Result;
use std::path::{Path};
use std::env;

   // const root_dir: String = match env::var("BIRD_TREE") {
   //    Ok(val) => val,
   //    Err(_) => return Err(BirdError::MissingEnvVar(env::VarError::NotPresent))
   // };

#[derive(Debug, Clone)]
pub struct BirdConfig {
   pub root_dir: String,
   pub egg_file: String,
   pub nest_file: String,
}

impl BirdConfig {
   pub fn new() -> Result<Self> {
      
      let root_dir: String = env::var("BIRD_TREE")?;

      Ok(Self {
         root_dir: root_dir.clone(),
         egg_file: root_dir.clone() + "/.bird-eggs.json",
         nest_file: root_dir.clone() + "/.bird-nest.json",
      })
   }
}
