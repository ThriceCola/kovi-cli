use colored::Colorize;
use std::path::Path;
use std::process::Command;

use super::get_client;

pub fn add(name: String) {
    if name.is_empty() {
        eprintln!("Name cannot be empty");
        return;
    }

    let plugin_path_str = format!("plugins/{name}");
    let plugin_path = Path::new(&plugin_path_str);

    // 检测有没有这个插件
    match plugin_path.try_exists() {
        Ok(boo) => {
            if !boo {
                let msg = format!("try to add {} plugin from crates.io ...", plugin_path_str);
                println!("{}", msg.truecolor(202, 225, 205));
                crates_io(&name);
            } else {
                let msg = format!("add {} plugin from local ...", plugin_path_str);
                println!("{}", msg.truecolor(202, 225, 205));

                local(plugin_path, &name)
            }
        }
        Err(e) => {
            println!("{e}");
        }
    }
}

fn crates_io(name: &str) {
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
            match add_command.status() {
                Ok(status) if status.success() => {
                    println!(
                        "\n{}",
                        format!("Plugin '{}' from crates.io add successfully!", crate_name)
                            .truecolor(202, 225, 205),
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
        Err(e) => {
            if let crates_io_api::Error::NotFound(_) = e {
                eprintln!("Plugin '{}' not found on crates.io", crate_name)
            }
        }
    }
}


fn local(plugin_path: &Path, name: &str) {
    let mut cargo_command = Command::new("cargo");
    cargo_command.arg("add").arg("--path").arg(plugin_path);

    match cargo_command.status() {
        Ok(status) if status.success() => {
            println!(
                "\n{}",
                format!("Plugin '{}' form local add successfully!", name).truecolor(202, 225, 205)
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
