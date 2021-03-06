use std::io;
use std::io::Write;
use std::sync::mpsc;
use std::thread;
use std::time;
use std::time::Instant;

use super::super::BASE_PAGE;
use super::color;
use super::color::Color;
use super::Command;

const SPINNER: [char; 10] = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];

pub fn command() -> Command {
    Command::new("test", "Make sure everything's working", "test", |_| {
        color_print!(Color::Magenta, "\n[*] Starting Self Check");

        // Make sure we can connect to the internet
        test("Internet Access", || {
            ureq::get("https://connorcode.com").call().is_ok()
        });

        // Make sure Genesis is up
        test("Genesis Up", || ureq::get(BASE_PAGE).call().is_ok());

        // Make sure the Session Id is being set
        test("Genesis Session", || {
            let session = ureq::get(&format!("{}/sis/view", BASE_PAGE)).call();
            match session.unwrap().header("set-cookie") {
                Some(cookie) => cookie.contains("JSESSIONID"),
                None => false,
            }
        });
    })
}

// Note to self -> I should make a unit testing framework
// because that is... necessary
fn test(name: &str, test: fn() -> bool) {
    let mut update: Instant = time::Instant::now();
    let mut spin: usize = 0;

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send(test()).unwrap();
    });

    loop {
        if let Ok(result) = rx.try_recv() {
            if result {
                print!(
                    "{}",
                    color::color(&format!("\r[+] {}\n", name), Color::Green)
                );
                break;
            }
            print!("{}", color::color(&format!("\r[-] {}\n", name), Color::Red));
            break;
        }

        if update.elapsed().as_millis() < 100 {
            continue;
        }
        update = time::Instant::now();

        let spin_char = SPINNER.iter().nth(spin).unwrap();
        spin = (spin + 1) % SPINNER.len();

        print!(
            "{}",
            color::color(&format!("\r[{}] {}", spin_char, name), Color::Blue)
        );
        io::stdout().flush().expect("Err flushing STD Out");
    }
}
