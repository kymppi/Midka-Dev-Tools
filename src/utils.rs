use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process::Command;

use colored::Colorize;
use tempfile::tempdir;
use tempfile::TempDir;
use toml::value::Table;

pub fn clone_github_repo(repo: &str) -> TempDir {
    let dir = tempdir().unwrap();

    // run the git clone command
    //let status =
    Command::new("git")
        .arg("clone")
        .arg(repo)
        .arg(".")
        .current_dir(&dir)
        .output()
        .expect("Something went wrong with git clone");

    //println!("{:?}", status);

    dir
}

pub fn run_command(command: &str, path: Option<std::path::PathBuf>) -> Result<(), anyhow::Error> {
    if command.is_empty() {
        return Ok(());
    }

    let path = path.unwrap_or(".".into());

    let result = Command::new("sh")
        .arg("-c")
        .arg(command)
        .current_dir(path)
        .output();

    if result.is_err() {
        return Err(anyhow::anyhow!(
            "{} {}",
            "Error running command:".red(),
            command
        ));
    }

    println!("{} {}", "Command executed:".cyan(), command);

    Ok(())
}

pub fn get_command<'a>(table: &'a Table, key: &'a str) -> &'a str {
    let command_value = table.get(&key.to_lowercase());

    let command = match command_value {
        Some(value) => value.as_str().unwrap_or(""),
        None => "",
    };

    command
}

pub fn run_commands_from_config(
    table: &Table,
    r#type: &str,
    language: &str,
    path: &PathBuf,
    status: &mut HashMap<String, bool>,
) {
    let install_result = run_command(get_command(&table, &language), Some(path.clone()));

    match install_result {
        Ok(_) => status.insert(r#type.to_string(), true),
        Err(e) => {
            println!("{}", e.to_string().red());
            status.insert(r#type.to_string(), false);
            return;
        }
    };
}

pub fn copy_dir_all(src: &PathBuf, dst: &PathBuf) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(&entry.path(), &dst.as_path().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), &dst.as_path().join(entry.file_name()))?;
        }
    }
    Ok(())
}