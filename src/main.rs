extern crate hash_maps;

fn main() {
    let v = vec![1, 45, 73, 32, 18];
    let mean = hash_maps::mean(v);
    println!("mean = {}", mean);

    println!("apple = {}", hash_maps::pig_latin("apple"));
    println!("apple = {}", hash_maps::pig_latin("first"));
}
