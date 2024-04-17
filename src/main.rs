use chrono::prelude::*;
use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::fs::{ self, File };

const VALID_FLAGS: [&'static str; 7] = [ "-V", "--version", "-h", "--help", "-l", "-c", "-n"];

struct Todo {
    list: HashMap<String, String>,
}


impl Todo {
    fn new() -> Todo {
        Todo {
            list: HashMap::new(),
        }
    }

    fn display(&self) {
        println!("Todo list:");
        for (id, (_key, value)) in self.list.iter().enumerate() {
            println!("{id}]=> {value}");
        }
    }

    fn clear(&mut self) {
        self.list.clear();
    }

    fn help() -> String {
        format!("usage: todo -n [<args>] 
            [-V | --version] Show current verion
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

fn process_args(todo: &mut Todo) {
    let args = match get_args() {
        Some(v) => v,
        None => {
            eprintln!("{}", Todo::help());
            std::process::exit(0);
        },
    };

    let mut index: usize = 0;
    while index < args.len() {
        match args[index].trim() {
            "--version" | "-V" => {
                println!("{}", Todo::version());
                std::process::exit(0);
            },
            "--help" | "-h" => {
                println!("{}", Todo::help());
                std::process::exit(0);
            },
            "-l" => {
                todo.display();
                std::process::exit(0);
            },
            "-c" => {
                todo.clear();
                println!("Todo list cleaned!");
                std::process::exit(0);
            },
            "-n" => {
                index += 1;
                let mut _todo = String::new();
                while index < args.len() {
                    if VALID_FLAGS.contains(&args[index].trim()) {
                        break;
                    } else {
                        _todo.push_str(&args[index].trim());
                        _todo.push(' ');
                    }
                    index += 1;
                }

                if &_todo == "" {
                    println!("{}", Todo::help());
                    std::process::exit(0);
                } else {
                    todo.list.insert(Local::now().to_string(), _todo);
                    std::process::exit(0);
                }
            },
            _ => (),
        }
    }
}

fn main() {
    // TODO:: Made I/O for todo.txt
    let path = "todo.txt";
    let file = if !fs::File::exists(path) {
        let _ = fs::File::create_new(path);
        fs::File::open(path).unwrap()
    } else {
        fs::File::open(path).unwrap()
    };



    let mut todo: Todo = Todo::new();
    process_args(&mut todo);
}
