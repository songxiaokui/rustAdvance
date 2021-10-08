extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess a number!");
    // 生成随机数
    let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("the secret_number is {}", secret_number);
    let mut number = 0;
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed type read line.");
        number += 1;
        let guess: u32 = match guess.trim().parse() {
            Ok(guess) => guess,
            Err(_) => continue,
        };
        println!("you guess this number is {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You Win! you guess {} count！", number);
                break;
            }
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!")
        };
    };
    println!("Game Over!")
}
