mod core;
mod utils;

use anyhow::Result;
use crate::core::{BirdConfig, BirdCli};

fn main() -> Result<()> {
   let config = BirdConfig::new()?;
   let value = crate::core::parse();
   println!("{:?}", value);

   value.subcmd.call(config);
   Ok(())
}
