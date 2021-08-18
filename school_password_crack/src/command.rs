mod crack;
mod help;
mod lock;
mod test;

#[macro_use]
use super::color;
use super::color::Color;
use super::common;

pub static mut COMMANDS: Vec<Command> = Vec::new();

pub unsafe fn load_commands() {
    COMMANDS.push(test::command());
    COMMANDS.push(help::command());
    COMMANDS.push(crack::command());
    COMMANDS.push(lock::command());
}

pub unsafe fn parse_command(args: &[String]) -> bool {
    let args_len = args.len();
    if args_len <= 1 {
        no_sub_command(false);
    } else if args_len >= 2 {
        for i in COMMANDS.iter() {
            if &args[1].to_lowercase() == &i.name.to_lowercase() {
                (i).execute(args);
                return true;
            }
        }
        incorrect_command(args[1].to_lowercase())
    }
    false
}

unsafe fn no_sub_command(sub_cmd: bool) {
    if !sub_cmd {
        color_print!(Color::Red, "[*] No sub-command supplied...");
    }
    color_print!(Color::Yellow, " └── SubCommands");
    for i in COMMANDS.iter() {
        if i.name == COMMANDS.last().unwrap().name {
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

unsafe fn incorrect_command(command: String) {
    color_print!(Color::Red, &*format!("[*] Unknown Command: `{}`", command));
    let mut best = "";
    let mut best_score = 0.0;
    for i in COMMANDS.iter() {
        let score = common::similarity(&command, &i.name);
        if score > best_score {
            best = &i.name;
            best_score = score;
        }
    }
    if best_score < 0.5 {
        no_sub_command(true);
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
