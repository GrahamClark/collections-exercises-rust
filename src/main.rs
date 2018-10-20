extern crate collections_exercises;

use std::collections::HashMap;

fn main() {
    let v = vec![1, 45, 73, 32, 18];
    let mean = collections_exercises::mean(v);
    println!("mean = {}", mean);

    println!("apple = {}", collections_exercises::pig_latin("apple"));
    println!("apple = {}", collections_exercises::pig_latin("first"));

    let mut departments :HashMap<String, Vec<String>> = HashMap::new();
    departments = collections_exercises::staff_to_dept("Add Sally to Engineering", departments);
    departments = collections_exercises::staff_to_dept("Add Amir to Sales", departments);
    departments = collections_exercises::staff_to_dept("Add Ben to Engineering", departments);

    println!("{:?}", departments);
}
