// use std::fs;
use std::io::{self, Write, stdout};
use std::error::Error;
use crossterm::terminal::{ enable_raw_mode, disable_raw_mode };
use crossterm::event::{ read, Event, KeyCode };  // This is used for handling keyevent read
use crossterm::terminal::size;
use crossterm::{execute, cursor, terminal::{Clear, ClearType}}; 


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

fn clear_screen() -> io::Result<()> {
    execute!(io::stdout(), Clear(ClearType::All))?;
    Ok(())
}


fn main() -> Result<(), Box<dyn Error>> {
    let (col, row) = size()?;
    println!("Col: {}\tRow: {}", col, row);
    
    enable_raw_mode()?; 
    let _ = clear_screen();
    let mut stdout = stdout();
    execute!(stdout, cursor::MoveTo(5,5)).unwrap();    
    execute!(stdout, cursor::MoveTo(10,10)).unwrap();    
    execute!(stdout, cursor::MoveTo(15,15)).unwrap();    


    'main_loop: loop {
    match read()? { 
        Event::Key(key_event) => {
            match key_event.code { 
                KeyCode::Char('q') => { 
                    println!("Q pressed! ... Exiting"); 
                    break 'main_loop; 
                },
                KeyCode::Enter => println!("Enter Pressed!"),
                KeyCode::Up => execute!(stdout, cursor::MoveTo(0, 0)).unwrap(),
                KeyCode::Down  => execute!(stdout, cursor::MoveTo(0,5)).unwrap(),
                KeyCode::Left => execute!(stdout, cursor::MoveTo(5,0)).unwrap(),
                KeyCode::Right => execute!(stdout, cursor::MoveTo(10,0)).unwrap(),
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
