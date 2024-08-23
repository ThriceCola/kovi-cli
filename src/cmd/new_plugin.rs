use crate::cmd::DEFAULT_PLUGIN_CODE;
use colored::Colorize;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::process::Command;

pub fn new_plugin(name: String) {
    if name.is_empty() {
        eprintln!("Name cannot be empty");
        return;
    }

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


    let path = format!("plugins/{name}");
    let path = Path::new(&path);

    let mut cargo_command = Command::new("cargo");
    cargo_command.arg("new").arg(path).arg("--lib");

    match cargo_command.status() {
        Ok(status) if status.success() => {
            //使用Toml库，获取根目录里面的Cargo.toml的kovi的版本号
            let kovi_dependency = {
                let root_cargo_toml =
                    std::fs::read_to_string("Cargo.toml").unwrap_or_else(|e| exit_and_eprintln(e));
                let parsed_toml: toml::Value =
                    toml::from_str(&root_cargo_toml).unwrap_or_else(|e| exit_and_eprintln(e));

                parsed_toml
                    .get("dependencies")
                    .and_then(|deps| deps.get("kovi"))
                    .expect("kovi dependency not found in root Cargo.toml")
                    .clone()
            };

            let cargo_toml_path = path.join("Cargo.toml");

            let mut cargo_toml = std::fs::OpenOptions::new()
                .append(true)
                .open(cargo_toml_path)
                .unwrap_or_else(|e| exit_and_eprintln(e));

            match kovi_dependency {
                toml::Value::String(version) => {
                    writeln!(cargo_toml, "kovi = \"{}\"", version)
                        .unwrap_or_else(|e| exit_and_eprintln(e));
                }
                toml::Value::Table(table) => {
                    write!(cargo_toml, "kovi = {{ ").unwrap_or_else(|e| exit_and_eprintln(e));
                    let mut iter = table.iter().peekable();
                    while let Some((key, value)) = iter.next() {
                        if key == "path" {
                            write!(cargo_toml, "path = ../../{}", value)
                                .unwrap_or_else(|e| exit_and_eprintln(e));
                        } else {
                            write!(cargo_toml, "{} = {}", key, value)
                                .unwrap_or_else(|e| exit_and_eprintln(e));
                        }

                        if iter.peek().is_some() {
                            write!(cargo_toml, ", ").unwrap_or_else(|e| exit_and_eprintln(e));
                        } else {
                            write!(cargo_toml, " ").unwrap_or_else(|e| exit_and_eprintln(e));
                        }
                    }
                    writeln!(cargo_toml, "}}").unwrap_or_else(|e| exit_and_eprintln(e));
                }
                _ => panic!("Unexpected format for kovi dependency"),
            }

            // 清空src/lib.rs，然后传入默认的代码
            let lib_path = path.join("src/lib.rs");
            let mut lib_rs = std::fs::OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(lib_path)
                .expect("Failed to open lib.rs");


            lib_rs
                .write_all(DEFAULT_PLUGIN_CODE.as_bytes())
                .expect("Failed to write to lib.rs");

            println!(
                "\n{}",
                format!("Plugin '{}' created successfully!", name).truecolor(202, 225, 205),
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
    eprintln!("Checking if in correct workspace...");
    std::process::exit(0);
}
