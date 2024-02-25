use rand::Rng;
use std::env;
// use std::io::{self, Write};

const UPPERCASE_CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWERCASE_CHARS: &str = "abcdefghijklmnopqrstuvwxyz";
const NUMBERS: &str = "0123456789";
const SPECIAL_CHARS: &str = "~!@#$%^&*()-_=+[]{}|;:',.<>?";

fn generate_password(
    length: usize,
    include_uppercase: bool,
    include_lowercase: bool,
    include_numbers: bool,
    include_special_chars: bool,
) -> String {
    let mut rng = rand::thread_rng();

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

    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset.chars().nth(idx).unwrap()
        })
        .collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let _rng = rand::thread_rng();

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

    let password = generate_password(12, include_uppercase, include_lowercase, include_numbers, include_special_chars);

    println!("Generated password: {}", password);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_password_default_options() {
        // Test generating a password with default options
        let password = generate_password(10, true, true, true, true);
        assert_eq!(password.len(), 10);
    }

    #[test]
    fn test_generate_password_no_uppercase() {
        // Test generating a password without uppercase letters
        let password = generate_password(10, false, true, true, true);
        assert!(!password.chars().any(char::is_uppercase));
    }

    #[test]
    fn test_generate_password_no_lowercase() {
        // Test generating a password without lowercase letters
        let password = generate_password(10, true, false, true, true);
        assert!(!password.chars().any(char::is_lowercase));
    }

    #[test]
    fn test_generate_password_no_numbers() {
        // Test generating a password without numbers
        let password = generate_password(10, true, true, false, true);
        assert!(!password.chars().any(char::is_numeric));
    }

    #[test]
    fn test_generate_password_no_special_chars() {
        // Test generating a password without special characters
        let password = generate_password(10, true, true, true, false);
        let special_chars: Vec<char> = "~!@#$%^&*()-_=+[]{}|;:',.<>?".chars().collect();
        assert!(!password.chars().any(|c| special_chars.contains(&c)));
    }
}
