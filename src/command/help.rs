use super::color;
use super::color::Color;
use super::common;
use super::Command;
use super::COMMANDS;
use crate::VERSION;

pub fn command() -> Command {
    Command::new(
        "help",
        "Get Info on this program :P",
        "help [COMMAND]",
        |args| {
            let commands = unsafe { COMMANDS.clone() }.unwrap();

            // For Help on a command
            if args.len() > 2 {
                let command = (&args[2]).clone();
                let mut found_command = false;
                let mut command_info: String = "".to_string();
                let mut command_usage: String = "".to_string();

                for i in commands.iter() {
                    if i.name == command.to_lowercase() {
                        command_info = (&i.description).clone();
                        command_usage = (&i.usage).clone();
                        found_command = true;
                        break;
                    }
                }

                if !found_command {
                    let mut best = "";
                    let mut best_score = 0.0;
                    let commands = commands;
                    for i in commands.iter() {
                        let score = common::similarity(&command, &i.name);
                        if score > best_score {
                            best = &i.name;
                            best_score = score;
                        }
                    }

                    color_print!(Color::Red, &format!("[*] Unknown Command: `{}`", command));

                    if best_score < 0.5 {
                        return;
                    }

                    color_print!(
                        Color::Cyan,
                        " └── Did you mean {}",
                        &color::color(best, Color::Magenta)
                    );

                    return;
                }

                color_print!(
                    Color::Cyan,
                    " └─── {}",
                    &color::color(&common::upper_first_char(&command), Color::Magenta)
                );
                color_print!(
                    Color::Cyan,
                    "     ├─── {}",
                    &command_info.replace("\\", "\n     └─── ")
                );
                color_print!(Color::Cyan, "     └─── Usage: {}", &command_usage);
                return;
            }

            // General Help
            color_print!(
                Color::Magenta,
                "\n{}",
                &HELP.replace("{{VERSION}}", VERSION)
            );
            color_print!(Color::Cyan, "\n[*] Available Commands");
            if commands.is_empty() {
                color_print!(Color::Red, " └─── Somthing Aint Right... No Commands Found");
            }

            let mut long = 0;
            for i in commands.iter() {
                if i.name.len() > long {
                    long = i.name.len();
                }
            }

            for i in commands.clone() {
                if i.name == commands.last().unwrap().name {
                    color_print!(
                        Color::Yellow,
                        " └─── {}{}— {}",
                        &common::upper_first_char(&i.name),
                        &common::get_spaceing(long, i.name.clone()),
                        &i.description
                    );
                    continue;
                }
                color_print!(
                    Color::Yellow,
                    " ├─── {}{}— {}",
                    &common::upper_first_char(&i.name),
                    &common::get_spaceing(long, i.name.clone()),
                    &i.description
                );
            }
        },
    )
}

/// Help Info
const HELP: &str = r#"[*] Welcome to SchoolPasswordCrack CLI By Connor Slade, Version {{VERSION}}!
[*] This is an open source program to crack passwords for the genesis System.
[*] It basically just tries lots of passwords until it is able to login
[*] For more info on any command just run 'help [COMMAND]'"#;
