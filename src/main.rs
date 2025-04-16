// Number Guessing Game
use std::io;

fn main(){
    println!("Guess the number!");
    print!("Please input your guess");
    let mut guess = String::new();

    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");

    println!(" You guessed : {}",guess);
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
