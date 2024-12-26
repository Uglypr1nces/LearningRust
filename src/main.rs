//Guessing game
use rand::Rng;
use std::io;
use std::any::type_name;

fn main() {
    println!("Hello, seletct a number between 1-10!");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    let string_slice: &str = &guess;
    let int_guess = string_slice.parse::<i32>();

    println!("{}", type_of(int_guessP));

    //nono ist ein geiler bastard
    //checkNum(generateNum(), int_guess);
}

fn generateNum() -> i32{
    let mut rng = rand::thread_rng();
    let random_number: i32 = rng.gen_range(0..10);
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