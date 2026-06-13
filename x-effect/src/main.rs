// use std::io::{ stdin };
// use std::env;
use std::fs;

fn read_from_file(filename: &str) -> Vec<String> { 
    let data: Vec<String> = Vec::<String>::new();
    let contents = fs::read_to_string(filename)
                            .expect("File Reading Error");
    for line in &mut contents.lines() {
        println!("{}", line);
    }
    return data;
}

/*
fn get_user_input() -> String {
    let mut user_input = String::new();
    println!("Enter your name: ");
    stdin().read_line(&mut user_input)
        .expect("Failed to read line");
    return user_input.trim().to_string();
}
*/


fn main() {
    let _: Vec<String> = read_from_file("data.txt");
    
}
