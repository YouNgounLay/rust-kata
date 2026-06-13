use std::io::stdin;   
use std::cmp::Ordering;
use rand::Rng;


fn main() {
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    
    let mut tries: usize = 15;
    while tries > 0 {
        println!("\nTries: {}", tries);
        println!("Please input your guess.");
        let mut guess = String::new(); 
        stdin().read_line(&mut guess)
            .expect("Please input a valid input");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, 
            Err(_) => { println!("Please provide a number"); continue; }
        };
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => { 
                println!("Correct"); 
                break;
            },

        }
        
        tries -= 1;
    }
    
}
