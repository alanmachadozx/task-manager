use std::io;

#[derive(Debug)] 
pub struct Task{
    pub id: String,
    pub text: String,
    pub status: Status
}
#[derive(Debug)]
pub enum Status{
    Todo,
    Doing,
    Done
}

pub fn add_task(tasks: &mut Vec<Task>){
    let mut id = String::new();
    let mut texts = String::new();
    println!("enter a task name");
    io::stdin()
        .read_line(&mut id)
        .expect("Error");
    println!("enter a description");
    io::stdin()
        .read_line(&mut texts)
        .expect("Error");
    tasks.push(Task { 
        id: id, text: texts, status: Status::Todo 
    });
}
pub fn list_tasks(tasks: &Vec<Task>){
    for task in tasks {
        println!("Id:{}, Description{}, Status: {:?}", task.id, task.text, task.status);
    }
}