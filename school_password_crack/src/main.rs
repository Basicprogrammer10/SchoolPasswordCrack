use std::env;
use std::io::Write;
use std::process;
use std::thread;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};
use ureq::{Agent, AgentBuilder};

#[macro_use]
mod color;
mod command;
mod common;
use color::Color;

static VERSION: &str = "0.0";

fn main() {
  let args: Vec<String> = env::args().collect();

  color_print!(
    Color::Green,
    "[*] Starting School Password Crack CLI (V{})",
    VERSION
  );

  command::parse_command(&args);
}