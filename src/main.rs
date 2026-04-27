use::std::io;
mod actions;
use actions::add_task;

pub fn main(){
    let mut commands = String::new();
    while true {
        io::stdin()
            .read_line(&mut commands)
            .expect("Error");
        if commands == "add"{
            add_task();
        }
    }
   
}