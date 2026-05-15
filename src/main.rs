use data_structures::tree::Tree;

fn main() {
    let mut tree = Tree::from(12);

    tree.insert(14);
    tree.insert(12);
    tree.insert(16);
    tree.insert(19);

    println!("{:?}", tree.contains(19));
    println!("{}", tree.len());
}
