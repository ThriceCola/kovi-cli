#[allow(dead_code)]
use anyhow::{Result, anyhow};
use fluent::concurrent::FluentBundle;
use fluent::{FluentArgs, FluentResource};
use std::{borrow::Cow, env, sync::LazyLock};
use unic_langid::langid;

pub(crate) fn update_get_latest_version_err(err: &str) -> Cow<'static, str> {
    LOCALE
        .message(
            "update-get-latest-version-err",
            Some(&LocaleArgs::new().set("error", err)),
        )
        .unwrap()
}

pub(crate) fn update_using_the_latest_version(version: &str) -> Cow<'static, str> {
    LOCALE
        .message(
            "update-using-the-latest-version",
            Some(&LocaleArgs::new().set("version", version)),
        )
        .unwrap()
}

pub(crate) fn update_has_new_version() -> Cow<'static, str> {
    LOCALE.message("update-has-new-version", None).unwrap()
}

pub(crate) fn cli_update_successful() -> Cow<'static, str> {
    LOCALE.message("cli-update-successful", None).unwrap()
}

pub(crate) fn proceed_with_the_installation() -> Cow<'static, str> {
    LOCALE
        .message("proceed-with-the-installation", None)
        .unwrap()
}

pub(crate) fn cargo_exited_with_status(status: &str) -> Cow<'static, str> {
    LOCALE
        .message(
            "cargo-exited-with-status",
            Some(&LocaleArgs::new().set("status", status)),
        )
        .unwrap()
}

pub(crate) fn failed_to_execute_cargo(error: &str) -> Cow<'static, str> {
    LOCALE
        .message(
            "failed-to-execute-cargo",
            Some(&LocaleArgs::new().set("error", error)),
        )
        .unwrap()
}

#[allow(dead_code)]
pub(crate) fn update_windows_manually_to_use() -> Cow<'static, str> {
    LOCALE
        .message("update-windows-manually-to-use", None)
        .unwrap()
}

pub(crate) fn name_cannot_be_empty() -> Cow<'static, str> {
    LOCALE.message("name-cannot-be-empty", None).unwrap()
}

pub(crate) fn not_cargo_workspace() -> Cow<'static, str> {
    LOCALE.message("not-cargo-workspace", None).unwrap()
}

pub(crate) fn plugin_created_successfully(name: &str) -> Cow<'static, str> {
    LOCALE
        .message(
            "plugin-created-successfully",
            Some(&LocaleArgs::new().set("name", name)),
        )
        .unwrap()
}

pub(crate) fn plugin_added_successfully(name: &str) -> Cow<'static, str> {
    LOCALE
        .message(
            "plugin-added-successfully",
            Some(&LocaleArgs::new().set("name", name)),
        )
        .unwrap()
}

pub(crate) fn error_eprintln(error: &str) -> Cow<'static, str> {
    LOCALE
        .message(
            "error-eprintln",
            Some(&LocaleArgs::new().set("error", error)),
        )
        .unwrap()
}

pub(crate) fn new_kovi_version_error(version: &str, err: &str) -> Cow<'static, str> {
    LOCALE
        .message(
            "new-kovi-version-error",
            Some(&LocaleArgs::new().set("error", err).set("version", version)),
        )
        .unwrap()
}

pub(crate) fn kovi_workspace_created_successfully(name: &str) -> Cow<'static, str> {
    LOCALE
        .message(
            "kovi-workspace-created-successfully",
            Some(&LocaleArgs::new().set("name", name)),
        )
        .unwrap()
}

pub(crate) fn you_can() -> Cow<'static, str> {
    LOCALE.message("you-can", None).unwrap()
}

pub(crate) fn next_steps_for_kovi_workspace(name: &str) -> Cow<'static, str> {
    LOCALE
        .message(
            "next-steps-for-kovi-workspace",
            Some(&LocaleArgs::new().set("name", name)),
        )
        .unwrap()
}

pub(crate) fn try_add_plugin_from_crates_io(name: &str) -> Cow<'static, str> {
    LOCALE
        .message(
            "try-add-plugin-from-crates-io",
            Some(&LocaleArgs::new().set("name", name)),
        )
        .unwrap()
}

