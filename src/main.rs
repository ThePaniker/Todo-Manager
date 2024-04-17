use chrono::prelude::*;
use std::collections::HashMap;
use std::env;

struct Todo {
    list: HashMap<String, String>,
}

impl Todo {

    fn display(&self) {
        println!("Todo list:");
        for (id, (key, value)) in self.list.iter().enumerate() {
            println!("{id}]=> {value}");
        }
    }

    fn clear(&mut self) {
        self.list.clear();
    }

    fn help() -> String {
        format!("usage: todo [<args>] 
            [-v | --version] Show current verion
            [-h | --help] Show manual
            [-l ] Show full list
            [-c ] Clear list")
    }

    fn version() -> String {
        format!("Todo version 0.2.0")
    }
}

fn get_args() -> Option<Vec<String>> {
    let args: Vec<String> = env::args().collect();
    let args = &args[1..];
    if args.len() == 0 { return None; }
    Some(args.to_vec())
}

fn process_args() {
    let args = match get_args() {
        Some(v) => v,
        None => {
            eprintln!("{}", Todo::help());
            std::process::exit(0);
        },
    };

    let mut index: usize = 0;
    while index < args.len() {
    }
}

fn main() {
    process_args();


 //   let mut todo: HashMap<String, String> = HashMap::new();
 //   let args: Vec<String> = env::args().collect();
 //   let flags: [&str; 7] = ["-h", "--help", "-v", "--version", "-V", "-n", "-l"];
 //   let mut index: usize = 0;
 //   while index < args.len() {
 //       match &args[index].as_str() {
 //           &"-v" | &"--version" | &"-V" => {
 //               println!("{}", Error::Version.to_string());
 //               index += 1;
 //           }
 //           &"-h" | &"--help" => {
 //               println!("{}", Error::Help.to_string());
 //               index += 1;
 //           }
 //           &"-l" => {
 //               if todo.len() == 0 {
 //                   println!("Todo List is Empty");
 //                   index += 1;
 //               } else {
 //                   println!("ToDo List:");
 //                   for (key, value) in &todo {
 //                       println!("{} >> {}", key, value);
 //                   }
 //                   index += 1;
 //               }
 //           }
 //           &"-n" => {
 //               index += 1;
 //               let mut _todo: String = String::new();
 //               while index < args.len() {
 //                   if flags.contains(&args[index].as_str()) {
 //                       break;
 //                   } else {
 //                       _todo.push_str(&args[index]);
 //                       _todo.push(' ');
 //                       index += 1;
 //                   }
 //               }
 //               if &_todo == "" {
 //                   println!("I can't add empty todo!");
 //                   println!("Error: {}", Error::Help.to_string());
 //                   std::process::exit(0);
 //               } else {
 //                   todo.insert(Local::now().to_string(), _todo);
 //               }
 //           }
 //           &"-c" => {
 //               if todo.len() == 0 {
 //                   println!("Todo List already empty!");
 //                   index += 1;
 //               } else {
 //                   todo.clear();
 //                   index += 1;
 //               }
 //           }
 //           _ => {
 //               index += 1;
 //           }
 //       }
 //   }
}
