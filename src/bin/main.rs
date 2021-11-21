use avl_trees::Tree;

fn main() {
    // Test deletion of every item in the tree
    let items = vec![2, 5, 9, 12, 13, 15, 16, 17, 18, 20, 35, 38];
    for item in items {
        delete_item(item);
    }
}

fn delete_item(item: u8) {
    let mut tree = Tree::new(12);

    tree.insert(5);
    tree.insert(18);

    tree.insert(9);
    tree.insert(2);

    tree.insert(35);

    tree.insert(15);
    tree.insert(13);
    tree.insert(17);
    tree.insert(16);


    tree.insert(38);
    tree.insert(20);
    //tree.insert(21);

    println!("Tree before deletion");
    tree.inorder_tree_walk();
    println!("");

    println!("Element to delete => {}", item);
    tree.delete(item);

    println!("Tree after deletion");
    tree.inorder_tree_walk();
    println!("\n");
}
