use crate::core::BirdConfig;
use crate::utils::errors::BirdError;

pub trait Command: Sized {
   fn call(self, config: &BirdConfig) -> Result<(), BirdError>;
}
