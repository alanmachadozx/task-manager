use std::io;

struct Task{
    id: String,
    text: String,
    status: Status
}
enum Status{
    Todo,
    Doing,
    Done
}
pub fn add_task(){
    let mut tasks: Vec<Task> = Vec::new();
    let mut ids = String::new();
    let mut texts = String::new();
    io::stdin()
        .read_line(&mut ids)
        .expect("Error");
    io::stdin()
        .read_line(&mut texts)
        .expect("Error");
    tasks.push(Task { id: ids, text: texts, status: Status::Todo });
}