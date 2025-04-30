use crate::cmd::DEFAULT_PLUGIN_CODE;
use crate::cmd::SIMPLE_PLUGIN_CODE;
use crate::error_eprintln;
use crate::name_cannot_be_empty;
use crate::not_cargo_workspace;
use crate::plugin_added_successfully;
use crate::plugin_created_successfully;
use crate::run_cargo_command_return;
use colored::Colorize;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

pub fn new_plugin(name: String, simple: bool, prefix: bool) {
    if name.is_empty() {
        let msg = name_cannot_be_empty();
        eprintln!("{msg}");
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
            let msg = not_cargo_workspace();
            eprintln!("{msg}");
            return;
        }
    }

    // 运行cargo new --lib命令
    {
        let mut cargo_command = Command::new("cargo");
        cargo_command.arg("new").arg(&plugin_path).arg("--lib");

        run_cargo_command_return!(cargo_command);

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

        writeln!(cargo_toml, "kovi.workspace = true").unwrap_or_else(|e| exit_and_eprintln(e));

        let msg = plugin_created_successfully(&name);
        let msg = msg.green();
        println!("\n{msg}");
    }

    let mut cargo_command = Command::new("cargo");
    cargo_command.arg("add").arg("--path").arg(plugin_path);

    run_cargo_command_return!(cargo_command);

    let msg = plugin_added_successfully(&name);
    let msg = msg.green();
    println!("\n{msg}");
}

fn exit_and_eprintln<E>(e: E) -> !
where
    E: std::fmt::Display,
{
    let msg = error_eprintln(&e.to_string());
    eprintln!("{msg}");
    std::process::exit(0);
}
