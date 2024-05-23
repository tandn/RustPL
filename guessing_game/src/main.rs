use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number (1-100)");
    let secret_num = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess_str = String::new();

        io::stdin()
            .read_line(&mut guess_str)
            .expect("Failed to read line");

        let guess: u32 = match guess_str.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_num) {
            Ordering::Greater => println!("Too big"),
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {println!("Correct!"); break;},
        }
    }
}
