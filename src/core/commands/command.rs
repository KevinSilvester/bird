use crate::core::BirdConfig;
use anyhow::Result;

pub trait Command: Sized {
   fn call(self, config: &BirdConfig) -> Result<()>;

   // fn call(self, config: &BirdConfig) {
   //    match self.call(&config) {
   //       Ok(()) => (),
   //       Err(err) => anyhow::anyhow!(err)
   //    }
   // }
}
