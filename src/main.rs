use chrono::prelude::*;
use std::collections::HashMap;
use std::env;

enum Error {
    Version,
    Help,
}

impl Error {
    fn to_string(&self) -> String {
        match self {
            Error::Version => "ToDo v0.01, 16.04.24".into(),
            Error::Help => "ToDo is simple tool to create and safe your plans.\n\
                           Using: todo <command> [plan]\n\
                           -n [plan]: Adding new todo to list\n\
                           -l: Current Todo List\n\
                           -c: Clear Todo List\n\
                           -h, --help: show Help\n\
                           -v, --version, -V: show Version"
                .into(),
        }
    }
}

fn main() {
    // TODO: Import/Export to file
    let mut todo: HashMap<String, String> = HashMap::new();
    let args: Vec<String> = env::args().collect();
    let flags: [&str; 7] = ["-h", "--help", "-v", "--version", "-V", "-n", "-l"];
    let mut index: usize = 0;
    while index < args.len() {
        match &args[index].as_str() {
            &"-v" | &"--version" | &"-V" => {
                println!("{}", Error::Version.to_string());
                index += 1;
            }
            &"-h" | &"--help" => {
                println!("{}", Error::Help.to_string());
                index += 1;
            }
            &"-l" => {
                if todo.len() == 0 {
                    println!("Todo List is Empty");
                    index += 1;
                } else {
                    println!("ToDo List:");
                    for (key, value) in &todo {
                        println!("{} >> {}", key, value);
                    }
                    index += 1;
                }
            }
            &"-n" => {
                index += 1;
                let mut _todo: String = String::new();
                while index < args.len() {
                    if flags.contains(&args[index].as_str()) {
                        break;
                    } else {
                        _todo.push_str(&args[index]);
                        _todo.push(' ');
                        index += 1;
                    }
                }
                if &_todo == "" {
                    println!("I can't add empty todo!");
                    println!("Error: {}", Error::Help.to_string());
                    std::process::exit(0);
                } else {
                    todo.insert(Local::now().to_string(), _todo);
                }
            }
            &"-c" => {
                if todo.len() == 0 {
                    println!("Todo List already empty!");
                    index += 1;
                } else {
                    todo.clear();
                    index += 1;
                }
            }
            _ => {
                index += 1;
            }
        }
    }
}
