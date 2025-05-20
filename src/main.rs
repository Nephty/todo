use std::env;
use todo_bin::{help, Todo};

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let mut dir_path: Option<String> = None;

    println!("Yayy");
    println!("{:?}", args);
    args.retain(|arg| {
        if let Some(path_str) = arg.strip_prefix("--path=") {
            dir_path = Some(path_str.to_string());
            return false;
        }
        true
    });
    println!("Yayy");
    println!("{:?}", args);
    println!("Yayy");

    let todo = Todo::new(dir_path).expect("Couldn't create the todo instance");

    if args.len() > 1 {
        let command = &args[1];
        match &command[..] {
            "list" => todo.list(),
            "add" => todo.add(&args[2..]),
            "rm" => todo.remove(&args[2..]),
            "done" => todo.done(&args[2..]),
            "raw" => todo.raw(&args[2..]),
            "edit" => todo.edit(&args[2..]),
            "sort" => todo.sort(),
            "reset" => todo.reset(),
            "restore" => todo.restore(),
            "help" | "--help" | "-h" | _ => help(),
        }
    } else {
        todo.list();
    }
}
