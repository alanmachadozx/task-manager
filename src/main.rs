use::std::io;
mod actions;
use actions::{add_task, Task};

pub fn main(){
    let mut tasks: Vec<Task> = Vec::new();
    let mut commands = String::new();
    while 1 == 1 {
        io::stdin()
            .read_line(&mut commands)
            .expect("Error");
        if commands == "add"{
            add_task(&mut tasks);
        }
    }
   
}