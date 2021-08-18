// Base code to make a command :P

use super::Command;

pub fn command() -> Command {
    Command::new("test", "Just A Test Command :P", "test", |_| {
        println!("Hello, world!");
    })
}
