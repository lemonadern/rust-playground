use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, "hi");
    println!("{:?}", map);
    println!("Hello, world!");

    println!("{:?}", map.get(&1));
}
