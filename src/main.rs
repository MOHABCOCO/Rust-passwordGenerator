use std::io::{self, Write};
use rand::Rng;

fn main() {
    
    loop{
        let mut length = String::from(""); 
        let mut password = String::from("");
        let all_symbols = String::from("abcdefghijklmnopqrstuvwxyz!@#$%^&*()_+-=><QWERTYUIOPASDFGHJKLZXCVBNM:;");
        println!("Password length: ");
        io::stdin().read_line(&mut length).expect("error (not a number)");
        let int_length : i32 = length.trim().parse().expect("error (not a number)");
        for i in 0..int_length{
            let random_index = rand::thread_rng().gen_range(0..alphabet.len());
            password.push(alphabet.chars().nth(random_index).unwrap());
        println!("Generated password: {}", password);
        print!("\n")
    
    }
}
