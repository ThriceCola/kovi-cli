use crate::cmd::DEFAULT_PLUGIN_CODE;
use crate::cmd::SIMPLE_PLUGIN_CODE;
use colored::Colorize;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

pub fn new_plugin(name: String, simple: bool, prefix: bool) {
    if name.is_empty() {
        eprintln!("Name cannot be empty");
        return;
    }

    let name = if prefix {
        "kovi-plugin-".to_string() + &name
    } else {
        name
    };

    let plugin_path = PathBuf::from("plugins".to_string()).join(&name);

    {
        //检测根项目Cargo.toml是否有[workspace]
        let mut cargo_toml = std::fs::OpenOptions::new()
            .read(true)
            .open("Cargo.toml")
            .unwrap_or_else(|e| exit_and_eprintln(e));

        let mut contents = String::new();
        cargo_toml
            .read_to_string(&mut contents)
            .unwrap_or_else(|e| exit_and_eprintln(e));

        if !contents.contains("[workspace]") {
            eprintln!("This is not a workspace");
            return;
        }
    }

    {
        let mut cargo_command = Command::new("cargo");
        cargo_command.arg("new").arg(&plugin_path).arg("--lib");

        match cargo_command.status() {
            Ok(status) if status.success() => {
                let lib_rs_path = plugin_path.join("src").join("lib.rs");
                let mut lib_rs = std::fs::OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .open(lib_rs_path)
                    .unwrap_or_else(|e| exit_and_eprintln(e));

                if simple {
                    lib_rs
                        .write_all(SIMPLE_PLUGIN_CODE.as_bytes())
                        .unwrap_or_else(|e| exit_and_eprintln(e));
                } else {
                    lib_rs
                        .write_all(DEFAULT_PLUGIN_CODE.as_bytes())
                        .unwrap_or_else(|e| exit_and_eprintln(e));
                }

                let cargo_toml_path = plugin_path.join("Cargo.toml");
                let mut cargo_toml = std::fs::OpenOptions::new()
                    .append(true)
                    .open(cargo_toml_path)
                    .unwrap_or_else(|e| exit_and_eprintln(e));

                writeln!(cargo_toml, "kovi.workspace = true")
                    .unwrap_or_else(|e| exit_and_eprintln(e));

                println!(
                    "\n{}",
                    format!("Plugin '{}' created successfully!", name).truecolor(202, 225, 205),
                );
            }
            Ok(status) => {
                eprintln!("Cargo exited with status: {}", status);
                return;
            }
            Err(e) => {
                eprintln!("Failed to execute cargo: {}", e);
                return;
            }
        }
    }


    let mut cargo_command = Command::new("cargo");
    cargo_command.arg("add").arg("--path").arg(plugin_path);

    match cargo_command.status() {
        Ok(status) if status.success() => {
            println!(
                "\n{}",
                format!("Plugin '{}' add successfully!", name).truecolor(202, 225, 205),
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

fn exit_and_eprintln<E>(e: E) -> !
where
    E: std::fmt::Display,
{
    eprintln!("Error: {e}");
    std::process::exit(0);
}
