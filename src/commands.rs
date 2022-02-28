use crate::json_serializer::{CommandConfiguration, CommandList};
use std::io::prelude::*;
use std::{fs::File, path::Path, process};

const BASE_PATH: &str = std::env!("BASE_PATH");

pub fn all(commands: &[CommandList]) {
    for command in commands {
        println!(
            "\x1b[93mTemplate\x1b[0m for {:?} ✔️ (found)",
            command.command
        );
        println!("\x1b[93mAction\x1b[0m: \n {:?}", command.action);
        println!("\n");
    }
}

/**
 * Recursive function that run and execute the creation of the files and folders
 */
pub fn run_command(command: &CommandList) {
    println!("Action: \x1b[93m{:?}\x1b[0m", command.command);
    run_on_childs(&command.action, &String::from("")).unwrap(); // initial action
}

/**
 * Create childs based on a CommandConfiguration object
 */
fn run_on_childs(
    action: &CommandConfiguration,
    relative_child: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let final_name = format!("{}{}{}", BASE_PATH, relative_child, action.name);

    // create root
    match action.file_type.as_str() {
        "folder" => {
            std::fs::create_dir(&final_name).unwrap_or_else(|e| {
                eprintln!("Error trying to create a folder: {}", e);
                process::exit(1);
            });
        }
        "file" => {
            let mut file = File::create(Path::new(&final_name)).unwrap_or_else(|e| {
                eprintln!("Error trying to create a file: {}", e);
                process::exit(1);
            });
            if action.example.is_some() {
                file.write_all(&(action.example).as_ref().unwrap().as_bytes())
                    .unwrap();
            }
            file.sync_all()?;
        }
        _ => panic!("No file type available"),
    };

    println!(
        "\x1b[93m{}\x1b[0m created on \x1b[42m{}\x1b[0m - ✔️",
        action.file_type, final_name
    );

    if action.childs.is_empty() {
        return Ok(());
    }

    // continue with recursive childs
    for child in &action.childs {
        run_on_childs(child, &format!("{}{}/", relative_child, action.name)).unwrap();
    }

    Ok(())
}
