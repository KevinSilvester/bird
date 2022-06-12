use crate::utils::errors::BirdError;
use std::fs;
use std::fs::File;

pub fn create_file(path: &str) -> Result<(), BirdError> {
   File::create(path)?;
   Ok(())
}

pub fn write_file(path: &str, content: &str) -> Result<(), BirdError> {
   std::fs::write(path, content)?;
   Ok(())
}

pub fn read_file(path: &str) -> Result<String, BirdError> {
   let contents = match fs::read_to_string(path) {
      Ok(val) => val,
      Err(_) => return Err(BirdError::IoError(path.to_string())),
   };
   Ok(contents)
}

