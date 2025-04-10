fn main() {
    let gathering = gollum_tasks("gathering coins", 12);
    println!("{}", gathering);
    
    let cooking = gollum_tasks("cooking", 30);
    println!("{}", cooking);
    
    let hunting = gollum_tasks("hunting", 120);
    println!("{}", hunting);
}

fn gollum_tasks(task_str: &str, time: i32) -> String {
    format!("Gollum has successfully done {} in {} minutes!", task_str, time)
}