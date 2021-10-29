use std::env;

#[macro_use]
mod color;
mod arg_parse;
mod command;
mod common;
mod random;
use color::Color;

// This should be like https://parents.genesisedu.com/SCHOOL_PAGE
pub const BASE_PAGE: &str = "";
pub const VERSION: &str = "2.1.9";
fn main() {
    let args: Vec<String> = env::args().collect();

    println!(
        "{}",
        color::color_bold(
            &format!("[*] Starting School Password Crack CLI (V{})", VERSION),
            Color::Green
        )
    );

    if BASE_PAGE.is_empty() {
        color_print!(Color::Red, "[-] Defult Base Page not defined");
    }

    // Load Commands
    let commands = command::load_commands();

    // Parse and run Sub Commands
    command::parse_command(commands, &args);
}
