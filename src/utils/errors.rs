use crate::colour;
use serde_json::error::Category;
use std::fmt;

#[derive(Debug)]
pub enum BirdError {
   IoError((String, std::io::Error)),
   // NotFile(String),
   // NotDir(String),
   TreeNotFound,
   ProgramsNotFound(Vec<String>),
   JsonError((String, String)),
   CommandFailed(String),
   Logger(String),
}

impl fmt::Display for BirdError {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match *self {
         BirdError::IoError((ref path, ref err)) => {
            writeln!(
               f,
               "{}: {} {err}",
               colour!(red, "ERROR"),
               colour!(amber, "{path}")
            )
         }
         // BirdError::NotFile(ref str) => {
         //    writeln!(
         //       f,
         //       "{}: Path {} is not a file",
         //       colour!(red, "ERROR"),
         //       colour!(amber, "{str}")
         //    )
         // }
         // BirdError::NotDir(ref str) => {
         //    writeln!(
         //       f,
         //       "{}: Path {} is not a directory",
         //       colour!(red, "ERROR"),
         //       colour!(amber, "{str}")
         //    )
         // }
         BirdError::TreeNotFound => writeln!(
            f,
            "{}: The environment variable 'BIRD_TREE' was not found",
            colour!(red, "ERROR")
         ),
         BirdError::ProgramsNotFound(ref programs) => writeln!(
            f,
            "{}: The program(s) [{}] were not found in {}",
            colour!(red, "ERROR"),
            programs.join(", "),
            colour!(amber, ".bird-eggs.json")
         ),
         BirdError::JsonError((ref file, ref msg)) => {
            writeln!(f, "{}: {} - {}", colour!(red, "ERROR"), colour!(blue, "{}", file), msg)
         }
         BirdError::CommandFailed(ref str) => {
            writeln!(
               f,
               "{}: {} - {}",
               colour!(red, "ERROR"),
               colour!(amber, "Command Failed"),
               str
            )
         }
         BirdError::Logger(ref err) => {
            writeln!(f, "{}: {}", colour!(red, "ERROR"), err)
         }
      }
   }
}

impl From<std::io::Error> for BirdError {
   fn from(err: std::io::Error) -> Self {
      BirdError::IoError(("".to_owned(), err))
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

impl From<format_serde_error::SerdeError> for BirdError {
   fn from(err: format_serde_error::SerdeError) -> Self {
      BirdError::JsonError(("JSON Error".to_owned(), err.to_string()))
   }
}

impl From<fern::InitError> for BirdError {
   fn from(err: fern::InitError) -> Self {
      BirdError::Logger(err.to_string())
   }
}
