use std::collections::hash_map::DefaultHasher;
use std::fs::OpenOptions;
use std::hash::Hash;
use std::hash::Hasher;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;

// External Crates
use home::home_dir;
use ureq::Agent;

// Internal Modules
use super::super::arg_parse;
use super::color;
use super::color::Color;
use super::common;
use super::Command;
use crate::random;
use crate::BASE_PAGE;

pub fn command() -> Command {
    Command::new(
        "info",
        "Get Info an account.",
        "lock <username> [password] [-nc No Cache] [--cache Cache Path] [--page BasePage]",
        |args| {
            let default_cache = home_dir()
                .unwrap()
                .join(Path::new(".SchoolPasswordCrack/cache"));

            let no_cache: bool = arg_parse::get_arg_value(args, "-nc").is_some();
            let cache_path: &str = arg_parse::get_arg_value(args, "--cache")
                .unwrap_or_else(|| default_cache.to_str().unwrap());
            let base_page: &str = arg_parse::get_arg_value(args, "--page").unwrap_or(BASE_PAGE);

            let user: Option<String> = string_from_option_ref(args.get(2));
            let mut pass: Option<String> = None;

            // Check for config file and see if accoutn password in stored
            // If so dont require one to be defined in program args
            if !no_cache && Path::new(cache_path).exists() && user.is_some() {
                pass = get_cache(Path::new(cache_path).to_path_buf(), user.clone().unwrap());
            }

            // Get Username and Password if not already defined
            if pass.is_none() {
                let new_pass = string_from_option_ref(args.get(3));
                if new_pass.is_some() && !new_pass.clone().unwrap().starts_with('-') {
                    pass = new_pass;
                }
            }

            let username = &match user {
                Some(i) => i,
                None => {
                    color_print!(Color::Red, "[-] No Username Defined");
                    return;
                }
            };

            let password = &match pass {
                Some(i) => i,
                None => {
                    color_print!(Color::Red, "[-] No Password Defined");
                    return;
                }
            };

            if !common::is_valid_email(username) {
                color_print!(
                    Color::Red,
                    "[-] The username supplied is not a valid Email..."
                );
            }

            color_print!(
                Color::Green,
                "\n[*] Starting Info on {}",
                &color::color(username, Color::Blue)
            );

            color_print!(Color::Magenta, "[i] Base Page: {}", base_page);
            color_print!(Color::Magenta, "[i] Username: {}", username);
            color_print!(Color::Magenta, "[i] Password: {}", password);
            println!();

            info(username, password, base_page);
        },
    )
}

pub fn info(username: &str, password: &str, base_page: &str) {
    // Login to account and get some info
    // Init new agent
    let agent = Agent::new();

    // Get jsessionid cookie
    match agent.get(&format!("{}/sis/view", base_page)).call() {
        Ok(_) => {}
        Err(_) => {
            color_print!(Color::Red, "[-] Error connecting to server");
            return;
        }
    };

    let body = match agent
        .post(&format!("{}/sis/j_security_check", base_page))
        .query("j_username", username)
        .query("j_password", password)
        .call()
    {
        Ok(body) => body.into_string().unwrap_or_else(|_| "".to_string()),
        Err(_) => {
            color_print!(Color::Red, "[-] Error connecting to server");
            return;
        }
    };
    if body.contains("Account is inactive") {
        color_print!(Color::Red, "[-] Account is inactive");
        return;
    }

    if !body.contains("workStudentId") {
        color_print!(Color::Red, "[-] Login Failed");
        return;
    }

    // Get Student Info
    let student = match Student::from_raw(body, username.to_string()) {
        Some(student) => student,
        None => {
            color_print!(Color::Red, "[-] Error parseing student info");
            return;
        }
    };
    student.display();
}

