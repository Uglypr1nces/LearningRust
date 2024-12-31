use std::io;
use rand::Rng;

fn main() {
    let mut player_choice = String::new();

    println!("Welcome to Rock, Paper, Scissors!");
    println!("pick one: rock, paper, scissors");

    let player_choice = loop{
        io::stdin()
        .read_line(&mut player_choice)
        .expect("Failed to read line");

        if player_choice.trim() == "rock" || player_choice.trim() == "paper" || player_choice.trim() == "scissors" {
            break player_choice.trim();
        } else {
            println!("Invalid choice, please try again");
            player_choice.clear();
        }
    };

    println!("You chose: {}", player_choice);
    println!("Computer chose: {}", generateChoice());

    checkWinner(player_choice.to_string(), generateChoice());
}

fn generateChoice() -> String {
    let choices = ["rock", "paper", "scissors"];
    let random_choice = choices[rand::thread_rng().gen_range(0..3)];
    random_choice.to_string()
}

fn checkWinner(x: String, y: String){

    println!("{x}, {y}");
    if x == y {
        println!("It's a tie!");
    } else if x == "rock" && y == "scissors" {
        println!("You win!");
    } else if x == "paper" && y == "rock" {
        println!("You win!");
    } else if x == "scissors" && y == "paper" {
        println!("You win!");
    } else {
        println!("You lose!");
    }
}