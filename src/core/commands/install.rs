use super::command::Command;
use crate::core::{BirdConfig, Egg};
use crate::utils::colour;
use anyhow::Result;

#[derive(clap::Parser, Debug)]
pub struct Install {
   #[clap(multiple_values = true)]
   /// List of programs to be installed
   pub programs: Vec<String>,

   #[clap(long, conflicts_with = "programs")]
   /// Install all the programs with installation commands provided
   pub all: bool,

   #[clap(long, multiple_values = true, conflicts_with = "programs")]
   /// Skip installing these programs if '--all' is passed in
   pub skip: Option<Vec<String>>
}

impl Command for Install {
   fn call(self, config: &BirdConfig) -> Result<()> {
      
      Ok(())
   }
}
