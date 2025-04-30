use crate::{cargo_exited_with_status, failed_to_execute_cargo};
use std::process::Command;

pub(crate) fn run_cargo_command(mut command: Command) -> Result<(), ()> {
    match command.status() {
        Ok(status) if status.success() => Ok(()),
        Ok(status) => {
            let status = format!("{}", status);
            let msg = cargo_exited_with_status(&status);
            eprintln!("{msg}");
            Err(())
        }
        Err(e) => {
            let e = e.to_string();
            let msg = failed_to_execute_cargo(&e);
            eprintln!("{msg}");
            Err(())
        }
    }
}

#[allow(clippy::crate_in_macro_def)]
#[macro_export]
macro_rules! run_cargo_command_return {
    ($cargo_command:expr) => {
        if let Err(_) = crate::utils::run_cargo_command($cargo_command) {
            return;
        }
    };
}
