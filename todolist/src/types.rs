use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub description: String, 
    pub done: bool, 
}

pub enum Command {
    Add(String),
    List,
    Done(usize), 
    Quit
}
