use clap::{Arg, ArgMatches, Command, ValueHint};

const COMMAND_NAME: String = "bird".to_owned();
const VERSION: String = "v0.1.0".to_owned();

pub enum CliSubCommands {
   Install(())
}

pub struct Cli {
   pub command_name: String,
   pub version: String,
}

impl Cli {

}
