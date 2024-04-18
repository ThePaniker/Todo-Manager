use chrono::prelude::*;
use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::io::prelude::*;
use std::fs::File;

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
        if self.list.len() == 0 {
            println!("List Empty!");
        } else {
            println!("Todo list:");
            for (id, (_key, value)) in self.list.iter().enumerate() {
                println!("{id}: {value}");
            }
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
        format!("Todo version 0.3.0")
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
    'main: loop {
        match args[index].trim() {
            "--version" | "-V" => {
                println!("{}", Todo::version());
                break 'main;
            },
            "--help" | "-h" => {
                println!("{}", Todo::help());
                break 'main;
            },
            "-l" => {
                todo.display();
                break 'main;
            },
            "-c" => {
                todo.clear();
                println!("Todo list cleaned!");
                break 'main;
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
                    break 'main;
                } else {
                    let mut _data = String::new();
                    let data = Local::now().to_string();
                    let data: Vec<_> = data.split_whitespace().collect();
                    let time = &data[1].split('.').collect::<Vec<_>>(); 
                    todo.list.insert(format!("{} {}", &data[0], time[0]), _todo);
                    break 'main;
                }
            },
            _ => {
                println!("{}", Todo::help());
                break 'main;
            },
        }
    }
}

<<<<<<< HEAD
fn main() {
=======
fn main() -> Result<(), std::io::Error> {

    let path = Path::new("./todo.txt");
    let display = path.display();

    let file = match File::open(&path) {
        Err(_) => {
            match File::create_new(&path) {
                Err(e) => panic!("Error: {}", e),
                Ok(file) => file,
            }
        },
        Ok(file) => file,
    };
>>>>>>> dev

    let mut todo: Todo = Todo::new();
    for line in std::io::read_to_string(file).unwrap().lines() {
        let line: Vec<&str> = line.split(' ').collect();
        let data = format!("{} {}", &line[0], &line[1]);
        let mut _todo = String::new();
        for i in line[2..].iter() {
            _todo.push_str(i);
            _todo.push(' ');
        }
        todo.list.insert(data, _todo);
    }

    process_args(&mut todo);

    let mut import = match File::create(path) {
        Err(_) => panic!("Couldn't create {}", display),
        Ok(file) => file,
    };

    for (key, value) in todo.list.iter() {
        let string = format!("{key} {value}\n");
        match import.write_all(string.as_bytes()) {
            Ok(_) => (),
            Err(_) => panic!("Couldn't write to {}", display),
        }
    }

    Ok(())
}
