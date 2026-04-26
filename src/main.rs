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

pub fn main(){
    let tasks: Vec<Task> = Vec::new();
}