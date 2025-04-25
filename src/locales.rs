use anyhow::{Result, anyhow};
use fluent::concurrent::FluentBundle;
use fluent::{FluentArgs, FluentResource};
use std::{borrow::Cow, env, sync::LazyLock};
use unic_langid::langid;

pub(crate) fn update_get_latest_version_err(err: &str) -> Cow<'_, str> {
    LOCALE
        .message(
            "update-get-latest-version-err",
            Some(&LocaleArgs::new().set("error", err)),
        )
        .unwrap()
}

pub(crate) fn update_using_the_latest_version(version: &str) -> Cow<'_, str> {
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

pub(crate) fn cargo_exited_with_status(status: &str) -> Cow<'_, str> {
    LOCALE
        .message(
            "cargo-exited-with-status",
            Some(&LocaleArgs::new().set("status", status)),
        )
        .unwrap()
}

pub(crate) fn failed_to_execute_cargo() -> Cow<'static, str> {
    LOCALE.message("failed-to-execute-cargo", None).unwrap()
}

pub(crate) fn update_windows_manually_to_use() -> Cow<'static, str> {
    LOCALE
        .message("update-windows-manually-to-use", None)
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

    pub(crate) fn message<'bundle, 'args>(
        &'bundle self,
        id: &str,
        args: Option<&'args LocaleArgs<'_>>,
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
    env::var("LC_ALL")
        .or_else(|_| env::var("LANG"))
        .unwrap_or_else(|_| "en-US".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_fluent() {
        init_fluent();
    }

    #[test]
    fn test_get_message() {
        let msg = LOCALE
            .message(
                "updete-get-latest-version-err",
                Some(&LocaleArgs::new().set("error", "i am error")),
            )
            .unwrap();

        println!("{msg}",);
    }
}
