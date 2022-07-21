// use anyhow::Result;
use crate::utils::errors::BirdError;
use dirs;
use std::path::PathBuf;

#[cfg(windows)]
const DEFAULT_SHELL: &str = "powershell";

#[cfg(unix)]
const DEFAULT_SHELL: &str = "bash";

#[derive(clap::Parser, Debug)]
pub struct BirdConfig {
   /// Path to eggs file.
   /// Default path '~/.config/bird/bird-eggs.json'.
   #[clap(
      long = "eggs",
      env = "BIRD_EGGS",
      global = true,
      hide_env_values = true,
      value_name = "FILE_PATH"
   )]
   pub eggs_file: Option<PathBuf>,

   /// Path to nest file.
   /// The nest file is autogerated and
   /// Default path '~/.local/share/bird/bird-nest.json'.
   #[clap(
      long = "nest",
      env = "BIRD_NEST",
      global = true,
      hide_env_values = true,
      value_name = "FILE_PATH"
   )]
   pub nest_file: Option<PathBuf>,

   /// Shell to run bird with.
   /// Default shell 'bash'.
   #[clap(long, multiple_values = false, env = "BIRD_SHELL", global = true, default_value = DEFAULT_SHELL, hide_env_values = true)]
   pub shell: String,
}

impl BirdConfig {
   fn default_eggs_file() -> String {
      match dirs::home_dir() {
         Some(dir) => {
            let mut d = dir.clone();
            d.push(".bird");
            d.push("bird-eggs.json");
            String::from(d.to_str().unwrap())
         }
         None => std::process::exit(1),
      }
   }

   fn default_nest_file() -> String {
      match dirs::home_dir() {
         Some(dir) => {
            let mut d = dir.clone();
            d.push(".bird");
            d.push("bird-nest.json");
            String::from(d.to_str().unwrap())
         }
         None => std::process::exit(1),
      }
   }

   pub fn eggs_file_path(&self) -> Result<String, BirdError> {
      match &self.eggs_file {
         Some(p) => Ok(p.to_str().unwrap().to_owned()),
         None => Ok(Self::default_eggs_file()),
      }
   }

   pub fn nest_file_path(&self) -> Result<String, BirdError> {
      match &self.nest_file {
         Some(p) => Ok(p.to_str().unwrap().to_owned()),
         None => Ok(Self::default_nest_file()),
      }
   }
}
