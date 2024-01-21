pub fn list_task(tasks: &Vec<String>) {
    println!("list of tasks");
    for (index, task) in tasks.iter().enumerate() {
        println!("================= {} ==================", index);
        println!("-> {}", task);
    }
}
