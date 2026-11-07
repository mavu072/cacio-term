/// Prints the help manual to the terminal.
pub fn print_help() {
    print!("{}", ansi_styled_text(include_str!("../assets/help.txt")));
}

/// Maps string literals in a template to ANSI escape sequences
/// and returns text formatted for the terminal
fn ansi_styled_text(text: &str) -> String {
    let template = &[
        ("reset", "\x1b[0m"),
        ("bold", "\x1b[1m"),
        ("dim", "\x1b[2m"),
        ("bg1", "\x1b[107m"),
        ("c1", "\x1b[31m"), // grey
        ("c2", "\x1b[32m"), // red
        ("c3", "\x1b[33m"), // green
        ("c4", "\x1b[34m"), // orange
        ("c5", "\x1b[35m"), // blue
        ("c6", "\x1b[36m"), // navy
        ("c7", "\x1b[37m"), // aquamarine
        ("c8", "\x1b[38m"), // darker grey
    ]; // See more colors https://gist.github.com/raghav4/48716264a0f426cf95e4342c21ada8e7

    let mut output = text.to_string();

    for (key, val) in template {
        // Format key to match template e.g. ${c1}
        let key = format!("$({})", key);
        // Replace template with ANSI sequence
        output = output.replace(&key, val);
    }

    // Return styled text
    output
}
