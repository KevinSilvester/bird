use crate::core::BirdConfig;
use crate::utils::colour;
use crate::core::commands::{Command as _, Chirp, Install};
use clap::{Arg, ArgMatches, Command, ValueHint, Parser};
use anyhow::Result;

#[derive(clap::Parser, Debug)]
pub enum SubCommand {
   #[clap(name = "install", bin_name = "install")]
   /// Install a specific program with using the installating commands set in '.bird-eggs.json'
   Install(Install),

   #[clap(name = "update", bin_name = "update")]
   /// Update a specific program with using the update commands set in '.bird-eggs.json'
   Update,

   #[clap(name = "uninstall", bin_name = "uninstall")]
   /// Uninstall a specific program with using the uninstallating commands set in '.bird-eggs.json'
   Uninstall,

   #[clap(name = "list", bin_name = "list")]
   /// List all programs in '.bird-eggs.json'
   List,

   #[clap(name = "chirp", bin_name = "chirp")]
   /// Chirp
   Chirp(Chirp)
}

impl SubCommand {
   pub fn call(self, config: BirdConfig) -> Result<()> {
      match self {
         Self::Install(cmd) => Ok(cmd.call(&config).unwrap()),
         Self::Update => Ok(println!("{}", colour::success("Updating..."))),
         Self::Uninstall => Ok(println!("{}", colour::warn("Uninstalling..."))),
         Self::List => Ok(println!("{}", colour::error("Listing..."))),
         Self::Chirp(cmd) => Ok(cmd.call(&config).unwrap())
      }
   }
}

#[derive(clap::Parser, Debug)]
#[clap(name = env!("CARGO_PKG_NAME"), version = env!("CARGO_PKG_VERSION"), bin_name = env!("CARGO_PKG_NAME"))]
pub struct BirdCli {
   #[clap(subcommand)]
   pub subcmd: SubCommand
}

pub fn parse() -> BirdCli {
   BirdCli::parse()
}
