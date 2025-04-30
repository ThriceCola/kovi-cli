use clap::{Parser, Subcommand};
use clap_cargo::style::CLAP_STYLING;
use cmd::{
    add::{add, add_to},
    new_kovi::new_kovi,
    new_plugin::new_plugin,
    update::update,
};

mod cmd;
mod locales;
mod utils;

pub(crate) use locales::*;

#[derive(Parser)]
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
#[command(styles = CLAP_STYLING)]
enum CargoCli {
    Kovi(KoviArgs),
}

#[derive(Parser, Debug)]
#[command(name = "kovi", version, about)]
struct KoviArgs {
    #[command(subcommand)]
    command: KoviCommands,
}

#[derive(Subcommand, Debug)]
enum KoviCommands {
    #[command(
        alias = "c",
        about = "Creates a new Kovi plugin with the specified name."
    )]
    Create {
        name: String,
        #[arg(short, long, help = "Generate lib.rs without extra examples")]
        simple: bool,
        #[arg(short, long, help = "Add 'kovi-plugin-' prefix to the plugin name")]
        prefix: bool,
    },

    #[command(
        alias = "n",
        about = "Creates a new Kovi project with a default or specified name and an optional version."
    )]
    New {
        #[arg()]
        name: Option<String>,

        #[arg(short, long, help = "Non guided approach")]
        simple: bool,
    },

    #[command(
        alias = "a",
        about = "Adds a new component or dependency to the existing Kovi project."
    )]
    Add {
        name: String,
        /// 给某一个插件添加依赖项
        #[arg(short, long, help = "Add a dependency to a specific plugin")]
        package: Option<String>,
    },

    #[command(about = "Updates the Kovi cli to the latest version.")]
    Update,
}

fn main() {
    let CargoCli::Kovi(args) = CargoCli::parse();

    match args.command {
        KoviCommands::Create {
            name,
            simple,
            prefix,
        } => new_plugin(name, simple, prefix),
        KoviCommands::New { name, simple } => new_kovi(name, simple),
        KoviCommands::Add { name, package } => match package {
            Some(package) => add_to(name, package),
            None => add(name),
        },
        KoviCommands::Update => update(),
    }
}
