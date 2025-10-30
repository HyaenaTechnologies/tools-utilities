use std::{
    io::{BufRead, Error, Lines, StdoutLock, Write, stdout},
    primitive::u8,
    process::{Command, Output, exit},
    result::{
        Result,
        Result::{Err, Ok},
    },
};

// Main Entry Point
// Synchronize GitHub Forks
fn main() -> () {
    let github_repositories: Result<Output, Error> = Command::new("gh")
        .arg("api")
        .arg("users/JohnMatthiasWabwire/repos")
        .arg("--jq")
        .arg(".[].html_url")
        .arg("--paginate")
        .output();
    let mut standard_output: StdoutLock = stdout().lock();

    match github_repositories {
        Ok(listing) => {
            standard_output.write_all(&listing.stdout).unwrap();
            writeln!(standard_output, "{}", listing.status).unwrap();

            let repositories: Lines<&[u8]> = listing.stdout.lines();

            for fork in repositories {
                let github_synchronize: Result<Output, Error> = Command::new("gh")
                    .arg("repo")
                    .arg("sync")
                    .arg(fork.unwrap())
                    .output();                

                match github_synchronize {
                    Ok(synchronizing) => {
                        writeln!(
                            standard_output,
                            "\x1b[32;1;3mSynchronizing GitHub Fork...\x1b[0m"
                        )
                        .unwrap();
                        standard_output.write_all(&synchronizing.stdout).unwrap();
                        writeln!(standard_output, "{}", synchronizing.status).unwrap();
                    }
                    Err(error) => {
                        eprintln!(
                            "\x1b[31;1;3;4mError Synchronizing GitHub Forks:\x1b[0m {}",
                            error
                        );
                        exit(1);
                    }
                };
            }
        }
        Err(error) => {
            eprintln!(
                "\x1b[31;1;3;4mError Listing GitHub Repositories:\x1b[0m {}",
                error
            );
            exit(1);
        }
    };
}
