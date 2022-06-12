use super::command::Command;
use crate::core::{BirdConfig, EggItem, Eggs, Nest};
use crate::utils::{colour, errors::BirdError};

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
      let eggs = Eggs::new(&config)?;

      if eggs.eggs.is_empty() {
         println!("\n{}", colour::warn("No programs found in '.birds-eggs.json'"));
         return Ok(());
      }

      if !Nest::exists(&config) {
         Nest::init(&config)?;
      }

      let mut nest = Nest::new(&config)?;

      match self.all {
         true => Self::install_all(self, &eggs, &mut nest, &config),
         false => Self::install_selected(self, &eggs, &mut nest, &config)?,
      }

      Ok(())
   }
}

impl Install {
   fn install_all(self, eggs: &Eggs, nest: &mut Nest, config: &BirdConfig) {
      for (key, value) in &eggs.eggs {
         if !self.skip.iter().any(|i| &i == &key) {
            match nest.nest.contains_key(&key.to_owned()) {
               true => println!("\n{}", colour::warn(&format!("{} is already installed", key))),
               false => match Self::install_program(&value.clone(), &eggs, nest, &config) {
                  Ok(_) => (),
                  Err(e) => println!("{}", &e),
               },
            }
         }
      }
   }

   fn install_selected(self, eggs: &Eggs, nest: &mut Nest, config: &BirdConfig) -> Result<(), BirdError> {
      let mut invalid_eggs = Vec::new();

      for (key, value) in &eggs.eggs {
         match self.programs.iter().any(|i| &i == &key) {
            true => match nest.nest.contains_key(&key.to_owned()) {
               true => println!("\n{}", colour::warn(&format!("{} is already installed", key))),
               false => match Self::install_program(&value.clone(), &eggs, nest, &config) {
                  Ok(_) => (),
                  Err(e) => println!("{}", &e),
               },
            },
            false => invalid_eggs.push(key.clone()),
         }
      }

      if !invalid_eggs.is_empty() {
         return Err(BirdError::ProgramsNotFound(invalid_eggs));
      }
      Ok(())
   }

   fn install_program(egg: &EggItem, eggs: &Eggs, nest: &mut Nest, config: &BirdConfig) -> Result<(), String> {
      match &egg.dependencies {
         Some(deps) => {
            for d in deps {
               if !nest.nest.contains_key(&d.to_owned()) {
                  if eggs.eggs.contains_key(&d.to_owned()) {
                     match eggs.eggs.get(&d.to_owned()) {
                        Some(ee) => {
                           println!(
                              "{} {}: {}",
                              format!("{}", colour::info("Installing dependency for")),
                              colour::warn(&egg.name),
                              &format!(": {}", colour::info(&ee.name))
                           );

                           match Self::install_program(&ee, &eggs, nest, &config) {
                              Ok(_) => (),
                              Err(err) => return Err(err),
                           }

                           match ee.install() {
                              Ok(_) => {
                                 nest.append(&ee.name, &config).unwrap();
                              }
                              Err(_err) => {
                                 return Err(format!(
                                    "\n{}\n{}",
                                    colour::error(&format!("Installation of dependency {} failed", &d)),
                                    colour::warn(&format!("Skipping '{}'", &egg.name))
                                 ))
                              }
                           }
                        }
                        _ => (),
                     }
                  } else {
                     return Err(format!(
                        "\nLine: 171: {}\n{}",
                        colour::error(&format!("Dependency {} was not found in .bird-eggs.json", &d)),
                        colour::warn(&format!("Skipping '{}'", &egg.name))
                     ));
                  }
               }
            }
            match egg.install() {
               Ok(_) => {
                  nest.append(&egg.name, &config).unwrap();
               }
               Err(_err) => {
                  return Err(format!(
                     "\n{}\n{}",
                     colour::error(&format!("Installation of {} failed", &egg.name)),
                     colour::warn(&format!("Skipping '{}'", &egg.name))
                  ))
               }
            }
         }
         None => match egg.install() {
            Ok(_) => {
               nest.append(&egg.name, &config).unwrap();
            }
            Err(_err) => {
               return Err(format!(
                  "\n{}\n{}",
                  colour::error(&format!("Installation of {} failed", &egg.name)),
                  colour::warn(&format!("Skipping '{}'", &egg.name))
               ))
            }
         },
      }
      Ok(())
   }
}
