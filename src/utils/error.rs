// // use thiserror::Error;
// use anyhow::Error

// #[derive(Error, Debug)]
// #[error("BirdError: {0}")]
// pub enum BirdError{
//    #[error("file/directory not found: {0}")]
//    IoError(#[from] std::io::Error),
   
//    #[error("invalid json: {0}")]
//    JsonError(#[from] serde_json::Error),

//    #[error("hello ")]
//    MissingEnvVar(#[from] std::env::VarError),
// }