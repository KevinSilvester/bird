use crate::utils::errors::BirdError;
use crate::utils::serializers::eggs;
use crate::utils::{colour, files};
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
      println!("{}", colour::info(&format!("\n*** Installing {}***", self.name)));

      if let Some(preinstall) = &self.preinstall {
         for command in preinstall {
            println!("{}", colour::info(&format!("Running preinstall command: {}", command)));

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

      if let Some(install) = &self.install {
         for command in install {
            println!("{}", colour::info(&format!("Running install command: {}", command)));

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

         println!("{}", colour::success(&format!("{} installed successfully", &self.name)))
      } else {
         println!("{}", colour::warn("No install commands"));
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
