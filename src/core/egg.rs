use crate::utils::colour;
use crate::utils::errors::BirdError;
use serde::{Deserialize, Serialize};
use std::process::{Command, Stdio};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Egg {
   pub name: String,
   pub preinstall: Option<Vec<String>>,
   pub install: Option<Vec<String>>,
   pub update: Option<Vec<String>>,
   pub uninstall: Option<Vec<String>>,
   pub dependencies: Option<Vec<String>>,
}

impl Egg {
   pub fn install(&self) -> Result<(), BirdError> {
      println!("{}", colour::info(&format!("\n*** Installing {}***", self.name)));

      if let Some(preinstall) = &self.preinstall {
         for command in preinstall {
            println!(
               "{}",
               colour::info(&format!("\nRunning preinstall command: {}", command))
            );

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
      } else {
         println!("{}", colour::warn("\nNo preinstall commands"));
      }

      if let Some(install) = &self.install {
         for command in install {
            println!(
               "{}",
               colour::info(&format!("\nRunning install command: {}", command))
            );

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
      } else {
         println!("{}", colour::warn("\nNo install commands"));
      }

      Ok(())
   }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EggsWrapper {
   #[serde(with = "eggs")]
   pub eggs: HashMap<String, Egg>
}

// ref: https://github.com/serde-rs/serde/issues/936#issuecomment-302281792
mod eggs {
   use super::Egg;
   use std::collections::HashMap;
   use serde::ser::Serializer;
   use serde::de::{Deserialize, Deserializer};
   
   pub fn serialize<S>(map: &HashMap<String, Egg>, serializer:S) -> Result<S::Ok, S::Error>
      where S: Serializer
   {
      serializer.collect_seq(map.values())
   }

   pub fn deserialize<'de, D>(deserializer: D) -> Result<HashMap<String, Egg>, D::Error>
   where D: Deserializer<'de>
   {
      let mut map = HashMap::new();
      for egg in Vec::<Egg>::deserialize(deserializer)? {
         map.insert(egg.name.clone(), egg);
      }
      Ok(map)
   }
}
