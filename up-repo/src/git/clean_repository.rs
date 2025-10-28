use std::{
    env::{current_dir, set_current_dir},
    fs::{DirEntry, ReadDir, read_dir},
    io::{Error, StdoutLock, Write, stdout},
    path::PathBuf,
    process::{Command, Output, exit},
    result::{
        Result,
        Result::{Err, Ok},
    },
};

// Clean All Git Repositories in a Directory
pub fn clean_repo() -> () {
    let current_directory_path: PathBuf = current_dir().unwrap();
    let current_directory: ReadDir = read_dir(current_directory_path).unwrap();

    for directory_entry in current_directory {
        let entry: DirEntry = directory_entry.unwrap();
        let path: PathBuf = entry.path();
        let git_repository: Result<(), Error> = set_current_dir(path);
        let mut standard_output: StdoutLock = stdout().lock();

        match git_repository {
            Ok(()) => {
                let git_gc: Result<Output, Error> = Command::new("git").arg("gc").output();

                match git_gc {
                    Ok(cleaning) => {
                        writeln!(
                            standard_output,
                            "\x1b[32;1;3mCleaning Git Repository...\x1b[0m"
                        )
                        .unwrap();
                        standard_output.write_all(&cleaning.stdout).unwrap();
                        writeln!(standard_output, "{}", cleaning.status).unwrap();
                    }
                    Err(error) => {
                        eprintln!(
                            "\x1b[31;1;3;4mError Cleaning Git Repositories:\x1b[0m {}",
                            error
                        );
                        exit(1);
                    }
                };
            }
            Err(error) => {
                eprintln!(
                    "\x1b[31;1;3;4mError Setting Current Directory:\x1b[0m {}",
                    error
                );
                writeln!(standard_output, "Skipping...").unwrap();
            }
        }
    }

    return ();
}
