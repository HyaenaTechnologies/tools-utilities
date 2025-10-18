use std::io::{StdoutLock, Write, stdout};

// Print Help Command Output
pub fn print_help_message() -> () {
    let mut standard_output: StdoutLock = stdout().lock();

    writeln!(standard_output, "\x1b[32;1;3;4mSystsem Update Tool\x1b[0m").unwrap();
    writeln!(standard_output, "").unwrap();
    writeln!(
        standard_output,
        "\x1b[32;1;3mCommands:\t\t Description:\x1b[0m"
    )
    .unwrap();
    writeln!(standard_output, "").unwrap();
    writeln!(standard_output, "\x1b[32;3mau\x1b[0m\t\t APT Upgrade").unwrap();
    writeln!(
        standard_output,
        "\x1b[32;3mdr\x1b[0m\t\t DNF System Release Upgrade"
    )
    .unwrap();
    writeln!(standard_output, "\x1b[32;3mdu\x1b[0m\t\t DNF Upgrade").unwrap();
    writeln!(
        standard_output,
        "\x1b[32;3mhelp\x1b[0m\t\t Print Commands and Flags"
    )
    .unwrap();
    writeln!(standard_output, "\x1b[32;3msr\x1b[0m\t\t Snap Refresh").unwrap();
    writeln!(
        standard_output,
        "\x1b[32;3mur\x1b[0m\t\t Ubuntu System Releade Upgrade"
    )
    .unwrap();
    writeln!(
        standard_output,
        "\x1b[32;3mversion\x1b[0m\t\t Print Version Number"
    )
    .unwrap();
    writeln!(standard_output, "").unwrap();
    writeln!(standard_output, "\x1b[32;1;3mFlags: Description:\x1b[0m").unwrap();
    writeln!(standard_output, "").unwrap();
    writeln!(standard_output, "\x1b[32;3m--au\x1b[0m\t\t APT Upgrade").unwrap();
    writeln!(
        standard_output,
        "\x1b[32;3m--dr\x1b[0m\t\t DNF System Release Upgrade"
    )
    .unwrap();
    writeln!(standard_output, "\x1b[32;3m--du\x1b[0m\t\t DNF Upgrade").unwrap();
    writeln!(
        standard_output,
        "\x1b[32;3m--h\x1b[0m\t\t Print Commands and Flags"
    )
    .unwrap();
    writeln!(standard_output, "\x1b[32;3m--sr\x1b[0m\t\t Snap Refresh").unwrap();
    writeln!(
        standard_output,
        "\x1b[32;3m--ur\x1b[0m\t\t Ubuntu System Releade Upgrade"
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
