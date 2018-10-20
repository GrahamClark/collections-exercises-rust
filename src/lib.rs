use std::collections::HashMap;

pub fn mean(v: Vec<i32>) -> f32 {
    let sum = v.iter().fold(0, |acc, x| acc + x);
    sum as f32 / v.len() as f32
}

pub fn pig_latin(s: &str) -> String {
    let (first_char, remainder) = car_cdr(s);
    match first_char.chars().next() {
        Some(c) if is_vowel(c) => format!("{}-hay", s),
        Some(_) => format!("{}-{}ay", remainder, first_char),
        _ => s.to_string()
    }
}

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

fn car_cdr(s: &str) -> (&str, &str) {
    match s.chars().next() {
        Some(c) => s.split_at(c.len_utf8()),
        None => s.split_at(0),
    }
}

fn is_vowel(c: char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U'  => true,
        _ => false
    }
}