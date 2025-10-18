use std::{
    env::args,
    io::{StdoutLock, Write, stdout},
    process::ExitCode,
    string::String,
    vec::Vec,
};

use super::{print_help::print_help_message, print_version::print_version_number};

use crate::upgrades::{
    apt_upgrade::upgrade_apt, dnf_release::release_dnf, dnf_upgrade::upgrade_dnf,
    snap_refresh::refresh_snap, ubuntu_release::release_ubuntu,
};

// Command Line Argument Tokenizer
pub fn tokenize_arguments() -> ExitCode {
    let command_line_arguments: Vec<String> = args().collect();
    let mut standard_output: StdoutLock = stdout().lock();

    if command_line_arguments.len() != 2 {
        writeln!(
            standard_output,
            "\x1b[31;1;3;4mCommand or Flag Required but not Both:\x1b[0m {:#?}",
            command_line_arguments
        )
        .unwrap();
        print_help_message();
        eprintln!("\x1b[31;1;3;4mError(1) - Exiting System Update Tool\x1b[0m");
        return ExitCode::FAILURE;
    } else {
        match command_line_arguments[1].trim() {
            "au" | "--au" => {
                upgrade_apt();
            }
            "dr" | "--dr" => {
                release_dnf();
            }
            "du" | "--du" => {
                upgrade_dnf();
            }
            "help" | "--h" => {
                print_help_message();
            }
            "sr" | "--sr" => {
                refresh_snap();
            }
            "ur" | "--ur" => {
                release_ubuntu();
            }
            "version" | "--v" => {
                print_version_number();
            }
            &_ => {
                writeln!(
                    standard_output,
                    "\x1b[31;1;3;4mUknown Command or Flag:\x1b[0m {:#?}",
                    command_line_arguments[1].trim()
                )
                .unwrap();
                print_help_message();
                eprintln!("\x1b[31;1;3;4mError(1) - Exiting System Update Tool\x1b[0m");
                return ExitCode::FAILURE;
            }
        };
    };

    return ExitCode::SUCCESS;
}
