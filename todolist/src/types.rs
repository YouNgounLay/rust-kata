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
