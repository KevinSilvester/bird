use super::command::Command;
use crate::core::{BirdConfig, EggItem, Eggs, Nest};
use crate::utils::errors::BirdError;
use crate::{colour, outln};
use dialoguer::{theme::ColorfulTheme, Confirm};

#[derive(clap::Parser, Debug)]
#[clap(arg_required_else_help = true)]
pub struct Install {
   /// List of programs to be installed
   #[clap(multiple_values = true)]
   pub programs: Vec<String>,

   /// Install all the programs with installation commands provided
   #[clap(long, short, conflicts_with = "programs")]
   pub all: bool,

   /// Skip installing these programs if '--all' is passed in
   #[clap(
      long,
      short,
      multiple_values = true,
      conflicts_with = "programs",
      value_name = "PROGRAMS"
   )]
   pub skip: Vec<String>,
}

impl Command for Install {
   fn call(self, config: &BirdConfig) -> Result<(), BirdError> {
      if !Eggs::exists(&config)? {
         Eggs::init(&config)?;
      }

      let eggs = Eggs::new(&config)?;

      if eggs.eggs.is_empty() {
         outln!(warn, "No programs found in '.birds-eggs.json'");
         return Ok(());
      }

      if !Nest::exists(&config)? {
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
      match Confirm::with_theme(&ColorfulTheme::default())
         .show_default(true)
         .wait_for_newline(true)
         .with_prompt("This may take a while! Do you want to continue?")
         .interact()
         .unwrap()
      {
         true => {
            for (key, value) in &eggs.eggs {
               if !self.skip.iter().any(|i| &i == &key) {
                  match nest.nest.contains_key(&key.to_owned()) {
                     true => outln!(info, "{} is already installed!", colour!(amber, "{}", key)),
                     false => match Self::install_program(&value.clone(), &eggs, nest, &config) {
                        Ok(_) => (),
                        Err((str_1, str_2)) => {
                           println!("{}: {}", colour!(red, "ALERT"), str_1);
                           outln!(info, "{}", str_2);
                           println!("--------------------------------------------------");
                        }
                     },
                  }
               }
            }
         }
         false => outln!(warn, "Installation cancelled!"),
      }
   }

   fn install_selected(self, eggs: &Eggs, nest: &mut Nest, config: &BirdConfig) -> Result<(), BirdError> {
      let mut invalid_eggs = Vec::new();

      for program in self.programs {
         match nest.nest.contains_key(&program) {
            true => outln!(info, "{} is already installed!", colour!(amber, "{}", program)),
            false => match eggs.eggs.get(&program) {
               Some(e) => match Self::install_program(&e, &eggs, nest, &config) {
                  Ok(_) => (),
                  Err((str_1, str_2)) => {
                     println!("--------------------------------------------------");
                     println!("{}: {}", colour!(red, "ALERT"), str_1);
                     outln!(info, "{}", str_2);
                     println!("--------------------------------------------------");
                  }
               },
               None => invalid_eggs.push(program),
            },
         }
      }

      if !invalid_eggs.is_empty() {
         return Err(BirdError::ProgramsNotFound(invalid_eggs));
      }
      Ok(())
   }

   fn install_program(
      egg: &EggItem,
      eggs: &Eggs,
      nest: &mut Nest,
      config: &BirdConfig,
   ) -> Result<(), (String, String)> {
      match &egg.dependencies {
         Some(deps) => {
            for d in deps {
               if nest.nest.contains_key(&d.to_owned()) {
                  continue;
               }

               if d.eq(&egg.name) {
                  return Err((
                     format!("Program {} cannot be its own dependency", colour!(blue, "{}", &d)),
                     format!("Skipping {} installation", colour!(amber, "{}", &egg.name)),
                  ));
               }

               match eggs.eggs.get(&d.to_owned()) {
                  Some(dep_egg) => {
                     println!("--------------------------------------------------");
                     println!(
                        "{}: Installing dependency for {}: {}",
                        colour!(amber, "ALERT"),
                        colour!(amber, "{}", &egg.name),
                        colour!(blue, "{}", &dep_egg.name)
                     );
                     println!("--------------------------------------------------");

                     match Self::install_program(&dep_egg, &eggs, nest, &config) {
                        Ok(_) => {
                           println!("--------------------------------------------------");
                           println!(
                              "{}: Installed dependency for {}: {}",
                              colour!(amber, "ALERT"),
                              colour!(amber, "{}", &egg.name),
                              colour!(blue, "{}", &dep_egg.name)
                           );
                           println!("--------------------------------------------------");
                        }
                        Err(_) => {
                           return Err((
                              format!(
                                 "Installation of dependency {} failed",
                                 colour!(blue, "{}", &dep_egg.name)
                              ),
                              format!("Skipping {} installation", colour!(amber, "{}", &egg.name)),
                           ))
                        }
                     }
                  }
                  None => {
                     return Err((
                        format!("Dependency {} was not found '.bird-eggs'", colour!(blue, "{}", &d)),
                        format!("Skipping {} installation", colour!(amber, "{}", &egg.name)),
                     ));
                  }
               }
            }
         }
         None => (),
      }

      println!("--------------------------------------------------");
      match egg.install(&config) {
         Ok(_) => {
            nest.append(&egg.name, &config).unwrap();
         }
         Err(_) => {
            return Err((
               format!("Installation {} of failed", colour!(amber, "{}", &egg.name)),
               format!("Skipping {} installation", colour!(amber, "{}", &egg.name)),
            ));
         }
      }
      println!("--------------------------------------------------");

      Ok(())
   }
}
