// External Crates
use micro_rand::Random;
use ureq::Agent;

// Internal Modules
use super::super::arg_parse;
use super::color;
use super::color::Color;
use super::common;
use super::Command;

// static SPINNER: [char; 10] = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];

// static mut RUNNING: i32 = 0;
// static mut REQUESTS: u32 = 0;

pub fn command() -> Command {
    Command::new(
        "info",
        "Get Info an account.",
        "lock <username> <password> [--page BasePage]",
        |args| {
            if args.len() <= 3 {
                color_print!(Color::Red, "[*] Not enough args supplied");
                return;
            }

            let base_page: &str = &arg_parse::get_arg_value(&args, "--page")
                .unwrap_or("https://parents.genesisedu.com/bernardsboe");

            // Get Username
            let username: &str = &args[2];

            // Get Password
            let password: &str = &args[3];

            if !common::is_valid_email(&username) {
                color_print!(
                    Color::Red,
                    "[-] The username supplied is not a valid Email..."
                );
            }

            color_print!(
                Color::Green,
                "\n[*] Starting Info on {}",
                &color::color(&username, Color::Blue)
            );

            color_print!(Color::Magenta, "[i] Base Page: {}", &base_page);
            color_print!(Color::Magenta, "[i] Username: {}", &username);
            color_print!(Color::Magenta, "[i] Password: {}", &password);
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

        let first_name = raw_name.split(">").nth(1)?.split("<").nth(0)?;

        let last_name = raw_name.split("\r\n").nth(2)?.split("\r\n").nth(0)?.trim();

        let student_id = raw
            .split("Student ID:")
            .nth(1)?
            .split(">")
            .nth(1)?
            .split("<")
            .nth(0)?;

        let grade = raw
            .split("Grade:")
            .nth(1)?
            .split(">")
            .nth(3)?
            .split("<")
            .nth(0)?
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
            .split(">")
            .nth(2)?
            .split("<")
            .nth(0)?
            .parse::<u8>()
            .unwrap_or(0);

        let dob = raw
            .split("Birthdate:")
            .nth(1)?
            .split(">")
            .nth(2)?
            .split("<")
            .nth(0)?;

        Some(Student::new(
            format!("{} {}", first_name, last_name),
            student_id.to_string(),
            grade,
            school.to_string(),
            email.to_string(),
            age,
            dob.to_string(),
        ))
    }

    #[rustfmt::skip]
    fn display(&self) {
        // Make a i64 seed from the student name
        let name = self.name.chars().fold(1, |acc, c| acc + c as i64);

        println!("╭─────────────╮");
        println!("│{}│  \x1B[37mName:   {}\x1B[0m", box_line(13, name^1), self.name);
        println!("│{}│  \x1B[31mID:     {}\x1B[0m", box_line(13, name^2), self.id);
        println!("│{}│  \x1B[32mGrade:  {}\x1B[0m", box_line(13, name^3), self.grade);
        println!("│{}│  \x1B[33mSchool: {}\x1B[0m", box_line(13, name^4), self.school);
        println!("│{}│  \x1B[34mEmail:  {}\x1B[0m", box_line(13, name^5), self.email);
        println!("│{}│  \x1B[35mAge:    {}\x1B[0m", box_line(13, name^6), self.age);
        println!("│{}│  \x1B[36mDOB:    {}\x1B[0m", box_line(13, name^7), self.dob);
        println!("╰─────────────╯");
    }
}

fn box_line(len: usize, seed: i64) -> String {
    let mut rand = Random::new(seed);
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
