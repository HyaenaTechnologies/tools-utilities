use std::{
    io::{Error, StdoutLock, Write, stdout},
    process::{Command, Output, exit},
    result::{
        Result,
        Result::{Err, Ok},
    },
};

// Ubuntu System Release Upgrade
pub fn release_ubuntu() -> () {
    let ubuntu_release: Result<Output, Error> = Command::new("do-release-upgrade").output();
    let mut standard_output: StdoutLock = stdout().lock();

    match ubuntu_release {
        Ok(release) => {
            standard_output.write_all(&release.stdout).unwrap();
            writeln!(standard_output, "{}", release.status).unwrap();
        }
        Err(error) => {
            eprintln!("\x1b[31;1;3;4mError Executing Ubuntu System Release Upgrade:\x1b[0m {}", error);
            exit(1);
        }
    };

    return ();
}
