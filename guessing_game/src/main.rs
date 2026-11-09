use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("guess a number!");

    let secret_num = rand::thread_rng().gen_range(1..=100);
    println!("secret number is:{secret_num}");

    loop {
        println!("input a num");
        
        let mut guess = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("read line fail");

        let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
        };
    
        println!("your guess: {guess}");

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too large"),
            Ordering::Equal => {
                println!("you win");
                break;
            }
        }
    }
}
