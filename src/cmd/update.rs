use crate::{cli_update_successful, proceed_with_the_installation};
use crate::{
    update_get_latest_version_err, update_has_new_version, update_using_the_latest_version,
};
use colored::Colorize;
use std::{
    io::{self, Write},
    process::Command,
};

use super::get_latest_version;

pub fn update() {
    let new_version = match get_latest_version_and_eq_now_version() {
        Some(v) => v,
        None => return,
    };

    #[cfg(not(windows))]
    normal_update(&new_version);

    #[cfg(windows)]
    windows_update(&new_version);
}

fn get_latest_version_and_eq_now_version() -> Option<String> {
    let now_version = env!("CARGO_PKG_VERSION");
    let new_version = match get_latest_version("kovi-cli") {
        Ok(v) => v,
        Err(e) => {
            let err = format!("{e}");
            let msg = update_get_latest_version_err(&err);
            eprintln!("{msg}");
            return None;
        }
    };

    if now_version == new_version {
        let msg = update_using_the_latest_version(&new_version);
        println!("\n{}", msg.green(),);
        return None;
    }

    Some(new_version)
}

#[cfg(not(windows))]
pub fn normal_update(new_version: &str) {
    use crate::run_cargo_command_return;

    {
        let update_has_new_version = update_has_new_version();
        let proceed_with_the_installation = proceed_with_the_installation();

        // [Y/n] 确认
        print!(
            "{update_has_new_version}\n{}\n:: {proceed_with_the_installation} [Y/n]",
            format!("({new_version})").green(),
        );
    }
    io::stdout().flush().unwrap();

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to io stdin");

    let input = input.to_lowercase();
    let input = input.trim();
    if input == "n" {
        return;
    }

    let mut cargo_command = Command::new("cargo");
    cargo_command.arg("install").arg("kovi-cli");

    run_cargo_command_return!(cargo_command);

    let msg = cli_update_successful();
    println!("\n{}", msg.green(),);
}

#[cfg(windows)]
pub fn windows_update(new_version: &str) {
    use crate::update_windows_manually_to_use;

    let update_has_new_version = update_has_new_version();

    println!(
        "{update_has_new_version}\n{}",
        format!("({new_version})").green()
    );

    let msg = update_windows_manually_to_use();

    println!("\n{msg}");
    println!("    cargo install kovi-cli");
}
