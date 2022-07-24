mod core;
mod utils;

use crate::core::BirdCli;
use crate::utils::errors::BirdError;
use clap::Parser;

fn main() {
   match run_main() {
      Ok(_) => (),
      Err(err) => eprintln!("{}", err),
   }
}

fn run_main() -> Result<(), BirdError> {
   let values = BirdCli::parse();

   // dbg!(&values);

   values.subcmd.call(values.config)?;
   Ok(())
}
