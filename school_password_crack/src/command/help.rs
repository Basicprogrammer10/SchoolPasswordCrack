use super::color;
use super::color::Color;
use super::common;
use super::Command;
use super::COMMANDS;

pub fn command() -> Command {
    Command::new("help", "Get Info on this program :P", |args| {
        // :'(
        // Ferris is sad about all this unsafe code.
        unsafe {
            // For Help on a command
            if args.len() > 2 {
                let command = (&args[2]).clone();
                let mut found_command = false;
                let mut command_info: String = "".to_string();

                for i in COMMANDS.iter() {
                    if i.name == command {
                        command_info = (&i.description).clone();
                        found_command = true;
                        break;
                    }
                }

                if !found_command {
                    color_print!(Color::Red, "[*] Command '{}' was not found!", &command);
                    return;
                }

                color_print!(Color::Cyan, "\n[*] Help For '{}'", &common::upper_first_char(&command));
                color_print!(
                    Color::Cyan,
                    "[*] {}",
                    &command_info
                );
                return;
            }

            // General Help
            color_print!(Color::Magenta, "\n{}", HELP);
            color_print!(Color::Cyan, "\n[*] Available Commands");
            if COMMANDS.is_empty() {
                color_print!(Color::Red, " └─── Somthing Aint Right... No Commands Found");
            }
            for i in COMMANDS.iter() {
                if i.name == COMMANDS.last().unwrap().name {
                    color_print!(
                        Color::Yellow,
                        " └─── {}",
                        &common::upper_first_char(&i.name)
                    );
                    continue;
                }
                color_print!(
                    Color::Yellow,
                    " ├─── {}",
                    &common::upper_first_char(&i.name)
                );
            }
        }
    })
}

/// Help Info
static HELP: &str = r#"[*] Welcome to SchoolPasswordCrack CLI By Connor Slade!
[*] This is an open source program to crack passwords for the genesis System.
[*] It basically just tries lots of passwords until it is able to login
[*] For more info on any commad just run 'help [COMMAND]'"#;
