use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter a integer.");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");

    let mut number: u8 = input.trim().parse().expect("Enter valid input.");

    println!("num value: {}", number);
}
