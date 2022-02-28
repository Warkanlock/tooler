use clap::Parser;
mod commands;
mod json_serializer;
use std::{env, process};

use crate::json_serializer::CommandList;

#[derive(Parser, Debug)]
#[clap(
    name = "~ Tooler ~",
    author = "Warkanlock",
    about = "Create a controlled ``folder>file`` structure for your team to use as a configuration file to prevent team mattes from creating their own structures or files by hand. Keep the configuration synchronized as a source of truth for the creation of ``components``, ``views``, ``services`` or whatever in your project. Add more commands inside the configuration file as an action object."
)]
struct Context {
    #[clap(
        short,
        long,
        help = "Use this to run commands you specified on your config file [command|view|service|...]"
    )]
    command: String,
}

fn main() {
    let arguments = Context::parse();
    println!("Invoke command {:?}", arguments.command);

    // validate if command exists on config object
    let config_path = env::var("CONFIG_PATH").unwrap_or_else(|err| {
        eprintln!("Error trying to read environment variable: {}", err);
        process::exit(1);
    });

    println!("Configuration file loaded from: {}", config_path);
    let configuration: json_serializer::Configuration =
        json_serializer::read_from_file(config_path).unwrap_or_else(|err| {
            eprintln!("Error trying to read the configuration file: {}", err);
            process::exit(1);
        });

    let list_of_commands: &Vec<CommandList> = &configuration.commands;

    // match command with real command
    match arguments.command.as_str() {
        "all" => commands::all(list_of_commands),
        x if list_of_commands.iter().any(|t| t.command == x) => {
            let actual_command = list_of_commands.iter().find(|t| t.command == x);

            if let Some(x) = actual_command {
                commands::run_command(x);
            }
        }
        _ => {
            println!("Command not specified");
            process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    // we set this path as a test object for the configuration file
    const CONFIG_PATH_TEST: &str = "./src/tests/config.test.json";
    const BASE_PATH_TEST: &str = "./src/tests/output/";

    #[test]
    fn read_env_variable() {
        temp_env::with_var("CONFIG_PATH", Some(CONFIG_PATH_TEST), || {
            assert_eq!(std::env::var("CONFIG_PATH").unwrap(), CONFIG_PATH_TEST);
        })
    }

    #[test]
    fn command_not_existent() -> Result<(), Box<dyn std::error::Error>> {
        let mut command = Command::cargo_bin("tooler")?;

        command.arg("--asd");

        command.assert().failure().stderr(predicate::str::contains(
            "Found argument '--asd' which wasn't expected",
        ));

        Ok(())
    }

    #[test]
    fn read_config_from_env_variable() {
        temp_env::with_var("CONFIG_PATH", Some(CONFIG_PATH_TEST), || {
            let config_path = std::env::var("CONFIG_PATH").unwrap();

            let configuration = json_serializer::read_from_file(config_path).unwrap();

            assert_eq!(configuration.commands.len(), 2);
            assert_eq!(configuration.commands[0].command, "test-command");
            assert_eq!(configuration.commands[1].command, "test-view");
        })
    }

    #[test]
    fn create_from_config() {
        temp_env::with_vars(
            vec![
                ("CONFIG_PATH", Some(CONFIG_PATH_TEST)),
                ("BASE_PATH", Some(BASE_PATH_TEST)),
            ],
            || {
                let config_path = std::env::var("CONFIG_PATH").unwrap();
                let output_path = std::env::var("BASE_PATH").unwrap();

                let configuration = json_serializer::read_from_file(config_path).unwrap();
                let output_path_dir = std::path::Path::new(&output_path);

                // create tmp output dir
                std::fs::create_dir(output_path_dir).unwrap();

                // run command
                commands::run_command(&configuration.commands[0]);

                // check if file got created
                assert_eq!(std::path::Path::exists(output_path_dir), true);

                // delete tmp output dir
                std::fs::remove_dir_all(output_path_dir).unwrap();
            },
        )
    }
}
