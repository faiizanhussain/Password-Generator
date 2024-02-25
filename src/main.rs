use rand::Rng;
use std::env;
use std::io::{self, Write};

const UPPERCASE_CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWERCASE_CHARS: &str = "abcdefghijklmnopqrstuvwxyz";
const NUMBERS: &str = "0123456789";
const SPECIAL_CHARS: &str = "~!@#$%^&*()-_=+[]{}|;:',.<>?";

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut rng = rand::thread_rng();

    let mut include_uppercase = true;
    let mut include_lowercase = true;
    let mut include_numbers = true;
    let mut include_special_chars = true;

    // Parse command-line arguments
    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "--no-uppercase" => include_uppercase = false,
            "--no-lowercase" => include_lowercase = false,
            "--no-numbers" => include_numbers = false,
            "--no-special-chars" => include_special_chars = false,
            _ => println!("Unknown option: {}", arg),
        }
    }

    let charset: String = {
        let mut charset = String::new();
        if include_uppercase {
            charset.push_str(UPPERCASE_CHARS);
        }
        if include_lowercase {
            charset.push_str(LOWERCASE_CHARS);
        }
        if include_numbers {
            charset.push_str(NUMBERS);
        }
        if include_special_chars {
            charset.push_str(SPECIAL_CHARS);
        }
        charset
    };

    print!("Enter password length: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let length: usize = input.trim().parse().unwrap();

    let password: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset.chars().nth(idx).unwrap()
        })
        .collect();

    println!("Generated password: {}", password);
}
