use::std::io;
mod actions;
use actions::{add_task, Task};

use crate::actions::list_tasks;

pub fn main(){
    let mut tasks: Vec<Task> = Vec::new();
    while 1 == 1 {
        let mut commands = String::new();
        io::stdin()
            .read_line(&mut commands)
            .expect("Error");
            
        match commands.trim(){
            "add" =>{
            add_task(&mut tasks);
        }
            "list" => {
            list_tasks(&tasks);
        }
            _ => {
                println!("Invalid command!");
            }
        }
    }
   
}