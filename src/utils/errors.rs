use super::colour;
use std::fmt;
use serde_json::error::Category;

#[derive(Debug)]
pub enum BirdError {
   IoError(String),
   TreeNotFound,
   ProgramsNotFound(Vec<String>),
   JsonError((String, String)),
   CommandFailed(String),
}

impl fmt::Display for BirdError {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
         &BirdError::IoError(ref str) => writeln!(
            f,
            "{}: The file/directory {} was not found",
            colour::error("Error"),
            str
         ),
         &BirdError::TreeNotFound => writeln!(
            f,
            "{}: The environment variable 'BIRD_TREE' was not found",
            colour::error("Error")
         ),
         &BirdError::ProgramsNotFound(ref programs) => writeln!(
            f,
            "{}: The programs [{}] were not found in .bird-eggs.json",
            colour::error("Error"),
            programs.join(", ")
         ),
         &BirdError::JsonError((ref file, ref msg )) => {
            writeln!(
               f,
               "{}: {} - {}",
               colour::error("JsonError"),
               colour::warn(file),
               msg
            )
         }
         &BirdError::CommandFailed(ref str) => {
            writeln!(
               f,
               "{}: {} - {}",
               colour::error("Error"),
               colour::warn("Command Failed"),
               str
            )
         }
      }
   }
}

impl From<std::io::Error> for BirdError {
   fn from(err: std::io::Error) -> Self {
      BirdError::IoError(err.to_string())
   }
}

impl From<std::env::VarError> for BirdError {
   fn from(_err: std::env::VarError) -> Self {
      BirdError::TreeNotFound
   }
}

impl From<serde_json::Error> for BirdError {
   fn from(err: serde_json::Error) -> Self {
      match err.classify() {
         Category::Eof => BirdError::JsonError(("Eof".to_owned(), err.to_string())),
         Category::Io => BirdError::JsonError(("Io".to_owned(), err.to_string())),
         Category::Syntax => BirdError::JsonError(("Syntax".to_owned(), err.to_string())),
         Category::Data => BirdError::JsonError(("Data".to_owned(), err.to_string())),
      }
   }
}
