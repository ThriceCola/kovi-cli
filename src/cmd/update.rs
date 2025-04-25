use colored::Colorize;
use std::{
    io::{self, Write},
    process::Command,
};

use super::get_latest_version;

#[cfg(not(windows))]
pub fn update() {
    use crate::{
        cli_update_successful, proceed_with_the_installation, update_get_latest_version_err,
        update_has_new_version, update_using_the_latest_version,
    };

    let now_version = env!("CARGO_PKG_VERSION");
    let new_version = match get_latest_version("kovi-cli") {
        Ok(v) => v,
        Err(e) => {
            let err = format!("{e}");
            let msg = update_get_latest_version_err(&err);
            eprintln!("{msg}");
            return;
        }
    };

    if now_version == new_version {
        let msg = update_using_the_latest_version(&new_version);
        println!("\n{}", msg.truecolor(202, 225, 205),);
        return;
    }

    {
        let update_has_new_version = update_has_new_version();
        let proceed_with_the_installation = proceed_with_the_installation();

        // [Y/n] 确认
        print!(
            "{}\n{}\n:: {} [Y/n]",
            update_has_new_version,
            format!("({new_version})").truecolor(202, 225, 205),
            proceed_with_the_installation
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

    match cargo_command.status() {
        Ok(status) if status.success() => {
            let msg = cli_update_successful();

            println!("\n{}", "msg".truecolor(202, 225, 205),);
        }
        Ok(status) => {
            eprintln!("Cargo exited with status: {}", status);
        }
        Err(e) => {
            eprintln!("Failed to execute cargo: {}", e);
        }
    }
}

#[cfg(windows)]
pub fn update() {
    let now_version = env!("CARGO_PKG_VERSION");
    let new_version = match get_latest_version("kovi-cli") {
        Ok(v) => v,
        Err(e) => {
            eprintln!(
                "Failed to get latest version: {}, please check your network connection.",
                e
            );
            return;
        }
    };

    if now_version == new_version {
        println!(
            "\n{}",
            format!("You are using the latest version ({new_version}) of Kovi cli.")
                .truecolor(202, 225, 205),
        );
        return;
    }

    println!(
        "There is a new version of kovi-cli\n{}",
        format!("({new_version})").truecolor(202, 225, 205)
    );

    println!("\nSorry");
    println!("On Windows, please manually update by running:");
    println!("    cargo install kovi-cli");
}
