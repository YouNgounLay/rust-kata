mod types;
use types::{Task, Command};
use std::io::{ Write, stdin, stdout };

fn add_new_task(description: &str, tasks: &mut Vec<Task>) {
    tasks.push(Task { description: description.to_string(), done: false });
}

fn print_tasks(tasks: &[Task]) {
    for (i, task) in tasks.iter().enumerate() {
        let completion_status: &str = if task.done { "Complete" } else { "Ongoing" };
        println!(
            "{:0>2} ({}):\t{}", 
            i+1, completion_status  , task.description
        );
    }
}

fn print_commands() {
    let commands = [ 
        "ADD new task to the list",
        "LIST all the tasks",
        "DONE a task", 
        "QUIT the program",
    ];
    println!("\nHere are a list of available commands: ");
    for (i, command) in commands.iter().enumerate() {
        println!("{:0>2}:\t{}", i+1, command);
    }
}

fn print_help(tasks: &[Task]) {
    println!("\n");
    print_tasks(tasks);
    print_commands(); 
}


fn complete_task(index: usize, tasks: &mut [Task]) {
    if index > tasks.len(){ 
        return;
    }
    tasks[index].done = true;
}

fn get_user_input(text: &str) -> String {
    print!("{}", text);
    let _ = stdout().flush();
    
    let mut user_input: String = String::new();
    stdin().read_line(&mut user_input).expect("This should work");
    
    user_input 
}


fn parse_command(input: &str) -> Command {
    match input.to_lowercase().trim() {
        "add" => {
            let extra_input: String = get_user_input("\nEnter description: ");    
            if extra_input.is_empty() { 
                println!("!!! Please provide a valid description !!!");
                Command::Quit
            } else {
                Command::Add(extra_input)
            }
        },
        "list" => Command::List,
        "done" => {
            let extra_input: String = get_user_input("\nEnter a task number [1->X]: ");    
            let extra_input: usize = match extra_input.parse() {
                Ok(num) => num, 
                Err(_) =>  {
                    println!("!!! Please provide a valid NUMBER !!!");
                    0 
                }
            };
            
            return if extra_input == 0 {Command::Quit} else {Command::Done(extra_input)}
        },
        "quit" => Command::Quit,
        _ => { 
            println!("\n!!! Unknown Command !!!");
            Command::Quit
        }
    }
}

fn execute_command(command: Command, tasks: &mut Vec<Task>) { 
    match command {
        Command::Add(description) => add_new_task(&description, tasks), 
        Command::List => print_tasks(tasks),
        Command::Done(i) => {
            if i > tasks.len()  {
                println!("\n!!! Invalid Range !!!");
                println!("\n\n\tGood Bye !");
            } else { 
                complete_task(i-1, tasks);
            }
        }
        Command::Quit => { 
            println!("\n\n\tGood Bye !");
        }
    }
}


// Optional

// fn save_state() { }
// fn load_state() { }

fn testing(tasks: &mut Vec<Task>) {
    let sample_tasks: Vec<&str> = vec![
        "No MEFP", "Limited Screentime", "Meditation", 
        "Journaling"
    ];

    for task in sample_tasks.iter() {
        add_new_task(task, tasks); 
    }
    complete_task(1,tasks);
    complete_task(3,tasks);
    
    print_help(tasks);
    let user_input = get_user_input("Enter Command [CHAR]: "); 
    let command: Command = parse_command(&user_input);
    execute_command(command,tasks); 
    execute_command(Command::List, tasks);
}



fn main() {
    let mut tasks: Vec<Task> = vec![];
    testing(&mut tasks);

}
