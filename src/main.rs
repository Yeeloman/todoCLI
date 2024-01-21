use std::io::{self, Read, Write};
use std::fs::{File, OpenOptions};
mod actions;
use actions::update::update_task;
use actions::delete::delete_task;
use actions::create::create_task;
use actions::list::list_task;
use actions::help::help_func;

fn main() {
    let mut tasks: Vec<String> = Vec::new();
    tasks = load_tasks().unwrap_or_else(|err| {
        eprintln!("Error loading tasks: {}", err);
        vec![]
    });
    help_func();
    loop {
        io::stdout().flush().unwrap();
        let mut action = String::new();
        println!("=============== action ===============");
        io::stdin().read_line(&mut action).expect("failed to read line");
        println!("=============== ====== ===============");

        let action = action.trim();
        if action == "quit" {
            save_tasks(&tasks).expect("Error saving tasks");
            println!("see you later!");
            break;
        }
        match action {
            "create" => create_task(&mut tasks),
            "list" => list_task(&tasks),
            "update" => update_task(&mut tasks),
            "delete" => delete_task(&mut tasks),
            "help" => help_func(),
            _ => println!("not a valid action"),
        }
    }
}

fn load_tasks() -> Result<Vec<String>, io::Error> {
    let mut file = File::open("tasks.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.lines().map(String::from).collect())
}

fn save_tasks(tasks: &[String]) -> Result<(), io::Error> {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("tasks.txt")?;
    
    for task in tasks {
        writeln!(file, "{}", task)?;
    }
    
    Ok(())
}
