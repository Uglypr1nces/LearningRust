use std::io;
use std::string;

fn main() {
    println!("what shall i calculate for you dumb sir?:");
    let mut calculation = String::new();
    let mut first_number = String::new();
    let mut second_number = String::new();
    let mut operator = String::new();

    loop{
        io::stdin()
            .read_line(&mut calculation)
            .expect("could not read line please try again");

        if calculation.contains("+") | calculation.contains("-") | calculation.contains("*") | calculation.contains("/"){
            break;
        }
        else{
            println!("not a valid calculation")
        }
    }
}


