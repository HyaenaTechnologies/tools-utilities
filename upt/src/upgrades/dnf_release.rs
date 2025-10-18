use std::{
    io::{Error, StdoutLock, Write, stdout},
    process::{Command, Output, exit},
    result::{
        Result,
        Result::{Err, Ok},
    },
};

// DNF System Release Upgrade
pub fn release_dnf() -> () {
    let dnf_release: Result<Output, Error> = Command::new("dnf")
        .arg("system-upgrade")
        .arg("download")
        .arg("--releasever=42")
        .output();
    let mut standard_output: StdoutLock = stdout().lock();

    match dnf_release {
        Ok(release) => {
            standard_output.write_all(&release.stdout).unwrap();
            writeln!(standard_output, "{}", release.status).unwrap();
        }
        Err(error) => {
            eprintln!(
                "\x1b[31;1;3;4mError Executing DNF System Release Upgrade:\x1b[0m {}",
                error
            );
            exit(1);
        }
    };

    return ();
}
