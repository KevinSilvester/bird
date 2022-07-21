use crate::core::{
   commands::{Chirp, Command as _, Install, Show, Uninstall, Update},
   BirdConfig,
};
use crate::utils::errors::BirdError;

const INSTALL_ABOUT: &str = "Install a specific program with using the installating commands set in '.bird-eggs.json'";

#[derive(clap::Parser, Debug)]
pub enum SubCommand {
   #[clap(name = "install", bin_name = "install", visible_alias = "i", long_about = INSTALL_ABOUT)]
   Install(Install),

   /// Update a specific program with using the update commands set in '.bird-eggs.json'
   #[clap(name = "update", bin_name = "update", visible_alias = "up")]
   Update(Update),

   /// Uninstall a specific program with using the uninstallating commands set in '.bird-eggs.json'
   #[clap(name = "uninstall", bin_name = "uninstall", visible_alias = "un")]
   Uninstall(Uninstall),

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
         Self::Update(cmd) => Ok(cmd.call(&config)?),
         Self::Uninstall(cmd) => Ok(cmd.call(&config)?),
         Self::Show(cmd) => Ok(cmd.call(&config)?),
         Self::Chirp(cmd) => Ok(cmd.call(&config)?),
      }
   }
}

#[derive(clap::Parser, Debug)]
#[clap(name = env!("CARGO_PKG_NAME"), version = env!("CARGO_PKG_VERSION"), bin_name = env!("CARGO_PKG_NAME"), long_about = "")]
pub struct BirdCli {
   #[clap(flatten)]
   pub config: BirdConfig,

   #[clap(subcommand)]
   pub subcmd: SubCommand,
}