pub(crate) fn add_local_plugin(name: &str) -> Cow<'static, str> {
    LOCALE
        .message(
            "add-local-plugin",
            Some(&LocaleArgs::new().set("name", name)),
        )
        .unwrap()
}

pub(crate) fn to_something_cannot_be_empty() -> Cow<'static, str> {
    LOCALE
        .message("to-something-cannot-be-empty", None)
        .unwrap()
}

pub(crate) fn try_add_plugin_from_crates_io_to_local_plugin(
    name: &str,
    package: &str,
) -> Cow<'static, str> {
    LOCALE
        .message(
            "try-add-plugin-from-crates-io-to-local-plugin",
            Some(&LocaleArgs::new().set("name", name).set("package", package)),
        )
        .unwrap()
}

pub(crate) fn plugin_directory_does_not_exist(directory: &str) -> Cow<'static, str> {
    LOCALE
        .message(
            "plugin-directory-does-not-exist",
            Some(&LocaleArgs::new().set("directory", directory)),
        )
        .unwrap()
}

pub(crate) fn plugin_added_from_crates_io_successfully(name: &str) -> Cow<'static, str> {
    LOCALE
        .message(
            "plugin-added-from-crates-io-successfully",
            Some(&LocaleArgs::new().set("name", name)),
        )
        .unwrap()
}

pub(crate) fn plugin_not_found_on_crates_io(name: &str) -> Cow<'static, str> {
    LOCALE
        .message(
            "plugin-not-found-on-crates-io",
            Some(&LocaleArgs::new().set("name", name)),
        )
        .unwrap()
}

pub(crate) fn plugin_local_added_successfully(name: &str) -> Cow<'static, str> {
    LOCALE
        .message(
            "plugin-local-added-successfully",
            Some(&LocaleArgs::new().set("name", name)),
        )
        .unwrap()
}

pub(crate) fn simple_handler_name_not_specified() -> Cow<'static, str> {
    LOCALE
        .message("simple-handler-name-not-specified", None)
        .unwrap()
}

pub(crate) fn you_specified_this_name_for_kovi_workspace() -> Cow<'static, str> {
    LOCALE
        .message("you-specified-this-name-for-kovi-workspace", None)
        .unwrap()
}

pub(crate) fn what_is_the_name_of_the_kovi_workspace() -> Cow<'static, str> {
    LOCALE
        .message("what-is-the-name-of-the-kovi-workspace", None)
        .unwrap()
}

pub(crate) fn are_you_want_to_add_message_command_plugins() -> Cow<'static, str> {
    LOCALE
        .message("are-you-want-to-add-message-command-plugins", None)
        .unwrap()
}

static LOCALES_EN_US: &str = include_str!("../locales/en-US.ftl");

static LOCALES_ZH_CN: &str = include_str!("../locales/zh-CN.ftl");

pub(crate) static LOCALE: LazyLock<LocaleContext> = LazyLock::new(LocaleContext::init);

pub(crate) struct LocaleContext {
    ftl: FluentBundle<FluentResource>,
}

#[derive(Debug)]
pub(crate) struct LocaleArgs<'args> {
    ftl: FluentArgs<'args>,
}
impl<'args> LocaleArgs<'args> {
    pub fn new() -> Self {
        let args = FluentArgs::new();
        Self { ftl: args }
    }

    pub fn set<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<Cow<'args, str>>,
        V: Into<fluent::FluentValue<'args>>,
    {
        self.ftl.set(key, value);
        self
    }
}
impl<'args> From<FluentArgs<'args>> for LocaleArgs<'args> {
    fn from(args: FluentArgs<'args>) -> LocaleArgs<'args> {
        Self { ftl: args }
    }
}

impl LocaleContext {
    fn init() -> Self {
        Self { ftl: init_fluent() }
    }

    pub(crate) fn message<'bundle>(
        &'bundle self,
        id: &str,
        args: Option<&LocaleArgs<'_>>,
    ) -> Result<Cow<'bundle, str>> {
        let msg = self
            .ftl
            .get_message(id)
            .ok_or(anyhow!("Message doesn't exist."))?;

        let pattern = msg.value().expect("Missing Value.");

        let fluent_args: Option<&'_ FluentArgs<'_>> = args.map(|locale| &locale.ftl);

        let mut errors = vec![];
        let message = self.ftl.format_pattern(pattern, fluent_args, &mut errors);

        if !errors.is_empty() {
            return Err(anyhow!("Formatting errors: {:?}", errors));
        }

        Ok(message)
    }
}