fn get_cache(path: PathBuf, to_find: String) -> Option<String> {
    let mut file = OpenOptions::new().read(true).open(path).ok()?;

    let mut data = String::new();
    file.read_to_string(&mut data).ok()?;

    for line in data.lines() {
        if line.is_empty() {
            continue;
        }

        let mut entry = line.split(':');
        let user = entry.next()?;
        let pass = entry.next()?;

        if user.to_lowercase() == to_find.to_lowercase() {
            return Some(pass.to_string());
        }
    }

    None
}

fn string_from_option_ref(opt: Option<&String>) -> Option<String> {
    opt?;

    Some(opt.unwrap().to_string())
}

#[derive(Hash)]
struct Student {
    name: String,
    id: String,
    grade: u8,
    school: String,
    email: String,
    age: u8,
    dob: String,
}

impl Student {
    fn new(
        name: String,
        id: String,
        grade: u8,
        school: String,
        email: String,
        age: u8,
        dob: String,
    ) -> Student {
        Student {
            name,
            id,
            grade,
            school,
            email,
            age,
            dob,
        }
    }

    fn from_raw(raw: String, email: String) -> Option<Student> {
        let raw_name = raw
            .split(r#"<td style="font-size: 1.5em;">"#)
            .nth(1)?
            .split("</td>")
            .next()?;

        let first_name = raw_name.split('>').nth(1)?.split('<').next()?;

        let last_name = raw_name.split("\r\n").nth(2)?.split("\r\n").next()?.trim();

        let student_id = raw
            .split("Student ID:")
            .nth(1)?
            .split('>')
            .nth(1)?
            .split('<')
            .next()?;

        let grade = raw
            .split("Grade:")
            .nth(1)?
            .split('>')
            .nth(3)?
            .split('<')
            .next()?
            .parse::<u8>()
            .unwrap_or(0);

        let school = raw
            .split("uppercase\">")
            .nth(2)?
            .split("\r\n")
            .nth(3)?
            .trim();

        let age = raw
            .split("Age:")
            .nth(1)?
            .split('>')
            .nth(2)?
            .split('<')
            .next()?
            .parse::<u8>()
            .unwrap_or(0);

        let dob = raw
            .split("Birthdate:")
            .nth(1)?
            .split('>')
            .nth(2)?
            .split('<')
            .next()?;

        Some(Student::new(
            format!("{} {}", first_name, last_name),
            student_id.to_string(),
            grade,
            school.to_string(),
            email,
            age,
            dob.to_string(),
        ))
    }

    #[rustfmt::skip]
    fn display(&self) {
        // Make a i64 seed from the student info
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        let name = hasher.finish() as i64;

        println!("╭─────────────╮");
        println!("│{}│  \x1B[37mName:   {}\x1B[0m", box_line(13, name^1), self.name);
        println!("│{}│  \x1B[31mID:     {}\x1B[0m", box_line(13, name^2), self.id);
        println!("│{}│  \x1B[34mEmail:  {}\x1B[0m", box_line(13, name^3), self.email);
        println!("│{}│  \x1B[33mSchool: {}\x1B[0m", box_line(13, name^4), self.school);
        println!("│{}│  \x1B[36mDOB:    {}\x1B[0m", box_line(13, name^5), self.dob);
        println!("│{}│  \x1B[35mAge:    {}\x1B[0m", box_line(13, name^6), self.age);
        println!("│{}│  \x1B[32mGrade:  {}\x1B[0m", box_line(13, name^7), self.grade);
        println!("╰─────────────╯");
    }
}

fn box_line(len: usize, seed: i64) -> String {
    let mut rand = random::Random::new(seed as i128);
    let mut line = String::new();
    for _ in 0..len {
        let c = rand.next_int_i64(30, 37) as u8;
        line.push_str(&format!("\x1B[{}m", c));
        match rand.next_f64().abs() {
            x if x < 0.5 => line.push('*'),
            _ => line.push(' '),
        }
    }
    line.push_str("\x1B[0;0m");
    line
}
