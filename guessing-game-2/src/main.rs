use std::io::{ self, Write, stdin };
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    let mut tries: usize = 15;
    while tries > 0 {
        let mut guess: String = String::new();
        print!("Please Enter a number: ");
        let _ = io::stdout().flush(); 
        stdin().read_line(&mut guess).expect("Should be able to read!");
        
        // Parsing guess from string to u32
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, 
            Err(_e) => {
                println!("\n!!! Please Enter a Numer !!!");
                continue;
            }
        };
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("\nToo Small"), 
            Ordering::Greater => println!("\nToo Big"), 
            Ordering::Equal => { 
                println!("\nYou Won!");
                break;
            }
        };
         
        tries -= 1;
    }
}