fn init_fluent() -> FluentBundle<FluentResource> {
    let lang = get_system_locale();

    let (land_ftl, lang_id) = match lang.as_str() {
        r if r.starts_with("zh-CN") || r.starts_with("zh_CN") => {
            (LOCALES_ZH_CN.to_string(), langid!("zh-CN"))
        }
        _ => (LOCALES_EN_US.to_string(), langid!("en-US")),
    };

    let res = FluentResource::try_new(land_ftl).expect("Failed to parse an FTL string.");

    let mut bundle = FluentBundle::new_concurrent(vec![lang_id, langid!("en-US")]);

    bundle.add_resource(res).unwrap();

    bundle
}

fn get_system_locale() -> String {
    // 首先尝试标准的 Unix 环境变量
    if let Ok(locale) = env::var("LC_ALL") {
        return locale;
    }

    if let Ok(locale) = env::var("LANG") {
        return locale;
    }

    // Windows 系统特有的处理
    #[cfg(target_os = "windows")]
    {
        if let Some(locale) = get_windows_locale() {
            return locale;
        }
    }

    "en-US".to_string()
}

#[cfg(target_os = "windows")]
fn get_windows_locale() -> Option<String> {
    use std::process::Command;

    // 尝试使用 PowerShell 获取系统语言
    if let Ok(output) = Command::new("powershell")
        .args(&[
            "-NoProfile",
            "-Command",
            "Get-Culture | Select-Object -ExpandProperty Name",
        ])
        .output()
    {
        if output.status.success() {
            let locale_str = String::from_utf8_lossy(&output.stdout);
            let locale_str = locale_str.trim();
            if !locale_str.is_empty() {
                return Some(locale_str.to_string());
            }
        }
    }

    // 如果 PowerShell 失败，尝试使用 wmic
    if let Ok(output) = Command::new("wmic")
        .args(&["os", "get", "locale", "/value"])
        .output()
    {
        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            for line in output_str.lines() {
                if line.starts_with("Locale=") {
                    let locale_code = line.strip_prefix("Locale=").unwrap_or("").trim();
                    if !locale_code.is_empty() {
                        // 将 Windows 语言代码转换为标准格式
                        return Some(convert_windows_locale_code(locale_code));
                    }
                }
            }
        }
    }

    None
}

