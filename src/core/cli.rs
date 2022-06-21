use crate::core::{
   commands::{Command as _, Install, Show, Chirp},
   BirdConfig,
};
use crate::utils::{colour, errors::BirdError};

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
   #[clap(name = "show", bin_name = "show")]
   Show(Show),

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
         Self::Show(cmd) => Ok(cmd.call(&config)?),
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
