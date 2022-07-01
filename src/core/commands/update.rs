use super::command::Command;
use crate::outln;
use crate::core::{BirdConfig, EggItem, Eggs, Nest};
use crate::utils::errors::BirdError;

#[derive(clap::Parser, Debug)]
#[clap(arg_required_else_help = true)]
pub struct Update {
   /// List of programs to be updated
   #[clap(multiple_values = true, exclusive = true)]
   pub programs: Vec<String>,

   /// Update all the programs with installation commands provided
   #[clap(long, short, conflicts_with = "programs")]
   pub all: bool,

   /// Skip updating these programs if '--all' is passed in
   #[clap(
      long,
      short,
      multiple_values = true,
      conflicts_with = "programs",
      value_name = "PROGRAMS"
   )]
   pub skip: Vec<String>,
}

impl Command for Update {
   fn call(self, config: &BirdConfig) -> Result<(), BirdError> {
      let eggs = Eggs::new(&config)?;

      if eggs.eggs.is_empty() {
         outln!(warn, "No programs found in '.birds-eggs.json'");
         return Ok(());
      }

      if !Nest::exists(&config) {
         Nest::init(&config)?;
      }

      let mut nest = Nest::new(&config)?;

      match self.all {
         true => Self::update_all(self, &eggs, &mut nest, &config),
         false => Self::update_selected(self, &eggs, &mut nest, &config)?,
      }

      Ok(())
   }
}

impl Update {
   fn update_all(self, eggs: &Eggs, nest: &Nest, config: &BirdConfig) {}

   fn update_selected(self, eggs: &Eggs, nest: &Nest, config: &BirdConfig) -> Result<(), BirdError> {
      Ok(())
   }
}
