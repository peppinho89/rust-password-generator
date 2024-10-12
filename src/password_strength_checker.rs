use crate::{
    constants::{LOWERCASE, NUMBERS, SPECIAL_CHARS, UPPERCASE},
    utils,
};

pub fn check_password_flow() {
    println!("Enter the password you want to check:");
    let password = utils::get_user_input().trim().to_string();

    let strength = check_password_strength(&password);
    println!("Password Strength: {}", strength);
}

fn check_password_strength(password: &str) -> String {
    let length = password.len();
    let mut score = 0;

    score += match length {
        12.. => 2,
        8..=11 => 1,
        _ => 0,
    };

    let has_lowercase = password.chars().any(|c| LOWERCASE.contains(c));
    let has_uppercase = password.chars().any(|c| UPPERCASE.contains(c));
    let has_numbers = password.chars().any(|c| NUMBERS.contains(c));
    let has_special_chars = password.chars().any(|c| SPECIAL_CHARS.contains(c));

    if has_lowercase {
        score += 1;
    }
    if has_uppercase {
        score += 1;
    }
    if has_numbers {
        score += 1;
    }
    if has_special_chars {
        score += 1;
    }

    if !contains_repeated_chars(password) {
        score += 1;
    }

    match score {
        0..=2 => "Weak".to_string(),
        3..=4 => "Moderate".to_string(),
        5..=6 => "Strong".to_string(),
        _ => "Very Strong".to_string(),
    }
}

fn contains_repeated_chars(password: &str) -> bool {
    let mut chars = password.chars().collect::<Vec<_>>();
    chars.dedup();
    chars.len() != password.len()
}
