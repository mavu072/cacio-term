/*!
 * A simple digital watch emulator for the terminal.
 *
 * ## Created
 * - Author: Avuyile Mgxotshwa
 * - Date: 10 October 2026
 *
 * ## Contributors
 * - Avuyile Mgxotshwa <mavu072@noreply.codeberg.org>
 *
 * ## Licence
 * Refer to LICENSE file.
 */

use color_eyre::eyre::Result;

mod init;
mod input;

fn main() -> Result<()> {
    init::print_help();

    let mut input = input::Input::new();

    // program loop
    while !input.quit {
        // read terminal input
        // let events = terminal.read_events()?;
        let events = [];
        input.update(&events);

        if input.reset {
            println!("A pressed");
        }
        if input.light {
            println!("B pressed");
        }
        if input.mode {
            println!("C pressed");
        }
        if input.start {
            println!("D pressed");
        }
    }

    println!(">>>>> Gracefully shutting down...");

    Ok(())
}
