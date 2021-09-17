use super::color;
use super::color::Color;
use super::common;
use super::Command;
use super::COMMANDS;

pub fn command() -> Command {
    Command::new(
        "help",
        "Get Info on this program :P",
        "help [COMMAND]",
        |args| {
            // :'(
            // Ferris is sad about all this unsafe code.
            unsafe {
                // For Help on a command
                if args.len() > 2 {
                    let command = (&args[2]).clone();
                    let mut found_command = false;
                    let mut command_info: String = "".to_string();
                    let mut command_usage: String = "".to_string();

                    for i in COMMANDS.clone().unwrap().iter() {
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
                        let commands = COMMANDS.clone().unwrap();
                        for i in commands.iter() {
                            let score = common::similarity(&command, &i.name);
                            if score > best_score {
                                best = &i.name;
                                best_score = score;
                            }
                        }

                        color_print!(
                            Color::Red,
                            "[*] Command {} {}",
                            &color::color(&command, Color::Blue),
                            &color::color("was not found!", Color::Red)
                        );

                        if best_score < 0.5 {
                            return;
                        }
                        color_print!(Color::Red, "[*] Unknown Command: `{}`", &best);
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
                color_print!(Color::Magenta, "\n{}", HELP);
                color_print!(Color::Cyan, "\n[*] Available Commands");
                if COMMANDS.clone().unwrap().is_empty() {
                    color_print!(Color::Red, " └─── Somthing Aint Right... No Commands Found");
                }
                for i in COMMANDS.clone().unwrap().iter() {
                    if i.name == COMMANDS.clone().unwrap().last().unwrap().name {
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
        },
    )
}

/// Help Info
static HELP: &str = r#"[*] Welcome to SchoolPasswordCrack CLI By Connor Slade!
[*] This is an open source program to crack passwords for the genesis System.
[*] It basically just tries lots of passwords until it is able to login
[*] For more info on any command just run 'help [COMMAND]'"#;
