use std::collections::HashMap;

use super::command::Command;
use crate::core::{BirdConfig , Egg};
use crate::utils::errors::BirdError;
use crate::utils::{colour, files, iter_hashmap};
// use anyhow::Result;

#[derive(clap::Parser, Debug)]
pub struct Install {
   /// List of programs to be installed
   #[clap(multiple_values = true)]
   pub programs: Vec<String>,

   /// Install all the programs with installation commands provided
   #[clap(long, conflicts_with = "programs")]
   pub all: bool,

   /// Skip installing these programs if '--all' is passed in
   #[clap(long, multiple_values = true, conflicts_with = "programs")]
   pub skip: Vec<String>,
}

impl Command for Install {
   fn call(self, config: &BirdConfig) -> Result<(), BirdError> {
      // get all programs from bird-eggs file
      // let mut eggs = files::read_eggs_file(config)?;

      // eggs.sort_by_key(|a| a.dependencies.is_some());

      // // if no programs in eggs abort exit the program
      // if eggs.len() == 0 {
      //    println!(
      //       "\n{}",
      //       colour::warn("No programs found in '.birds-eggs.json'")
      //    );
      //    return Ok(());
      // }

      // read .bird-eggs to hashmap
      // All:
      // iterate through all eggs.
      // check for egg in nest
      // if there are dependencies
      // check for dependency eggs in nest
      // if not in nest, check if it exists in the eggs hashmap
      // if does install the egg(s) first then install the original egg
      // if the dependencies aren't found in the nest or eggs hasmap then 
      // print an error message and skip installation for that egg.

      // Programs:
      // iterate through programs vec
      // check if the the program exists in the eggs hashmap
      // if not found print error and skip else
      // check if the eggs already installed in the nest hashmap
      // if not then, check if it has dependencies,
      // if has dependencies check for their 

      let mut eggs: HashMap<String, Egg> = files::eggs_to_hashmap(config)?;

      let cb = |k: &String, v: &Egg| {
         
      };

      match self.all {
         true => {
            // iter_hashmap(&eggs, |k, v| println!("{}", k));
            for (key, value) in &eggs {
               
            }
         }
         false => {}
      }

      // match self.all {
      //    true => {
      //       for egg in eggs {
      //          if !self.skip.iter().any(|s| s == &egg.name) {
      //             // egg.install()?;
      //             println!("\n\n{:?}", &egg);
      //          }
      //       }
      //       return Ok(());
      //    }
      //    false => {
      //       
      //    }
      // }

      // // if '--all' flag was used, install all programs
      // if self.all {
      //    for egg in eggs {
      //       if !self.skip.iter().any(|s| s == &egg.name) {
      //          // egg.install()?;
      //          println!("\n\n{:?}", &egg);
      //       }
      //    }
      //    return Ok(());
      // }

      // let mut invalid_eggs = Vec::new();

      // for program in self.programs {
      //    if let Some(egg) = eggs.iter().find(|e| e.name == program) {
      //       egg.install()?;
      //    } else {
      //       invalid_eggs.push(program);
      //    }
      // }

      // if !invalid_eggs.is_empty() {
      //    return Err(BirdError::ProgramsNotFound(invalid_eggs));
      // }

      Ok(())
   }
}
