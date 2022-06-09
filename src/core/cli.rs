use crate::core::commands::{Chirp, Command as _, Install};
use crate::core::BirdConfig;
use crate::utils::colour;
use crate::utils::errors::BirdError;
use clap::Parser;
// use anyhow::Result;

#[derive(clap::Parser, Debug)]
pub enum SubCommand {
   /// Install a specific program with using the installating commands set in '.bird-eggs.json'
   #[clap(name = "install", bin_name = "install")]
   Install(Install),

   /// Update a specific program with using the update commands set in '.bird-eggs.json'
   #[clap(name = "update", bin_name = "update")]
   Update,

   /// Uninstall a specific program with using the uninstallating commands set in '.bird-eggs.json'
   #[clap(name = "uninstall", bin_name = "uninstall")]
   Uninstall,

   /// List all programs in '.bird-eggs.json'
   #[clap(name = "list", bin_name = "list")]
   List,

   /// Chirp
   #[clap(name = "chirp", bin_name = "chirp")]
   Chirp(Chirp),
}

impl SubCommand {
   pub fn call(self, config: BirdConfig) -> Result<(), BirdError> {
      match self {
         Self::Install(cmd) => Ok(cmd.call(&config)?),
         Self::Update => Ok(println!("{}", colour::success("Updating..."))),
         Self::Uninstall => Ok(println!("{}", colour::warn("Uninstalling..."))),
         Self::List => Ok(println!("{}", colour::error("Listing..."))),
         Self::Chirp(cmd) => Ok(cmd.call(&config)?),
      }
   }
}

#[derive(clap::Parser, Debug)]
#[clap(name = env!("CARGO_PKG_NAME"), version = env!("CARGO_PKG_VERSION"), bin_name = env!("CARGO_PKG_NAME"))]
pub struct BirdCli {
   #[clap(subcommand)]
   pub subcmd: SubCommand,
}

pub fn parse() -> BirdCli {
   BirdCli::parse()
}
