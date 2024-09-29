use crates_io_api::SyncClient;

pub mod add;
pub mod new_kovi;
pub mod new_plugin;
pub mod update;

static KOVI_DEFAULT_VERSION: &str = "0.8.0";

static SIMPLE_PLUGIN_CODE: &str = r#"use kovi::PluginBuilder as p;

#[kovi::plugin]
async fn main() {
}
"#;

static DEFAULT_PLUGIN_CODE: &str = r#"use kovi::PluginBuilder as plugin;

#[kovi::plugin]
async fn main() {
    plugin::on_msg(|event| async move {
        if event.borrow_text() == Some("hi") {
            event.reply("hi")
        }
    });
}
"#;

static DEFAULT_MAIN_CODE: &str = r#"use kovi::build_bot;

fn main() {
    build_bot!().run();
}
"#;

pub fn get_latest_version(name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = get_client();

    let cli_info = client.get_crate(name).unwrap();

    let max_version = cli_info.crate_data.max_version;

    Ok(max_version)
}

pub fn get_client() -> SyncClient {
    SyncClient::new(
        "kovi cli (https://github.com/threkork/kovi-cli)",
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
        "kovi cli (https://github.com/threkork/kovi-cli)",
        std::time::Duration::from_millis(1000),
    )
    .unwrap();

    let a = client.get_crate("kovi-plugin").unwrap();
    println!("{:?}", a);
}
