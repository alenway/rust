pub fn variables() {
    println!("Variables:");
    let logical: bool = true;
    let a_float: f64 = 23.324;
    let an_integer = 234;

    let mut mutable = 255;

    println!("{}", mutable);
    mutable = 666;
    println!("{}", mutable);
    let mutable = true;
    println!("{}", mutable);

    println!("{}", logical);
    println!("{}", a_float);
    println!("{}", an_integer);
}
