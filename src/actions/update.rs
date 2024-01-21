use std::io::{self, Write};

pub fn update_task(tasks: &mut Vec<String>) {
    io::stdout().flush().unwrap();
    let mut task: String = String::new();
    let mut index: String = String::new();
    println!("==== provide the task index ====");
    io::stdin().read_line(&mut index).expect("failed to read index");
    match index.trim().parse::<usize>() {
        Ok(index) => {
            println!("the old task: {}", tasks[index]);
            println!("the new task");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut task).expect("failed to read line");
            tasks[index] = task;
        }
        Err(error) => {
            println!("error: {}", error);
        }
    }}
