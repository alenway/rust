use std::io;
fn main() {
    let mut a: i8 = 10;
    let mut b: i16 = 300;

    println!("a: {}, b: {}", a, b);

    a = 20;
    b = 500;

    println!("updated a: {}, b: {}", a, b);
}
