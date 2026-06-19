// use std::fs;
use std::error::Error;
use crossterm::terminal::{ enable_raw_mode, disable_raw_mode };
use crossterm::event::{ read, Event, KeyCode };  // This is used for handling keyevent read
use crossterm::terminal::size;


/*
fn read_file(filename: &str) -> Result<String, Box<dyn Error>> { 
    let data: String = fs::read_to_string(filename)?;
    println!("{}", data);
    Ok(data)
}

fn testing() {
    let filename: &str = "data.txt";
    let _ = read_file(filename);
}
*/


fn main() -> Result<(), Box<dyn Error>> {
    let (col, row) = size()?;
    println!("Col: {}\tRow: {}", col, row);
    
    enable_raw_mode()?; 
    'main_loop: loop {
    match read()? { 
        Event::Key(key_event) => {
            match key_event.code { 
                KeyCode::Char('q') => { 
                    println!("Q pressed! ... Exiting"); 
                    break 'main_loop; 
                },
                KeyCode::Enter => println!("Enter Pressed!"),
                KeyCode::Up => println!("Up Pressed!"),
                KeyCode::Down  => println!("Down Pressed!"),
                KeyCode::Left => println!("Left Pressed!"),
                KeyCode::Right => println!("Right Pressed!"),
                KeyCode::CapsLock => println!("CapsLock Pressed!"),
                _ => println!("Hello World!"),
            };
        },
        _ => println!("Hello World!"),
    };
    }
    
    disable_raw_mode()?;
    Ok(())
}
