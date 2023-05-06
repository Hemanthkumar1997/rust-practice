use std::io;

fn main() {
    println!("Hello, world!");

    println!("enter number");

    let mut num = String::new();

    io::stdin().read_line(&mut num).expect("failed to read line");

    println!("number = {}", num)
}
