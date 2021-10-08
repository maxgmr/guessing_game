//Guessing game program
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("I'm thinking of a number between 1 and 100...");

    let secret_number = rand::thread_rng().gen_range(1..101);
    let mut guess_count = 1;

    loop {
        println!("Please input your guess:");

        let mut guess = String::new(); //Create new mutable String guess by calling the "new" function of the type String.

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() { //u32 = parse string as unsigned 32-bit integer
            Ok(num) => num,
            Err(_) => continue
        };

        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Higher!"),
            Ordering::Greater => println!("Lower!"),
            Ordering::Equal => {
                println!("Yes, the number was {}! You win!", secret_number);
                println!("You needed {} tries to guess the number.", guess_count);
                break;
            }
        }

        guess_count += 1;
    }
}
