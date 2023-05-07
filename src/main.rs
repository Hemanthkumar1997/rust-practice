use rand::Rng;
use std::io;

fn main() {
    println!("enter number");

    let secret_num = rand::thread_rng().gen_range(1..101);

    let mut num = String::new();

    io::stdin().read_line(&mut num).expect("failed to read line");

    println!("number guessed = {}", num);

    println!("real number = {}", secret_num);
}
