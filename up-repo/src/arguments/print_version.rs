use std::{
    io::{StdoutLock, Write, stdout},
    primitive::str,
};

// Print Version Number
pub fn print_version_number() -> () {
    let mut standard_output: StdoutLock = stdout().lock();
    let version_number: &str = "1.0.0";

    writeln!(
        standard_output,
        "\x1b[32;1;3;4mGit Repository Update Tool\x1b[0m"
    )
    .unwrap();
    writeln!(standard_output, "").unwrap();
    writeln!(
        standard_output,
        "\x1b[32;1;3mVersion Number:\x1b[0m\t{}",
        version_number
    )
    .unwrap();

    return ();
}
