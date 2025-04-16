// Number Guessing Game
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}

// Basics

// fn main() {
//     let ans : u32 = sum(1,2);
//     println!("{}",ans);
//     println!("{}",even(2));

// }

// fn sum(a:u32,b:u32) -> u32{
//     return a+b;
// }
// fn even(a:u32) -> bool{
//     return a%2==0;
// }
