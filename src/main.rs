use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    loop {
        println!("enter number");

        let secret_num: i32 = rand::thread_rng().gen_range(1..5);

        let mut num: String = String::new();

        io::stdin().read_line(&mut num).expect("failed to read line");
        let guessed_num: i32 = num.trim_end().parse().expect("failed to parse");

        println!("secret num = {}", secret_num);
        match guessed_num.cmp(&secret_num) {
            Ordering::Less => println!("Less"),
            Ordering::Greater => println!("More"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }

}
