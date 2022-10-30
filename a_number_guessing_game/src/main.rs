
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("-----------Guess the Number-----------");
    println!("Please input your guess:");

    let rand_number = rand::thread_rng().gen_range(1..=100);
    println!("The Secret Number Is: {}",rand_number);
    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
    .expect("Failed to read the line");

    

    println!("Your guess is: {}", guess);

    match guess.cmp(&rand_number){
        Ordering::Equal => println!("To large"),
        Ordering::Greater => println!("To big"),
        Ordering::Less => println!("To small"),
    }
}
