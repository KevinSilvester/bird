// use anyhow::Result;
use std::env;
use crate::utils::errors::BirdError;

#[derive(Debug, Clone)]
pub struct BirdConfig {
   pub root_dir: String,
   pub egg_file: String,
   pub nest_file: String,
}

impl BirdConfig {
   pub fn new() -> Result<Self, BirdError> {
      let root_dir: String = env::var("BIRD_TREE")?;

      // let root_dir = "/home/wsl1/__root__".to_owned();

      Ok(Self {
         root_dir: root_dir.clone(),
         egg_file: root_dir.clone() + "/.bird-eggs.json",
         nest_file: root_dir.clone() + "/.bird-nest.json",
      })
   }
}
