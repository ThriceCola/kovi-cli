use crate::cmd::{
    DEFAULT_MAIN_CODE, DEFAULT_MAIN_CODE_HAS_CMD, KOVI_DEFAULT_VERSION, get_latest_version,
};

use crate::{
    are_you_want_to_add_message_command_plugins, kovi_workspace_created_successfully,
    new_kovi_version_error, next_steps_for_kovi_workspace, run_cargo_command_return,
    simple_handler_name_not_specified, what_is_the_name_of_the_kovi_workspace, you_can,
    you_specified_this_name_for_kovi_workspace,
};
use colored::Colorize;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Input, Select};
use std::io::Write;
use std::path::Path;
use std::process::Command;

const DEFAULT_KOVI_NAME: &str = "kovi-bot";

pub fn new_kovi(name: Option<String>, simple: bool) {
    if simple {
        simple_handler(name)
    } else {
        guided(name)
    }
}

fn guided(name: Option<String>) {
    let name = match name {
        Some(name) => {
            let msg = you_specified_this_name_for_kovi_workspace();
            println!("{msg} {}", name.green());
            name
        }
        None => {
            let msg = what_is_the_name_of_the_kovi_workspace();
            let msg = format!("{msg}");
            let name: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt(msg)
                .default(DEFAULT_KOVI_NAME.to_string())
                .interact_text()
                .unwrap();
            name
        }
    };

    // 是否查看更多可选选项
    let is_add_cmd_plugin: bool = {
        let msg = are_you_want_to_add_message_command_plugins();
        let msg = format!("{msg}");
        let items = ["Yes", "No"];
        let select = Select::with_theme(&ColorfulTheme::default())
            .with_prompt(msg)
            .items(&items)
            .default(0)
            .interact()
            .unwrap();

        match select {
            0 => true,
            1 => false,
            _ => unreachable!(),
        }
    };

    cargo_new_kovi(&name, is_add_cmd_plugin);
}

fn simple_handler(name: Option<String>) {
    let name = name.unwrap_or_else(|| {
        let name = DEFAULT_KOVI_NAME.to_string();
        let msg = simple_handler_name_not_specified().yellow();
        println!("{msg} {name}");
        name
    });

    cargo_new_kovi(&name, false);
}

fn cargo_new_kovi(name: &str, is_add_cmd_plugin: bool) {
    let mut cargo_command = Command::new("cargo");
    cargo_command.arg("new").arg(name);

    run_cargo_command_return!(cargo_command);

    let path = format!("./{}", name);
    let path = Path::new(&path);

    let cargo_path = path.join("Cargo.toml");

    //给Cargo.toml加入Kovi依赖
    let mut cargo_toml = std::fs::OpenOptions::new()
        .append(true)
        .open(cargo_path)
        .expect("Failed to open Cargo.toml");

    let version = match get_latest_version("kovi") {
        Ok(v) => v,
        Err(e) => {
            let v = KOVI_DEFAULT_VERSION.to_string();
            //报错获取失败，使用默认版本
            let msg = new_kovi_version_error(&v, &e.to_string());
            eprintln!("{msg}");
            v
        }
    };

    writeln!(cargo_toml, "kovi = \"{}\"", version).expect("Failed to write to Cargo.toml");

    writeln!(
        cargo_toml,
        "\n[workspace]\n\n[workspace.dependencies]\nkovi = \"{version}\""
    )
    .expect("Failed to write to Cargo.toml");

    // 清空src/main.rs，然后传入默认的代码
    let main_path = path.join("src/main.rs");
    let mut main_rs = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(main_path)
        .expect("Failed to open main.rs");

    if is_add_cmd_plugin {
        main_rs
            .write_all(DEFAULT_MAIN_CODE_HAS_CMD.as_bytes())
            .expect("Failed to write to main.rs");
    } else {
        main_rs
            .write_all(DEFAULT_MAIN_CODE.as_bytes())
            .expect("Failed to write to main.rs");
    }

    drop(cargo_toml);
    drop(main_rs);

    if is_add_cmd_plugin {
        let mut cargo_command = Command::new("cargo");
        cargo_command.current_dir(format!("./{}", name));
        cargo_command.arg("add").arg("kovi-plugin-cmd");

        run_cargo_command_return!(cargo_command);
    }

    let kovi_workspace_created_successfully = kovi_workspace_created_successfully(name).green();
    let you_can = you_can();
    let next_steps_for_kovi_bot = next_steps_for_kovi_workspace(name);
    println!("\n{kovi_workspace_created_successfully}\n{you_can}\n{next_steps_for_kovi_bot}",);
}
