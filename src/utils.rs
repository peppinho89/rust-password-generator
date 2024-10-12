use std::io;

pub fn ask_yes_no(prompt: &str) -> bool {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    matches!(input.trim().to_lowercase().as_str(), "y" | "yes")
}

pub fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input
}
