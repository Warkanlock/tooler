#![allow(warnings)]

use serde::Deserialize;
use serde_json::from_reader;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub enum FileType {
    File,
    Folder,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct CommandConfiguration {
    pub name: String,
    pub childs: Vec<CommandConfiguration>,
    pub file_type: String,
    pub example : Option<String>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct CommandList {
    pub command: String,
    pub action: CommandConfiguration,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Configuration {
    pub version: String,
    pub commands: Vec<CommandList>,
}

pub fn read_from_file<P: AsRef<Path>>(path: P) -> Result<Configuration, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let u: Configuration = from_reader(reader)?;

    // Return the `Configuration`.
    println!("Configuration: ✅");

    Ok(u)
}
