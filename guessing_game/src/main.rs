use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Hello, player!");

    let secret_number = rand::rng().random_range(1..=100);

    //println!("The secret number is: {secret_number}"); 

    loop {
        println!("Please input your guess (1-100) :)");

        let mut guess = String::new(); 

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("DING DING DING!");
                break;
            }
        }
    }
}

