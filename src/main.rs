use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main(){
    println!("Guess the number!");
    
    let answer = rand::thread_rng().gen_range(1..=100);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    
    match guess.cmp(&answer){
        Ordering::Equal => println!("You Win!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Less => println!("Too small")
    }
    println!("The answer was {answer}");
    println!("You guessed: {guess}");
}