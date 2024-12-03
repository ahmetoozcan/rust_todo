use rust_todo::Todo;
use std::env;

fn main() {
    let mut todo: Todo = Todo::init().expect("Error while creating TODO instance");

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let arg = &args[1];

        match &arg[..] {
            "add" => todo.add(&args[2..]),
            "list" => todo.list(),
            "reset" => todo.reset(),
            "remove" => todo.remove(&args[2..]),
            "edit" => todo.edit(&args[2..]),
            "done" => todo.done(&args[2..]),
            "sort" => todo.sort(),
            "raw" => todo.filter(&args[2..]),
            "help" => todo.print_help(),
            _ => {
                eprintln!("Unknown command");
                todo.print_help();
            }
        }
    }
}
