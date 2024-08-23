pub mod add;
pub mod new_kovi;
pub mod new_plugin;
pub mod update;

static DEFAULT_PLUGIN_CODE: &str = r#"use kovi::PluginBuilder;

#[kovi::plugin]
pub fn main(mut plugin: PluginBuilder) {
    plugin.on_msg(move |event| {
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
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    struct CrateResponse {
        #[serde(rename = "crate")]
        crate_: CrateInfo,
    }

    #[derive(Deserialize, Debug)]
    struct CrateInfo {
        max_version: String,
    }

    let url = format!("https://crates.io/api/v1/crates/{name}");

    let client = reqwest::blocking::Client::new();
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::USER_AGENT,
        reqwest::header::HeaderValue::from_static(
            "kovi cli (https://github.com/threkork/kovi-cli)",
        ),
    );

    let response = client.get(&url).headers(headers).send()?.text()?;
    let response: CrateResponse = serde_json::from_str(&response)?;
    Ok(response.crate_.max_version)
}


#[test]
fn name() {
    let a = get_latest_version("kovi-cli");
    println!("{:?}", a)
}
