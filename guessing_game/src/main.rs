use std::io;

fn main() {

    //println!("x = {x} and y + 2 = {}", y + 2); in rust to change the value of the variable we write it like this with comma

    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)              //---read_line has to variants err and ok ----- .except returns these variants accordingly
        .expect("Failed to read line"); 

    println!("You guessed: {guess}");
}