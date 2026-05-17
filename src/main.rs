use data_structures::tree::Tree;

fn main() {
    let mut tree = Tree::from(12);

    tree.insert(10);
    tree.insert(13);
    tree.insert(100);
    tree.insert(2);
}
