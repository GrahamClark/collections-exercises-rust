use std::collections::HashMap;

pub fn staff_to_dept(command: &str, h: HashMap<String, Vec<String>>) -> HashMap<String, Vec<String>> {
    match parse_command(command) {
        Some(x) => apply_command(x, h),
        _ => h
    }
}

enum Command {
    Add,
    Remove
}

fn apply_command(params: (Command, String, String), h: HashMap<String, Vec<String>>) -> HashMap<String, Vec<String>> {
    let (command, name, dept) = params;
    match command {
        Command::Add => add_employee(h, name, dept),
        Command::Remove => h
    }
}

fn add_employee(mut h: HashMap<String, Vec<String>>, name: String, dept: String) -> HashMap<String, Vec<String>> {
    h.entry(dept).or_insert(Vec::new()).push(name);
    h
}

fn parse_command(command: &str) -> Option<(Command, String, String)> {
    let words: Vec<&str> = command.split(' ').collect();
    if words.len() != 4 {
        return None;
    }

    match str_to_command(words[0]) {
        Some(c) => Some((c, words[1].to_string(), words[3].to_string())),
        _ => None
    }
}

fn str_to_command(command: &str) -> Option<Command> {
    match command {
        "Add" => Some(Command::Add),
        "Remove" => Some(Command::Remove),
        _ => None
    }
}
