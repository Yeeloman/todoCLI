use std::io::{self, Write};

pub fn delete_task(tasks: &mut Vec<String>) {
    io::stdout().flush().unwrap();
    let mut index: String = String::new();
    println!("======= provide the task index =======");
    io::stdin().read_line(&mut index).expect("failed to read index");
    match  index.trim().parse() {
        Ok(index) => {
            tasks.remove(index);
        }
        Err(error) => {
            println!("error: {}", error);
        }
    }
}
