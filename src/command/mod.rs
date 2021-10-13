mod crack;
mod help;
mod info;
mod lock;
mod test;

#[macro_use]
use super::color;
use super::color::Color;
use super::common;

pub static mut COMMANDS: Option<Vec<Command>> = None;

pub fn load_commands() -> Vec<Command> {
    let commands = vec![
        crack::command(),
        help::command(),
        info::command(),
        lock::command(),
        test::command(),
    ];

    unsafe {
        COMMANDS = Some(commands.clone());
    }
    commands
}

pub fn parse_command(commands: Vec<Command>, args: &[String]) -> bool {
    let args_len = args.len();
    if args_len <= 1 {
        no_sub_command(commands, false);
        return false;
    }
    if args_len >= 2 {
        for i in commands.iter() {
            if args[1].to_lowercase() == i.name.to_lowercase() {
                (i).execute(args);
                return true;
            }
        }
        incorrect_command(commands, args[1].to_lowercase())
    }
    false
}

fn no_sub_command(commands: Vec<Command>, sub_cmd: bool) {
    if !sub_cmd {
        color_print!(Color::Red, "[*] No sub-command supplied...");
    }
    color_print!(Color::Yellow, " └── SubCommands");
    for i in commands.iter() {
        if i.name == commands.last().unwrap().name {
            color_print!(
                Color::Yellow,
                "     └─── {}",
                &common::upper_first_char(&i.name)
            );
            continue;
        }
        color_print!(
            Color::Yellow,
            "     ├─── {}",
            &common::upper_first_char(&i.name)
        );
    }
}

fn incorrect_command(commands: Vec<Command>, command: String) {
    color_print!(Color::Red, &*format!("[*] Unknown Command: `{}`", command));
    let mut best = "";
    let mut best_score = 0.0;
    for i in commands.iter() {
        let score = common::similarity(&command, &i.name);
        if score > best_score {
            best = &i.name;
            best_score = score;
        }
    }
    if best_score < 0.5 {
        no_sub_command(commands, true);
        return;
    }
    color_print!(
        Color::Cyan,
        " └── Did you mean {}",
        &color::color(best, Color::Magenta)
    );
}

pub struct Command {
    pub name: String,
    pub description: String,
    pub usage: String,
    pub func: fn(args: &[String]),
}

impl Command {
    pub fn new(name: &str, description: &str, usage: &str, func: fn(args: &[String])) -> Command {
        Command {
            name: name.to_string(),
            description: description.to_string(),
            usage: usage.to_string(),
            func,
        }
    }

    pub fn execute(&self, args: &[String]) {
        (self.func)(args)
    }
}

// Impl Copy for Command
impl Clone for Command {
    fn clone(&self) -> Self {
        Command {
            name: self.name.clone(),
            description: self.description.clone(),
            usage: self.usage.clone(),
            func: self.func,
        }
    }
}
