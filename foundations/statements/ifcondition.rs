use std::io::stdin;

fn main() {
    let mut input = String::new();

    println!("Enter you age: ");

    stdin()
        .read_line(&mut input)
        .expect("Failed to take input.");

    let number: u8 = input.trim().parse().expect("Enter a valid input.");

    if number > 18 {
        println!("elegable to vote: {}", number);
    } else if number == 18 {
        println!("congratulations.");
    } else {
        println!("not elegable to vote.")
    }
}
