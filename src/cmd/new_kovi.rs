use crate::cmd::{DEFAULT_MAIN_CODE, KOVI_DEFAULT_VERSION, get_latest_version};
use crate::{
    cargo_exited_with_status, failed_to_execute_cargo, kovi_workspace_created_successfully,
    new_kovi_version_error, next_steps_for_kovi_workspace, you_can,
};
use colored::Colorize;
use std::io::Write;
use std::path::Path;
use std::process::Command;

pub fn new_kovi(name: String, version: Option<String>) {
    let mut cargo_command = Command::new("cargo");
    cargo_command.arg("new").arg(&name);

    match cargo_command.status() {
        Ok(status) if status.success() => {
            let path = format!("./{}", name);
            let path = Path::new(&path);

            let cargo_path = path.join("Cargo.toml");

            //给Cargo.toml加入Kovi依赖
            let mut cargo_toml = std::fs::OpenOptions::new()
                .append(true)
                .open(cargo_path)
                .expect("Failed to open Cargo.toml");

            let version = match version {
                Some(v) => v,
                None => match get_latest_version("kovi") {
                    Ok(v) => v,
                    Err(e) => {
                        let v = KOVI_DEFAULT_VERSION.to_string();
                        //报错获取失败，使用默认版本
                        let msg = new_kovi_version_error(&v, &e.to_string());
                        eprintln!("{msg}");
                        v
                    }
                },
            };

            writeln!(cargo_toml, "kovi = \"{}\"", version).expect("Failed to write to Cargo.toml");

            // writeln!(cargo_toml, "kovi = {{ path = \"../../kovi\" }}")
            //     .expect("Failed to write to Cargo.toml");

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
                .expect("Failed to open lib.rs");

            main_rs
                .write_all(DEFAULT_MAIN_CODE.as_bytes())
                .expect("Failed to write to lib.rs");

            #[allow(clippy::format_in_format_args)]
            {
                let kovi_workspace_created_successfully =
                    kovi_workspace_created_successfully(&name).truecolor(202, 225, 205);
                let you_can = you_can();
                let next_steps_for_kovi_bot =
                    next_steps_for_kovi_workspace(&name).truecolor(202, 225, 205);

                println!(
                    "\n{kovi_workspace_created_successfully}\n{you_can}\n{next_steps_for_kovi_bot}",
                );
            }
        }
        Ok(status) => {
            let status = format!("{}", status);
            let msg = cargo_exited_with_status(&status);
            eprintln!("{msg}");
        }
        Err(e) => {
            let e = e.to_string();
            let msg = failed_to_execute_cargo(&e);
            eprintln!("{msg}");
        }
    }
}
