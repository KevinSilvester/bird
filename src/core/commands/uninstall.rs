use super::command::Command;
use crate::core::{BirdConfig, EggItem, Eggs, Nest};
use crate::utils::errors::BirdError;
use crate::{colour, outln};
use dialoguer::{theme::ColorfulTheme, Confirm};

#[derive(clap::Parser, Debug)]
#[clap(arg_required_else_help = true)]
pub struct Uninstall {
   /// List of programs to be updated
   #[clap(multiple_values = true)]
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

impl Command for Uninstall {
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
         true => Self::uninstall_all(self, &eggs, &mut nest, &config),
         false => Self::uninstall_selected(self, &eggs, &mut nest, &config)?,
      }

      Ok(())
   }
}

impl Uninstall {
   fn uninstall_all(self, eggs: &Eggs, nest: &mut Nest, config: &BirdConfig) {
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
                     false => outln!(info, "{} hasn't been installed yet!", colour!(amber, "{}", key)),
                     true => match Self::uninstall_program(&value.clone(), &eggs, nest, &config) {
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
         false => outln!(warn, "Uninstallation cancelled!"),
      }
   }

   fn uninstall_selected(self, eggs: &Eggs, nest: &mut Nest, config: &BirdConfig) -> Result<(), BirdError> {
      let mut invalid_eggs = Vec::new();

      for program in self.programs {
         match nest.nest.contains_key(&program) {
            false => outln!(info, "{} hasn't been installed yet!", colour!(amber, "{}", program)),
            true => match eggs.eggs.get(&program) {
               Some(e) => match Self::uninstall_program(&e, &eggs, nest, &config) {
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

   fn uninstall_program(
      egg: &EggItem,
      _eggs: &Eggs,
      nest: &mut Nest,
      config: &BirdConfig,
   ) -> Result<(), (String, String)> {
      println!("--------------------------------------------------");
      match egg.uninstall() {
         Ok(_) => {
            nest.remove(&egg.name, &config).unwrap();
         }
         Err(_) => {
            return Err((
               format!("Update {} of failed", colour!(amber, "{}", &egg.name)),
               format!("Skipping {} update", colour!(amber, "{}", &egg.name)),
            ));
         }
      }
      println!("--------------------------------------------------");

      Ok(())
   }
}
