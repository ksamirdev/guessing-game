use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    guess_game()
}

fn guess_game() {
    loop {
        println!("Input your number.");

        let rand_number = rand::thread_rng().gen_range(1..101);

        println!("nUmBeR iS nOt {}", rand_number);

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&rand_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Equal => {
                println!("{}", "GGs, EZ You win!".green());
                break;
            }
            Ordering::Greater => println!("{}", "Too big!".red()),
        }
    }
}
