use avl_trees::Tree;

fn main() {
    let mut tree = Tree::new(6);

    tree.insert(7);
    tree.insert(8);
    tree.insert(5);
    tree.insert(5);
    tree.insert(2);

    tree.inorder_tree_walk();
}
