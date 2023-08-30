use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    loop{
        println!("Enter the number");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };    

        println!("you Guessed :{guess}");

        match guess.cmp(&secret_number) {
            Ordering::Equal => {println!("Guessed Correct");break;},
            Ordering::Greater => println!("Guessed Greater"),
            Ordering::Less => println!("Guessed Less"),
        }
    }
}