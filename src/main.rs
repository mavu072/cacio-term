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

mod init;
mod input;

fn main() {
    println!("Starting up...");

    init::print_help();
}
