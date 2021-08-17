use super::Command;

pub fn command() -> Command {
    Command::new("test", "Just A Test Command :P", |_| {
        println!("Hello, world!");
    })
}
