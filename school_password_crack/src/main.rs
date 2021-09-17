#![allow(unused_attributes)]

use std::env;

#[macro_use]
mod color;
mod arg_parse;
mod command;
mod common;
use color::Color;

pub const BASE_PAGE: &str = "https://parents.genesisedu.com/bernardsboe";
pub const VERSION: &str = "0.1.4";

fn main() {
    let args: Vec<String> = env::args().collect();

    println!(
        "{}",
        color::color_bold(
            &format!("[*] Starting School Password Crack CLI (V{})", VERSION),
            Color::Green
        )
    );

    // Load Commands
    let commands = command::load_commands();

    // Parse and run Sub Commands
    command::parse_command(commands, &args);
}
