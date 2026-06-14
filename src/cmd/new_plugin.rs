use crate::cmd::{default_plugin_code, simple_plugin_code};
use crate::{
    error_eprintln, name_cannot_be_empty, not_cargo_workspace, plugin_added_successfully,
    plugin_created_successfully, run_cargo_command_return,
};
use colored::Colorize;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

const WORKSPACE_DEPS_DRIVERS: &[(&str, &str)] =
    &[("kovi-milky", "kovi_milky"), ("kovi-onebot", "kovi_onebot")];

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

    let plugin_path = PathBuf::from("plugins").join(&name);

    // 使用 cargo_metadata 检测工作区并读取 [workspace.dependencies]
    let workspace_root = match resolve_workspace_root() {
        Ok(root) => root,
        Err(msg) => {
            eprintln!("{msg}");
            return;
        }
    };

    // 检测 workspace.dependencies 中声明了哪些驱动
    let drivers = detect_workspace_drivers(&workspace_root);

    // 运行 cargo new --lib
    {
        let mut cargo_command = Command::new("cargo");
        cargo_command.arg("new").arg(&plugin_path).arg("--lib");

        run_cargo_command_return!(cargo_command);

        // 写入 lib.rs
        let lib_rs_path = plugin_path.join("src").join("lib.rs");
        let mut lib_rs = std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(lib_rs_path)
            .unwrap_or_else(|e| exit_and_eprintln(e));

        // 提取驱动 Rust import 名（如 "kovi_milky"）传入模板
        let driver_imports: Vec<&str> = drivers.iter().map(|&(_, import)| import).collect();

        if simple {
            let code = simple_plugin_code(&driver_imports);
            lib_rs
                .write_all(code.as_bytes())
                .unwrap_or_else(|e| exit_and_eprintln(e));
        } else {
            let code = default_plugin_code(&driver_imports);
            lib_rs
                .write_all(code.as_bytes())
                .unwrap_or_else(|e| exit_and_eprintln(e));
        }

        // 写入 Cargo.toml — kovi + 检测到的驱动 workspace 依赖
        let cargo_toml_path = plugin_path.join("Cargo.toml");
        let mut plugin_cargo = std::fs::OpenOptions::new()
            .append(true)
            .open(cargo_toml_path)
            .unwrap_or_else(|e| exit_and_eprintln(e));

        writeln!(plugin_cargo, "kovi.workspace = true").unwrap_or_else(|e| exit_and_eprintln(e));

        for &(_crate_name, _) in &drivers {
            writeln!(plugin_cargo, "{_crate_name}.workspace = true")
                .unwrap_or_else(|e| exit_and_eprintln(e));
        }

        let msg = plugin_created_successfully(&name);
        println!("\n{}", msg.green());
    }

    // 将插件添加到工作区
    let mut cargo_command = Command::new("cargo");
    cargo_command.arg("add").arg("--path").arg(plugin_path);

    run_cargo_command_return!(cargo_command);

    let msg = plugin_added_successfully(&name);
    println!("\n{}", msg.green());
}

/// 使用 cargo_metadata 确认工作区合法，并返回工作区根路径。
fn resolve_workspace_root() -> Result<PathBuf, String> {
    let metadata = cargo_metadata::MetadataCommand::new()
        .no_deps()
        .exec()
        .map_err(|_| {
            format!("{}", not_cargo_workspace()) // "This is not a cargo workspace."
        })?;

    Ok(metadata.workspace_root.into_std_path_buf())
}

/// 读取工作区根 Cargo.toml，找出 `[workspace.dependencies]` 中声明了哪些驱动。
fn detect_workspace_drivers(workspace_root: &Path) -> Vec<(&'static str, &'static str)> {
    let cargo_toml_path = workspace_root.join("Cargo.toml");
    let content = match std::fs::read_to_string(&cargo_toml_path) {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };

    WORKSPACE_DEPS_DRIVERS
        .iter()
        .filter(|(crate_name, _)| content.contains(crate_name))
        .copied()
        .collect()
}

fn exit_and_eprintln<E>(e: E) -> !
where
    E: std::fmt::Display,
{
    let msg = error_eprintln(&e.to_string());
    eprintln!("{msg}");
    std::process::exit(0);
}
