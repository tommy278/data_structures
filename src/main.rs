use data_structures::hash_map::HashMap;

fn main() {
    let mut table = HashMap::new();

    table.insert(12, "Hello");

    print!("{:?}", table.get(12));
}
