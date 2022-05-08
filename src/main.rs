use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the fcken number");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("Secret Number is {}",secret_number);

    loop{
        println!("Please input your guess...");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number...");
                continue;
            },
        };

        // println!("You guessed {}",guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("Nice , You Guessed it!");
                break;
            },
        }
    }
    
}
