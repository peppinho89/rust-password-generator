mod constants;
mod password_generator;
mod password_strength_checker;
mod qrcode;
mod utils;

fn main() {
    print_ascii_art();
    // Ask the user whether they want to generate or check password strength
    println!("Choose an option:");
    println!("1. Generate a password");
    println!("2. Check password strength");

    let option: u32 = utils::get_user_input()
        .trim()
        .parse()
        .expect("Please enter a valid number");

    match option {
        1 => password_generator::generate_password_flow(),
        2 => password_strength_checker::check_password_flow(),
        _ => println!("Invalid option selected. Please restart and choose 1 or 2."),
    }
}

fn print_ascii_art() {
    let ascii_art = r#"
    
  ____            _     ____              _  ____            
  |  _ \ _   _ ___| |_  |  _ \__      ____| |/ ___| ___ _ __  
  | |_) | | | / __| __| | |_) \ \ /\ / / _` | |  _ / _ \ '_ \ 
  |  _ <| |_| \__ \ |_  |  __/ \ V  V / (_| | |_| |  __/ | | |
  |_| \_\\__,_|___/\__| |_|     \_/\_/ \__,_|\____|\___|_| |_|
                                                              
 
    "#;
    println!("{}", ascii_art);
}
