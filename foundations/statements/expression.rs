use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter your age: ");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to take input");

    let age: u8 = input.trim().parse().expect("Enter a valid number");

    let status = if age >= 18 { "Adult" } else { "Minor" };

    println!("Status: {}", status);
}
