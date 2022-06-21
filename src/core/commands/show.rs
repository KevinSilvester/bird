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
   #[clap(long, short, exclusive = true)]
   pub all: bool,

   /// List all programs installed using bird in and in '.bird-eggs'
   #[clap(long, short, exclusive = true)]
   pub installed: bool,

   /// List all programs that are in '.bird-eggs' but not installed using bird
   #[clap(long, short, exclusive = true)]
   pub not_installed: bool,
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

      match self.program {
         Some(p) => Self::show_specific(&p, &eggs, &nest)?,
         None => {
            if self.all {
               Self::show_all(&eggs, &nest)?
            } else if self.installed {
               Self::show_installed(&eggs, &nest)?
            } else {
               Self::show_not_installed(&eggs, &nest)?
            }
         }
      }

      Ok(())
   }
}

impl Show {
   fn show_specific(program: &str, eggs: &Eggs, nest: &Nest) -> Result<(), BirdError> {
      match eggs.eggs.get(program) {
         Some(e) => Ok(Self::print(e, nest.nest.get(program))),
         None => Err(BirdError::ProgramNotFound(program.to_owned())),
      }
   }

   fn show_all(eggs: &Eggs, nest: &Nest) -> Result<(), BirdError> {
      for (key, value) in &eggs.eggs {
         Self::print(value, nest.nest.get(key));
      }
      Ok(())
   }

   fn show_installed(eggs: &Eggs, nest: &Nest) -> Result<(), BirdError> {
      for (key, value) in &eggs.eggs {
         if nest.nest.contains_key(key) {
            Self::print(value, nest.nest.get(key));
         }
      }
      Ok(())
   }

   fn show_not_installed(eggs: &Eggs, nest: &Nest) -> Result<(), BirdError> {
      for (key, value) in &eggs.eggs {
         if !nest.nest.contains_key(key) {
            Self::print(value, nest.nest.get(key));
         }
      }
      Ok(())
   }

   fn print(egg_item: &EggItem, nest_item: Option<&NestItem>) {
      const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";
      println!("{}: {}", colour!(blue, "name"), colour!(green, "{}", &egg_item.name));

      match nest_item {
         Some(item) => {
            println!("{}: {}", colour!(blue, "installed"), colour!(green, "yes"));
            match item.timestamp {
               Some(t) => println!("{}: {}", colour!(blue, "install/update date"), t.format(FORMAT)),
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
