use crate::utils::errors::BirdError;
use crate::utils::files;
use crate::utils::serializers::eggs;
use crate::{colour, outln};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::process::{Command, Stdio};

use super::BirdConfig;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct EggItem {
   pub name: String,
   pub preinstall: Option<Vec<String>>,
   pub install: Option<Vec<String>>,
   pub update: Option<Vec<String>>,
   pub uninstall: Option<Vec<String>>,
   pub dependencies: Option<Vec<String>>,
}

impl EggItem {
   pub fn install(&self) -> Result<(), BirdError> {
      println!("{} {}", colour!(blue, "Installing",), colour!(green, "{}", &self.name));

      if let Some(preinstall_cmds) = &self.preinstall {
         println!("{} Running preinstall commands", colour!(green, "=>"));
         for command in preinstall_cmds {
            println!("   {} cmd `{}`", colour!(blue, "=>"), colour!(amber, "{}", &command));

            let preinstall_cmd = Command::new("fish")
               .stderr(Stdio::inherit())
               .stdout(Stdio::inherit())
               .args(&["-c", command])
               .status()
               .expect(&format!("command '{}' failed", command));

            if !preinstall_cmd.success() {
               return Err(BirdError::CommandFailed(command.to_owned()));
            }
         }
      }

      if let Some(install_cmds) = &self.install {
         println!("{} Running install commands", colour!(green, "=>"));
         for command in install_cmds {
            println!("   {} cmd `{}`", colour!(blue, "=>"), colour!(amber, "{}", &command));
            let install_cmd = Command::new("fish")
               .stderr(Stdio::inherit())
               .stdout(Stdio::inherit())
               .args(&["-c", command])
               .status()
               .expect(&format!("command '{}' failed", command));

            if !install_cmd.success() {
               return Err(BirdError::CommandFailed(command.to_owned()));
            }
         }

         println!("{}", colour!(green, "{} installed successfully", &self.name));
      } else {
         outln!(
            warn,
            "No install commands found for {}",
            colour!(amber, "{}", &self.name)
         );
      }
      Ok(())
   }

   pub fn update(&self) -> Result<(), BirdError> {
      println!("{} {}", colour!(blue, "Updating",), colour!(green, "{}", &self.name));

      if let Some(update_cmds) = &self.update {
         println!("{} Running update commands", colour!(green, "=>"));
         for command in update_cmds {
            println!("   {} cmd `{}`", colour!(blue, "=>"), colour!(amber, "{}", &command));
            let install_cmd = Command::new("fish")
               .stderr(Stdio::inherit())
               .stdout(Stdio::inherit())
               .args(&["-c", command])
               .status()
               .expect(&format!("command '{}' failed", command));

            if !install_cmd.success() {
               return Err(BirdError::CommandFailed(command.to_owned()));
            }
         }

         println!("{}", colour!(green, "{} updated successfully", &self.name));
      } else {
         outln!(
            warn,
            "No update commands found for {}",
            colour!(amber, "{}", &self.name)
         );
      }
      Ok(())
   }

   pub fn uninstall(&self) -> Result<(), BirdError> {
      println!("{} {}", colour!(blue, "Uninstalling",), colour!(green, "{}", &self.name));

      if let Some(uninstall_cmds) = &self.uninstall {
         println!("{} Running uninstall commands", colour!(green, "=>"));
         for command in uninstall_cmds {
            println!("   {} cmd `{}`", colour!(blue, "=>"), colour!(amber, "{}", &command));
            let install_cmd = Command::new("fish")
               .stderr(Stdio::inherit())
               .stdout(Stdio::inherit())
               .args(&["-c", command])
               .status()
               .expect(&format!("command '{}' failed", command));

            if !install_cmd.success() {
               return Err(BirdError::CommandFailed(command.to_owned()));
            }
         }

         println!("{}", colour!(green, "{} uninstalled successfully", &self.name));
      } else {
         outln!(
            warn,
            "No uninstall commands found for {}",
            colour!(amber, "{}", &self.name)
         );
      }
      Ok(())
   }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Eggs {
   #[serde(with = "eggs")]
   pub eggs: BTreeMap<String, EggItem>,
}

impl Eggs {
   pub fn new(config: &BirdConfig) -> Result<Self, BirdError> {
      Ok(Self {
         eggs: Self::file_to_btreemap(&config)?,
      })
   }

   pub fn file_to_btreemap(config: &BirdConfig) -> Result<BTreeMap<String, EggItem>, BirdError> {
      let json = files::read_file(&config.eggs_file)?;

      let parsed_json: Eggs = match serde_json::from_str(&json) {
         Ok(s) => s,
         Err(err) => return Err(BirdError::JsonError((".bird-egg.json".to_owned(), err.to_string()))),
      };

      Ok(parsed_json.eggs)
   }
}
