use std::io;

#[derive(Debug)]
pub struct Task {
    pub id: String,
    pub text: String,
    pub status: Status,
}
#[derive(Debug)]
pub enum Status {
    Todo,
    Doing,
    Done,
}

pub fn add_task(tasks: &mut Vec<Task>) {
    let mut id = String::new();
    let mut texts = String::new();
    let mut num = String::new();
    let mut status = Status::Todo;

    println!("Enter a task name");
    io::stdin().read_line(&mut id).expect("Error");
    println!("Enter a description");
    io::stdin().read_line(&mut texts).expect("Error");
    while 1 == 1 {
        println!("Enter a number Enter a number from 1 to 3, where 1 = Todo, 2 = Doing, 3 = Done");
        io::stdin().read_line(&mut num).expect("Error");
        if num == "1" {
            break;
        }
        if num == "2" {
            status = Status::Doing;
            break;
        }
        if num == "3" {
            status = Status::Done;
            break;
        }
    }

    tasks.push(Task {
        id: id,
        text: texts,
        status: status,
    });
}
pub fn list_tasks(tasks: &Vec<Task>) {
    for task in tasks {
        println!(
            "Id:{}, Description{}, Status: {:?}",
            task.id, task.text, task.status
        );
    }
}
