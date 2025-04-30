use colored::Colorize;
use std::path::Path;
use std::process::Command;

use crate::{
    add_local_plugin, name_cannot_be_empty, plugin_added_from_crates_io_successfully,
    plugin_directory_does_not_exist, plugin_local_added_successfully,
    plugin_not_found_on_crates_io, run_cargo_command_return, to_something_cannot_be_empty,
    try_add_plugin_from_crates_io, try_add_plugin_from_crates_io_to_local_plugin,
};

use super::get_client;

pub fn add(name: String) {
    if name.is_empty() {
        let msg = name_cannot_be_empty();
        eprintln!("{msg}");
        return;
    }

    let plugin_path_str = format!("plugins/{name}");
    let plugin_path = Path::new(&plugin_path_str);

    // 检测有没有这个插件
    match plugin_path.try_exists() {
        Ok(boo) => {
            if !boo {
                let msg = try_add_plugin_from_crates_io(&name).green();
                println!("{msg}");
                crates_io(&name, None);
            } else {
                let msg = add_local_plugin(&plugin_path_str).green();
                println!("{msg}");

                local(plugin_path, &name)
            }
        }
        Err(e) => {
            println!("{e}");
        }
    }
}

pub fn add_to(name: String, package: String) {
    if name.is_empty() {
        let msg = name_cannot_be_empty();
        eprintln!("{msg}");
        return;
    }

    if package.is_empty() {
        let msg = to_something_cannot_be_empty();
        eprintln!("{msg}");
        return;
    }

    let plugin_path_str = format!("plugins/{package}");
    let plugin_path = Path::new(&plugin_path_str);

    // 检测有没有这个插件
    match plugin_path.try_exists() {
        Ok(boo) => {
            if boo {
                let msg = try_add_plugin_from_crates_io_to_local_plugin(&name, &package).green();
                println!("{msg}");
                crates_io(&name, Some(&package));
            } else {
                let msg = plugin_directory_does_not_exist(&plugin_path_str);
                eprintln!("{msg}");
            }
        }
        Err(e) => {
            println!("{e}");
        }
    }
}

fn crates_io(name: &str, package: Option<&str>) {
    //检测name之前是否包含 "kovi-plugin-"
    let crate_name = if name.starts_with("kovi-plugin-") {
        name.to_string()
    } else {
        format!("kovi-plugin-{name}")
    };

    let client = get_client();
    match client.get_crate(&crate_name) {
        Ok(_) => {
            let mut add_command = Command::new("cargo");
            add_command.arg("add").arg(&crate_name);

            if let Some(package) = package {
                add_command.arg("--package").arg(package);
            }

            run_cargo_command_return!(add_command);

            let msg = plugin_added_from_crates_io_successfully(&crate_name).green();
            println!("\n{msg}");
        }
        Err(e) => {
            if let crates_io_api::Error::NotFound(_) = e {
                let msg = plugin_not_found_on_crates_io(&crate_name);
                eprintln!("{msg}");
            }
        }
    }
}

fn local(plugin_path: &Path, name: &str) {
    let mut cargo_command = Command::new("cargo");
    cargo_command.arg("add").arg("--path").arg(plugin_path);

    run_cargo_command_return!(cargo_command);

    let msg = plugin_local_added_successfully(name).green();
    println!("\n{msg}");
}
