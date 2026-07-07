use crate::cmd::{
    DRIVER_NAMES, DriverKind, KOVI_DEFAULT_VERSION, driver_crate_name, get_latest_version,
    main_code_13,
};

use crate::{
    are_you_want_to_add_message_command_plugins, kovi_workspace_created_successfully,
    new_kovi_version_error, next_steps_for_kovi_workspace, run_cargo_command_return,
    what_is_the_name_of_the_kovi_workspace, which_driver_to_use, you_can,
    you_specified_this_name_for_kovi_workspace,
};
use colored::Colorize;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Input, Select};
use std::path::Path;
use std::process::Command;

const DEFAULT_KOVI_NAME: &str = "kovi-bot";

pub fn new_kovi(name: Option<String>, simple: bool, driver_arg: Option<String>, cmd_arg: bool) {
    let name = resolve_name(name, simple);
    let (driver, add_cmd) = resolve_driver_and_cmd(simple, driver_arg, cmd_arg, &name);
    cargo_new_kovi(&name, driver, add_cmd);
}

fn resolve_name(name: Option<String>, simple: bool) -> String {
    if simple {
        name.unwrap_or_else(|| {
            let msg = crate::simple_handler_name_not_specified();
            println!("{} {}", msg.yellow(), DEFAULT_KOVI_NAME);
            DEFAULT_KOVI_NAME.to_string()
        })
    } else {
        match name {
            Some(name) => {
                let msg = you_specified_this_name_for_kovi_workspace();
                println!("{msg} {}", name.green());
                name
            }
            None => {
                let msg = what_is_the_name_of_the_kovi_workspace();
                Input::with_theme(&ColorfulTheme::default())
                    .with_prompt(msg.to_string())
                    .default(DEFAULT_KOVI_NAME.to_string())
                    .interact_text()
                    .unwrap()
            }
        }
    }
}

/// Resolves driver and cmd-plugin flags. Prompts interactively when not specified via CLI.
fn resolve_driver_and_cmd(
    simple: bool,
    driver_arg: Option<String>,
    cmd_arg: bool,
    _name: &str,
) -> (DriverKind, bool) {
    let driver = if simple {
        match driver_arg.as_deref() {
            Some("onebot") => DriverKind::OneBot,
            _ => DriverKind::Milky,
        }
    } else {
        match driver_arg.as_deref() {
            Some("milky") => {
                println!("{} milky", which_driver_to_use().green());
                DriverKind::Milky
            }
            Some("onebot") => {
                println!("{} onebot", which_driver_to_use().green());
                DriverKind::OneBot
            }
            None => {
                let msg = which_driver_to_use();
                let select = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt(msg)
                    .items(DRIVER_NAMES)
                    .default(0)
                    .interact()
                    .unwrap();

                match select {
                    0 => DriverKind::Milky,
                    1 => DriverKind::OneBot,
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    };

    let add_cmd = if cmd_arg {
        true
    } else if simple {
        false
    } else {
        let msg = are_you_want_to_add_message_command_plugins();
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

    (driver, add_cmd)
}

pub fn cargo_new_kovi(name: &str, driver: DriverKind, is_add_cmd_plugin: bool) {
    let mut cargo_command = Command::new("cargo");
    cargo_command.arg("new").arg(name);

    run_cargo_command_return!(cargo_command);

    let path = format!("./{}", name);
    let path = Path::new(&path);

    let cargo_path = path.join("Cargo.toml");

    let kovi_version = match get_latest_version("kovi") {
        Ok(v) => v,
        Err(e) => {
            let v = KOVI_DEFAULT_VERSION.to_string();
            //报错获取失败，使用默认版本
            let msg = new_kovi_version_error(&v, &e.to_string());
            eprintln!("{msg}");
            v
        }
    };

    let driver_crate = driver_crate_name(driver);

    let driver_version = match get_latest_version(driver_crate) {
        Ok(v) => v,
        Err(_) => kovi_version.clone(),
    };

    // 读取已有 Cargo.toml，追加依赖和 workspace
    let mut cargo_content =
        std::fs::read_to_string(&cargo_path).expect("Failed to read Cargo.toml");

    // 用 workspace = true 引用依赖，版本号只写在 [workspace.dependencies] 中
    cargo_content.push_str(&format!(
        "\nkovi.workspace = true\n{driver_crate}.workspace = true\n\n[workspace]\n\n[workspace.dependencies]\nkovi = \"{kovi_version}\"\n{driver_crate} = \"{driver_version}\"\n"
    ));

    std::fs::write(&cargo_path, &cargo_content).expect("Failed to write Cargo.toml");

    // 写入 src/main.rs（0.13 异步驱动模式）
    let main_path = path.join("src/main.rs");
    let main_code = main_code_13(driver, is_add_cmd_plugin);
    std::fs::write(&main_path, main_code).expect("Failed to write to main.rs");

    if is_add_cmd_plugin {
        let driver_feature = match driver {
            DriverKind::Milky => "milky",
            DriverKind::OneBot => "onebot",
        };

        let mut cargo_command = Command::new("cargo");
        cargo_command.current_dir(path);
        cargo_command
            .arg("add")
            .arg("kovi-plugin-cmd")
            .arg("--features")
            .arg(driver_feature);

        run_cargo_command_return!(cargo_command);
    }

    let kovi_workspace_created_successfully = kovi_workspace_created_successfully(name).green();
    let you_can = you_can();
    let next_steps_for_kovi_bot = next_steps_for_kovi_workspace(name);
    println!("\n{kovi_workspace_created_successfully}\n{you_can}\n{next_steps_for_kovi_bot}",);
}
