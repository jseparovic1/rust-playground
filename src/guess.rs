use std::cmp::Ordering;
use std::io;

use rand;
use rand::Rng;

pub fn guess() {
    let number: u32 = rand::thread_rng().gen_range(1, 100);

    println!("Guess the number!");

    loop {
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line...");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&number) {
            Ordering::Less => println!("Too small."),
            Ordering::Greater => println!("Too big."),
            Ordering::Equal => {
                println!("Good game!!");
                break;
            }
        }
    }
}
