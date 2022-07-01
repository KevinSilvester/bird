use std::slice::Windows;

use super::command::Command;
use crate::core::{BirdConfig, EggItem, Eggs, Nest, NestItem};
use crate::utils::errors::BirdError;
use crate::{colour, outln};

#[derive(clap::Parser, Debug)]
#[clap(arg_required_else_help = true)]
pub struct Show {
   /// Show info for a specif program in '.bird-eggs'
   #[clap(multiple_values = false, exclusive = true)]
   program: Option<String>,

   /// List all programs in '.bird-eggs'
   #[clap(long, short, conflicts_with_all = &["program", "installed"])]
   all: bool,

   /// List all programs installed using bird in and in '.bird-eggs'
   #[clap(long, short, conflicts_with_all = &["all", "program"])]
   installed: bool,

   /// List all programs that are in '.bird-eggs' but not installed using bird
   #[clap(long, short, conflicts_with_all = &["all", "program", "installed"])]
   not_installed: bool,

   /// Only show program names
   #[clap(long, short)]
   short: bool,
}

impl Command for Show {
   fn call(self, config: &BirdConfig) -> Result<(), BirdError> {
      let eggs = Eggs::new(&config)?;

      if eggs.eggs.is_empty() {
         println!();
         outln!(warn, "No programs found in '.birds-eggs.json'");
         // println!("\n{}", colour::warn("No programs found in '.birds-eggs.json'"));
         return Ok(());
      }

      if !Nest::exists(&config) {
         Nest::init(&config)?;
      }

      let nest = Nest::new(&config)?;

      match &self.program {
         Some(p) => Self::show_specific(&self, &p, &eggs, &nest)?,
         None => {
            if self.all {
               Self::show_all(&self, &eggs, &nest)?
            } else if self.installed {
               Self::show_installed(&self, &eggs, &nest)?
            } else {
               Self::show_not_installed(&self, &eggs, &nest)?
            }
         }
      }

      Ok(())
   }
}

impl Show {
   fn show_specific(&self, program: &str, eggs: &Eggs, nest: &Nest) -> Result<(), BirdError> {
      match eggs.eggs.get(program) {
         Some(e) => {
            Self::print(&self, e, nest.nest.get(program));
            Ok(())
         }
         None => Err(BirdError::ProgramNotFound(program.to_owned())),
      }
   }

   fn show_all(&self, eggs: &Eggs, nest: &Nest) -> Result<(), BirdError> {
      for (key, value) in &eggs.eggs {
         Self::print(&self, value, nest.nest.get(key));
      }
      Ok(())
   }

   fn show_installed(&self, eggs: &Eggs, nest: &Nest) -> Result<(), BirdError> {
      for (key, value) in &eggs.eggs {
         if nest.nest.contains_key(key) {
            Self::print(&self, value, nest.nest.get(key));
         }
      }
      Ok(())
   }

   fn show_not_installed(&self, eggs: &Eggs, nest: &Nest) -> Result<(), BirdError> {
      for (key, value) in &eggs.eggs {
         if !nest.nest.contains_key(key) {
            Self::print(&self, value, nest.nest.get(key));
         }
      }
      Ok(())
   }

   fn print(&self, egg_item: &EggItem, nest_item: Option<&NestItem>) {
      if *&self.short {
         println!("{}", &egg_item.name);
      } else {
         let date_format = "%Y-%m-%d %H:%M:%S";
         println!("{}: {}", colour!(blue, "name"), colour!(green, "{}", &egg_item.name));

         match nest_item {
            Some(item) => {
               println!("{}: {}", colour!(blue, "installed"), colour!(green, "yes"));
               match item.timestamp {
                  Some(t) => println!("{}: {}", colour!(blue, "install/update date"), t.format(date_format)),
                  None => println!("{}: {}", colour!(blue, "install/update date"), colour!(amber, "null")),
               }
            }
            None => {
               println!("{}: {}", colour!(blue, "installed"), colour!(red, "no"));
               println!("{}: {}", colour!(blue, "install/update date"), colour!(amber, "null"));
            }
         }

         println!("{}:", colour!(blue, "commands"));

         print!("   ");
         match &egg_item.preinstall {
            Some(i) => {
               println!("{}:", colour!(blue, "preinstall"));
               for s in i {
                  print!("      - ");
                  println!("\"{}\"", s)
               }
            }
            None => println!("{}: {}", colour!(blue, "preinstall"), colour!(amber, "null")),
         }

         print!("   ");
         match &egg_item.install {
            Some(i) => {
               println!("{}:", colour!(blue, "install"));
               for s in i {
                  print!("      - ");
                  println!("\"{}\"", s)
               }
            }
            None => println!("{}: {}", colour!(blue, "install"), colour!(amber, "null")),
         }

         print!("   ");
         match &egg_item.update {
            Some(i) => {
               println!("{}:", colour!(blue, "update"));
               for s in i {
                  print!("      - ");
                  println!("\"{}\"", s)
               }
            }
            None => println!("{}: {}", colour!(blue, "update"), colour!(amber, "null")),
         }

         print!("   ");
         match &egg_item.uninstall {
            Some(i) => {
               println!("{}:", colour!(blue, "uninstall"));
               for s in i {
                  print!("      - ");
                  println!("\"{}\"", s)
               }
            }
            None => println!("{}: {}", colour!(blue, "uninstall"), colour!(amber, "null")),
         }

         print!("   ");
         match &egg_item.dependencies {
            Some(i) => {
               println!("{}:", colour!(blue, "dependencies"));
               for s in i {
                  print!("      - ");
                  println!("\"{}\"", s)
               }
            }
            None => println!("{}: {}", colour!(blue, "dependencies"), colour!(amber, "null")),
         }

         println!("\n")
      }
   }
}
