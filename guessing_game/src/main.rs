use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::Write;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        print!("Please input your guess: ");
        io::stdout().flush().unwrap();
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // Shadowing the previous guess binding.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        println!("You guessed: {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Greater => println!("Too big!"),
        }
    }
}
