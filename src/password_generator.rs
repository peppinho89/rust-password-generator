use crate::{
    constants::{LOWERCASE, NUMBERS, SPECIAL_CHARS, UPPERCASE},
    qrcode, utils,
};
use getrandom::getrandom;

pub fn generate_password_flow() {
    println!("Enter the desired password length:");
    let length: usize = utils::get_user_input()
        .trim()
        .parse()
        .expect("Please enter a number");

    let include_special_chars = utils::ask_yes_no("Include special characters? (y/n): ");
    let include_numbers = utils::ask_yes_no("Include numbers? (y/n): ");
    let include_uppercase = utils::ask_yes_no("Include uppercase letters? (y/n): ");

    let password = generate_password(
        length,
        include_special_chars,
        include_numbers,
        include_uppercase,
    );

    qrcode::generate_qr_code(&password);

    println!("Generated Password: {}", password);
}

fn generate_password(
    length: usize,
    include_special_chars: bool,
    include_numbers: bool,
    include_uppercase: bool,
) -> String {
    let mut charset = String::from(LOWERCASE);

    if include_special_chars {
        charset.push_str(SPECIAL_CHARS);
    }
    if include_numbers {
        charset.push_str(NUMBERS);
    }
    if include_uppercase {
        charset.push_str(UPPERCASE);
    }

    let mut password = String::new();
    let mut random_bytes = vec![0u8; length];
    getrandom(&mut random_bytes).expect("Failed to get random bytes");

    for byte in random_bytes {
        let index = byte as usize % charset.len();
        password.push(charset.chars().nth(index).unwrap());
    }

    password
}
