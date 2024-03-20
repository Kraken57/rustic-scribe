use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    //println!("x = {x} and y + 2 = {}", y + 2); in rust to change the value of the variable we write it like this with comma

    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");
    
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)              //---read_line has to variants err and ok ----- .except returns these variants accordingly
        .expect("Failed to read line"); 

    let guess: u32 = guess.trim().parse().expect("Please type a number!");  

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}