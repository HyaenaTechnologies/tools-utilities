use std::io::{StdoutLock, Write, stdout};

// Print Help Command Output
pub fn print_help_message() -> () {
    let mut standard_output: StdoutLock = stdout().lock();

    writeln!(
        standard_output,
        "\x1b[32;1;3;4mGit Respository Update Tool\x1b[0m"
    )
    .unwrap();
    writeln!(standard_output, "").unwrap();
    writeln!(
        standard_output,
        "\x1b[32;1;3mCommands:\t\t Description:\x1b[0m"
    )
    .unwrap();
    writeln!(standard_output, "").unwrap();
    writeln!(
        standard_output,
        "\x1b[32;3mclean\x1b[0m\t\t Git Garbage Collection"
    )
    .unwrap();
    writeln!(standard_output, "\x1b[32;3mupdate\x1b[0m\t\t Git Pull").unwrap();
    writeln!(
        standard_output,
        "\x1b[32;3mhelp\x1b[0m\t\t Print Commands and Flags"
    )
    .unwrap();
    writeln!(
        standard_output,
        "\x1b[32;3mversion\x1b[0m\t\t Print Version Number"
    )
    .unwrap();
    writeln!(standard_output, "").unwrap();
    writeln!(
        standard_output,
        "\x1b[32;1;3mFlags:\t\t Description:\x1b[0m"
    )
    .unwrap();
    writeln!(standard_output, "").unwrap();
    writeln!(
        standard_output,
        "\x1b[32;3m--c\x1b[0m\t\t Git Garbage Collection"
    )
    .unwrap();
    writeln!(standard_output, "\x1b[32;3m--u\x1b[0m\t\t Git Pull").unwrap();
    writeln!(
        standard_output,
        "\x1b[32;3m--h\x1b[0m\t\t Print Commands and Flags"
    )
    .unwrap();
    writeln!(
        standard_output,
        "\x1b[32;3m--v\x1b[0m\t\t Print Version Number"
    )
    .unwrap();
    writeln!(standard_output, "").unwrap();

    return ();
}