#[cfg(target_os = "windows")]
fn convert_windows_locale_code(code: &str) -> String {
    // 将常见的 Windows 语言代码转换为标准格式
    match code {
        "0804" => "zh-CN".to_string(), // 中文(简体)
        "0404" => "zh-CN".to_string(), // 中文(繁体)
        _ => {
            // 如果是未知代码，返回默认值
            "en-US".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_fluent() {
        init_fluent();
    }

    #[test]
    fn test_print_local() {
        // 1. update_get_latest_version_err
        println!(
            "{}",
            update_get_latest_version_err("Failed to fetch latest version")
        );

        // 2. update_using_the_latest_version
        println!("{}", update_using_the_latest_version("v1.0.0"));

        // 3. update_has_new_version
        println!("{}", update_has_new_version());

        // 4. cli_update_successful
        println!("{}", cli_update_successful());

        // 5. proceed_with_the_installation
        println!("{}", proceed_with_the_installation());

        // 6. cargo_exited_with_status
        println!("{}", cargo_exited_with_status("127"));

        // 7. failed_to_execute_cargo
        println!("{}", failed_to_execute_cargo("Cargo command not found"));

        // 8. update_windows_manually_to_use
        println!("{}", update_windows_manually_to_use());

        // 9. name_cannot_be_empty
        println!("{}", name_cannot_be_empty());

        // 10. not_cargo_workspace
        println!("{}", not_cargo_workspace());

        // 11. plugin_created_successfully
        println!("{}", plugin_created_successfully("my-plugin"));

        // 12. plugin_added_successfully
        println!("{}", plugin_added_successfully("my-plugin"));

        // 13. error_eprintln
        println!("{}", error_eprintln("An unexpected error occurred"));

        // 14. new_kovi_version_error
        println!(
            "{}",
            new_kovi_version_error("v2.0.0", "Failed to install new version")
        );

        // 15. kovi_workspace_created_successfully
        println!("{}", kovi_workspace_created_successfully("my-workspace"));

        // 16. you_can
        println!("{}", you_can());

        // 17. next_steps_for_kovi_workspace
        println!("{}", next_steps_for_kovi_workspace("my-workspace"));

        // 18. try_add_plugin_from_crates_io
        println!("{}", try_add_plugin_from_crates_io("my-plugin"));

        // 19. add_local_plugin
        println!("{}", add_local_plugin("my-local-plugin"));

        // 20. to_something_cannot_be_empty
        println!("{}", to_something_cannot_be_empty());

        // 21. try_add_plugin_from_crates_io_to_local_plugin
        println!(
            "{}",
            try_add_plugin_from_crates_io_to_local_plugin("my-plugin", "my-package")
        );

        // 22. plugin_directory_does_not_exist
        println!("{}", plugin_directory_does_not_exist("/path/to/plugin"));

        // 23. plugin_added_from_crates_io_successfully
        println!("{}", plugin_added_from_crates_io_successfully("my-plugin"));

        // 24. plugin_not_found_on_crates_io
        println!("{}", plugin_not_found_on_crates_io("missing-plugin"));

        // 25. plugin_local_added_successfully
        println!("{}", plugin_local_added_successfully("my-local-plugin"));

        // 26. simple_handler_name_not_specified
        println!("{}", simple_handler_name_not_specified());

        // 27. you_specified_this_name_for_kovi_workspace
        println!("{}", you_specified_this_name_for_kovi_workspace());

        // 28. what_is_the_name_of_the_kovi_workspace
        println!("{}", what_is_the_name_of_the_kovi_workspace());

        // 29. are_you_want_to_add_message_command_plugins
        println!("{}", are_you_want_to_add_message_command_plugins());
    }

    #[test]
    fn test_get_system_locale() {
        // 测试基本功能，这应该不会崩溃
        let locale = get_system_locale();
        assert!(!locale.is_empty());
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn test_convert_windows_locale_code() {
        // 测试常见的 Windows 语言代码转换
        assert_eq!(convert_windows_locale_code("0804"), "zh-CN");
        assert_eq!(convert_windows_locale_code("0404"), "zh-TW");
        assert_eq!(convert_windows_locale_code("0409"), "en-US");
        assert_eq!(convert_windows_locale_code("0809"), "en-GB");
        assert_eq!(convert_windows_locale_code("0411"), "ja-JP");
        assert_eq!(convert_windows_locale_code("0412"), "ko-KR");
        assert_eq!(convert_windows_locale_code("040c"), "fr-FR");
        assert_eq!(convert_windows_locale_code("0407"), "de-DE");
        assert_eq!(convert_windows_locale_code("0410"), "it-IT");
        assert_eq!(convert_windows_locale_code("0c0a"), "es-ES");
        assert_eq!(convert_windows_locale_code("0416"), "pt-BR");
        assert_eq!(convert_windows_locale_code("0419"), "ru-RU");

        // 测试未知代码
        assert_eq!(convert_windows_locale_code("9999"), "en-US");
        assert_eq!(convert_windows_locale_code(""), "en-US");
    }

    #[test]
    fn test_locale_context_message() {
        let context = LocaleContext::init();

        // 测试一个已知的消息
        let result = context.message("name-cannot-be-empty", None);
        assert!(result.is_ok());

        // 测试不存在的消息
        let result = context.message("non-existent-message", None);
        assert!(result.is_err());
    }

    #[test]
    fn test_locale_args() {
        let args = LocaleArgs::new()
            .set("name", "test-plugin")
            .set("version", "1.0.0");

        // 验证 LocaleArgs 可以正常创建和使用
        let context = LocaleContext::init();
        let result = context.message("plugin-created-successfully", Some(&args));
        assert!(result.is_ok());

        let message = result.unwrap();
        assert!(message.contains("test-plugin"));
    }
}
