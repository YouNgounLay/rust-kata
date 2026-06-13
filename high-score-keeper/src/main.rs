use std::fs;
use rand::Rng;
use std::cmp::Ordering; 
use std::io::{ stdin, stdout, Write };

fn play_guessing_game() -> u32 {
    let mut tries: u32 = 15;
    let secret_number = rand::thread_rng().gen_range(0..=100);
    while tries > 0 {
        let mut guess: String = String::new();
        print!("\nPlease Enter a number: ");
        let _ = stdout().flush();
        stdin().read_line(&mut guess).expect("This should work");
        let guess: u32 = match guess.trim().parse() {
                        Ok(num) => num,
                        Err(_) => {
                            println!("\n!!! Please provide a number !!!");
                            continue;
                        }
                    };
            
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("\n Too Small!!"),
            Ordering::Greater => println!("\n Too Big!!"),
            Ordering::Equal => {
                println!("\nYou Won!!!"); 
                break 
            }
        };
        tries -= 1;
    }
    return tries;
}



fn main() {
    let filename: &str = "score.txt";
    let data = fs::read_to_string(filename)
                .unwrap_or("0".to_string())
                .parse::<u32>()
                .unwrap_or(0);
    let score: u32 = play_guessing_game(); 
    
    if  score > data { 
        fs::write(filename, score.to_string()).expect("Write failed");
        println!("New High Score!");
    }
    
    
}

