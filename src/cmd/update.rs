use colored::Colorize;
use std::{io, process::Command};

use super::get_latest_version;

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

    // [Y/n] 确认
    println!(
        "There is a new version of kovi-cli\n{}",
        format!("({new_version})").truecolor(202, 225, 205)
    );
    print!("Proceed with the installation? [Y/n]");

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
            println!(
                "\n{}",
                "Kovi cli update successful!".truecolor(202, 225, 205),
            );
        }
        Ok(status) => {
            eprintln!("Cargo exited with status: {}", status);
        }
        Err(e) => {
            eprintln!("Failed to execute cargo: {}", e);
        }
    }
}
