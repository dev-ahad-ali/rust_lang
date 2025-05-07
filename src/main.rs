use gussing_game::Guess;
use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guess the number !");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is : {secret_number}"); // remove in production

    loop {
        println!("Please input your guess !");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // if !(1..=100).contains(&guess) {
        //     println!("The secret number will be between 1 to 100");
        //     continue;
        // };

        let correct_range_guess = Guess::new(guess);

        println!("You guessed : {}", correct_range_guess.value());

        match correct_range_guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too low"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win !");
                break;
            }
        }
    }
}
