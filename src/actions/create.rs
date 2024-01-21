use std::io::{self, Write};
pub fn create_task(tasks: &mut Vec<String>) {
    let mut task = String::new();
    print!("he new task: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut task).expect("failed to read line");
    let task = task.trim().to_string();
    tasks.push(task);
}
