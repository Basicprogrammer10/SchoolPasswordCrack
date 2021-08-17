use super::Command;

pub fn command() -> Command {
    Command::new("test", |_| {
        println!("Hello, world!");
    })
}