use rand::Rng;
use std::io;
use std::any::type_name;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new(); //creates a new mutuable (changable) variable and assigns it to be a string

    io::stdin() // allows us to handle user input 
        .read_line(&mut guess) 
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}

fn generateNum() -> i32{
    let random_number = rand::thread_rng().gen_range(0..=10);
    return random_number;
}

fn checkNum(x: i32, y: i32){
    if x == y{
        println!("you won")
    } else {
        println!("wrong guess")
    }
}

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}