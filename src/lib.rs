use colored::*;
use std::{
    env,
    fs::{File, OpenOptions},
    io::{self, BufReader, BufWriter, Read, Write},
    process::exit,
};

pub struct Todo {
    pub todo: Vec<String>,
    pub todo_path: String,
    pub todo_file: File,
}

impl Todo {
    pub fn init() -> Result<Self, String> {
        let todo_path: String = format!(
            "{}/.todo",
            env::var("USERPROFILE").unwrap_or_else(|_| {
                return env::var("HOME").unwrap_or_else(|_| {
                    return ".".to_string();
                });
            })
        );

        let todo_file: File = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .append(true)
            .open(&todo_path)
            .expect("Error while opening todo file");

        let mut buf_reader = BufReader::new(&todo_file);

        let mut contents = String::new();

        buf_reader.read_to_string(&mut contents).unwrap();

        let todo: Vec<String> = contents.lines().map(str::to_string).collect();

        Ok(Todo {
            todo: todo,
            todo_path: todo_path,
            todo_file: todo_file,
        })
    }

    pub fn add(&self, args: &[String]) {
        if args.is_empty() {
            eprint!("Add command must have an argument!");
            exit(1);
        }

        let mut buf_writer = BufWriter::new(&self.todo_file);

        for arg in args {
            if arg.trim().is_empty() {
                continue;
            }
            if arg.trim().len() < 2 {
                eprint!("Too short to be todo!?");
                continue;
            }

            let line = format!("[ ] {}\n", arg);
            buf_writer
                .write_all(line.as_bytes())
                .expect("Unable to add");
        }
    }

    pub fn list(&self) {
        let stdout = io::stdout();

        let mut writer = BufWriter::new(stdout);
        let mut _data = String::new();

        for (number, task) in self.todo.iter().enumerate() {
            let number = (number + 1).to_string().bold();

            let check = &task[..3];
            let task = &task[4..];

            if check == "[*]" {
                _data = format!("{} {}\n", number, task.strikethrough().red());
            } else {
                _data = format!("{} {}\n", number, task.purple());
            }

            writer
                .write_all(_data.as_bytes())
                .expect("Error while writing to stdout");
        }
    }

    pub fn remove(&self, args: &[String]) {
        if args.is_empty() {
            eprint!("TODO remove takes 1 argument!\n");
            exit(1);
        }

        let todo_file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.todo_path)
            .expect("Error while opening todo file!\n");

        let mut buf_writer = BufWriter::new(todo_file);

        // For removing all done todos if not removes given index only
        if args[0] == "done" {
            for line in self.todo.iter() {
                if line.chars().nth(1).unwrap() == '*' {
                    continue;
                }
                let line = format!("{}\n", line);

                buf_writer
                    .write_all(line.as_bytes())
                    .expect("Error while removing TODO\n");
            }
        } else {
            for (pos, line) in self.todo.iter().enumerate() {
                if args[0] == (pos + 1).to_string() {
                    continue;
                }
                let line = format!("{}\n", line);

                buf_writer
                    .write_all(line.as_bytes())
                    .expect("Error while removing TODO\n");
            }
        }
    }

    pub fn done(&self, args: &[String]) {
        if args.is_empty() {
            eprint!("TODO done takes 1 argument!\n");
            exit(1);
        }

        let todo_file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.todo_path)
            .expect("Error while opening todo file!\n");

        let mut buf_writer = BufWriter::new(todo_file);

        for (pos, line) in self.todo.iter().enumerate() {
            if args[0] == (pos + 1).to_string() {
                let line = format!("[*] {}\n", &line[4..]);

                buf_writer
                    .write_all(line.as_bytes())
                    .expect("Error while checking TODO!\n");
                continue;
            }
            let line = format!("{}\n", line);

            buf_writer
                .write_all(line.as_bytes())
                .expect("Error while checking TODO!\n");
        }
    }

    pub fn edit(&self, args: &[String]) {
        if args.len() != 2 {
            eprint!("Edit command excepts only one pair of argument!\n");
            exit(1);
        }

        let todo_file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.todo_path)
            .expect("Error while opening todo file!\n");

        let mut buf_writer = BufWriter::new(todo_file);

        for (pos, line) in self.todo.iter().enumerate() {
            if args[0] == (pos + 1).to_string() {
                let line = format!("[ ] {}\n", args[1]);

                buf_writer
                    .write_all(line.as_bytes())
                    .expect("Error while editing TODO!\n");

                continue;
            }

            let line = format!("{}\n", line);

            buf_writer
                .write_all(line.as_bytes())
                .expect("Error while editing TODO!\n");
        }
    }

    pub fn reset(&self) {
        let todo_file: File = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.todo_path)
            .expect("Error while opening todo file");

        let mut buf_writer = BufWriter::new(todo_file);

        buf_writer
            .write_all("".as_bytes())
            .expect("Error while reseting TODO!");
    }

    pub fn sort(&mut self) {
        let mut done: Vec<String> = Vec::new();
        let mut undone: Vec<String> = Vec::new();

        for task in &self.todo {
            if task.starts_with("[*]") {
                done.push(task.clone());
            } else {
                undone.push(task.clone());
            }
        }

        undone.sort();
        done.sort();

        self.todo = undone;
        self.todo.extend(done);

        let todo_file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.todo_path)
            .expect("Error while opening todo file!");

        let mut buf_writer = BufWriter::new(todo_file);

        for task in &self.todo {
            let line = format!("{}\n", task);
            buf_writer
                .write_all(line.as_bytes())
                .expect("Error while writing sorted TODOs to file!");
        }
    }

    pub fn filter(&self, args: &[String]) {
        if args.len() == 1 {
            eprintln!("Filter command accepts one argument (done, undone)\n")
        }

        let stdout = io::stdout();

        let mut writer = BufWriter::new(stdout);
        let mut _data = String::new();

        for (number, task) in self.todo.iter().enumerate() {
            let number = (number + 1).to_string().bold();

            let check = &task[..3];
            let task = &task[4..];

            if args[0] == "done" {
                if check == "[*]" {
                    _data = format!("{} {}\n", number, task.strikethrough().red());
                    writer
                        .write_all(_data.as_bytes())
                        .expect("Error while writing to stdout");
                }
            } else if args[0] == "undone" {
                if check == "[ ]" {
                    _data = format!("{} {}\n", number, task.purple());
                    writer
                        .write_all(_data.as_bytes())
                        .expect("Error while writing to stdout");
                }
            }
        }
    }

    pub fn print_help(&self) {
        println!("Available commands:");
        println!("  add <task>         - Add a new task");
        println!("  done <index>       - Mark a task as done by its index");
        println!("  remove <index|done> - Remove a task by its index or all done tasks");
        println!("  edit <index> <task> - Edit a task by its index");
        println!("  filter <done|undone> - Filter tasks by done or undone");
        println!("  list               - List all tasks");
        println!("  reset              - Reset the todo list");
        println!("  sort               - Sort tasks by undone and done");
        println!("  help               - Show this help message");
    }
}
