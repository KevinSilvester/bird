use crate::utils::errors::BirdError;
use std::fs;
use std::fs::File;
use std::path::PathBuf;

pub fn create_file(path: &str) -> Result<(), BirdError> {
   let path_buf = PathBuf::from(path);
   let dirs = match path_buf.parent() {
      Some(d) => d,
      None => &path_buf,
   };
   fs::create_dir_all(dirs)?;
   match File::create(path) {
      Ok(_) => Ok(()),
      Err(err) => Err(BirdError::IoError((path.to_string(), err))),
   }
}

pub fn write_file(path: &str, content: &str) -> Result<(), BirdError> {
   match fs::write(path, content) {
      Ok(_) => Ok(()),
      Err(err) => Err(BirdError::IoError((path.to_string(), err))),
   }
}

pub fn read_file(path: &str) -> Result<String, BirdError> {
   let contents = match fs::read_to_string(path) {
      Ok(val) => val,
      Err(err) => return Err(BirdError::IoError((path.to_string(), err))),
   };
   Ok(contents)
}
