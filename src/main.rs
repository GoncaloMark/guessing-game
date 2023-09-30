use std::io;
use rand::Rng;
use std::cmp::Ordering; 
pub fn main() {
    println!("Hello, Guess the Number!");
    let mut rng = rand::thread_rng();
    let n1: u8 = rng.gen_range(0..10);
    
    let mut guess = String::new();

    loop {
        guess.clear();
        io::stdin()
        .read_line(&mut guess)
        .expect("Error Reading Input");

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        match guess.cmp(&n1) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!"); 
                break;
            },
        }
    }
}