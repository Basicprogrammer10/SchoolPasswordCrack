use super::color;
use super::color::Color;
use super::Command;

pub fn command() -> Command {
    Command::new(
        "crack",
        "Crack A password",
        "crack student@bernardsboe.com",
        |args| {
            color_print!(
                Color::Green,
                "\n[*] Starting Crack on {}",
                &color::color(&args[2], Color::Blue)
            );
        },
    )
}
