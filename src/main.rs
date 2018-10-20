extern crate hash_maps;

use std::collections::HashMap;

fn main() {
    let v = vec![1, 45, 73, 32, 18];
    let mean = hash_maps::mean(v);
    println!("mean = {}", mean);

    println!("apple = {}", hash_maps::pig_latin("apple"));
    println!("apple = {}", hash_maps::pig_latin("first"));

    let mut departments :HashMap<String, Vec<String>> = HashMap::new();
    departments = hash_maps::staff_to_dept("Add Sally to Engineering", departments);
    departments = hash_maps::staff_to_dept("Add Amir to Sales", departments);
    departments = hash_maps::staff_to_dept("Add Ben to Engineering", departments);

    println!("{:?}", departments);
}
