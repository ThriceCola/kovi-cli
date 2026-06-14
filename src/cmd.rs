use crates_io_api::SyncClient;

pub mod add;
pub mod new_kovi;
pub mod new_plugin;
pub mod update;

static KOVI_DEFAULT_VERSION: &str = "0.13";

pub fn simple_plugin_code(drivers: &[&str]) -> String {
    let driver_uses = drivers
        .iter()
        .map(|d| format!("use {d}::*;"))
        .collect::<Vec<_>>()
        .join("\n");

    if drivers.is_empty() {
        r#"use kovi::PluginBuilder as P;

#[kovi::plugin]
async fn main() {
}
"#.to_string()
    } else {
        format!(
            r#"use kovi::PluginBuilder as P;
{driver_uses}

#[kovi::plugin]
async fn main() {{
}}
"#
        )
    }
}

pub fn default_plugin_code(drivers: &[&str]) -> String {
    let driver_uses = drivers
        .iter()
        .map(|d| format!("use {d}::*;"))
        .collect::<Vec<_>>()
        .join("\n");

    if drivers.is_empty() {
        r#"use kovi::PluginBuilder as plugin;

#[kovi::plugin]
async fn main() {
    plugin::on_msg(|event| async move {
        if event.borrow_text() == Some("hi") {
            event.reply("hi")
        }
    });
}
"#.to_string()
    } else {
        format!(
            r#"use kovi::PluginBuilder as plugin;
{driver_uses}

#[kovi::plugin]
async fn main() {{
    plugin::on_msg(|event| async move {{
        if event.borrow_text() == Some("hi") {{
            event.reply("hi")
        }}
    }});
}}
"#
        )
    }
}

// ── 0.13 driver-based templates ──

/// Driver name enum used to select templates
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DriverKind {
    Milky,
    OneBot,
}

pub(crate) const DRIVER_NAMES: &[&str] = &["Milky", "OneBot (v11)"];

pub(crate) fn driver_crate_name(driver: DriverKind) -> &'static str {
    match driver {
        DriverKind::Milky => "kovi-milky",
        DriverKind::OneBot => "kovi-onebot",
    }
}

pub(crate) fn main_code_13(driver: DriverKind, has_cmd: bool) -> String {
    let (driver_crate, driver_type) = match driver {
        DriverKind::Milky => ("kovi_milky", "MilkyDriver"),
        DriverKind::OneBot => ("kovi_onebot", "OneBotDriver"),
    };

    let plugins = if has_cmd { " kovi_plugin_cmd" } else { "" };

    format!(
        r#"use kovi::tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {{
    let driver_config = {driver_crate}::load_local_conf()?;
    let driver = {driver_crate}::{driver_type}::new(driver_config);

    let bot = kovi::build_bot!(driver;{plugins});

    bot.run().await;
    Ok(())
}}
"#
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_code_13() {
        let milky_simple = main_code_13(DriverKind::Milky, false);
        println!("=== Milky simple ===\n{milky_simple}");
        assert!(milky_simple.contains("kovi_milky"));
        assert!(milky_simple.contains("MilkyDriver"));
        assert!(milky_simple.contains("tokio::main"));

        let onebot_cmd = main_code_13(DriverKind::OneBot, true);
        println!("=== OneBot with cmd ===\n{onebot_cmd}");
        assert!(onebot_cmd.contains("kovi_onebot"));
        assert!(onebot_cmd.contains("OneBotDriver"));
        assert!(onebot_cmd.contains("kovi_plugin_cmd"));
    }

    #[test]
    fn test_plugin_code() {
        // No drivers
        let simple = simple_plugin_code(&[]);
        println!("=== Simple (no driver) ===\n{simple}");
        assert!(!simple.contains("use kovi_milky"));
        assert!(!simple.contains("use kovi_onebot"));

        let default = default_plugin_code(&[]);
        println!("=== Default (no driver) ===\n{default}");
        assert!(default.contains("event.reply"));

        // Milky only
        let simple_milky = simple_plugin_code(&["kovi_milky"]);
        println!("=== Simple (milky) ===\n{simple_milky}");
        assert!(simple_milky.contains("use kovi_milky::*;"));

        // OneBot only
        let default_ob = default_plugin_code(&["kovi_onebot"]);
        println!("=== Default (onebot) ===\n{default_ob}");
        assert!(default_ob.contains("use kovi_onebot::*;"));

        // Both
        let both = default_plugin_code(&["kovi_milky", "kovi_onebot"]);
        println!("=== Default (both) ===\n{both}");
        assert!(both.contains("use kovi_milky::*;"));
        assert!(both.contains("use kovi_onebot::*;"));
    }
}

pub fn get_latest_version(name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = get_client();

    let cli_info = client.get_crate(name).unwrap();

    let max_version = cli_info.crate_data.max_version;

    Ok(max_version)
}

pub fn get_client() -> SyncClient {
    SyncClient::new(
        "kovi cli (https://github.com/thricecola/kovi-cli)",
        std::time::Duration::from_millis(1000),
    )
    .unwrap()
}

#[test]
fn latest() {
    let a = get_latest_version("kovi-cli");
    println!("{:?}", a)
}

#[test]
fn crates_io_some() {
    let client = SyncClient::new(
        "kovi cli (https://github.com/thricecola/kovi-cli)",
        std::time::Duration::from_millis(1000),
    )
    .unwrap();

    let a = client.get_crate("kovi").unwrap();
    println!("{:?}", a);
}
