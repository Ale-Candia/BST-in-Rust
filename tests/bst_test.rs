use avl_trees::Tree;

#[test]
fn search_existing_element() {
    let mut bst = Tree::new(15);

    bst.insert(18);
    bst.insert(17);
    bst.insert(20);
    bst.insert(6);
    bst.insert(3);
    bst.insert(7);
    bst.insert(13);
    bst.insert(9);
    bst.insert(4);
    bst.insert(2);

    assert_eq!(bst.search(4), Some(4));
}

#[test]
fn search_unexisting_element() {
    let mut bst = Tree::new(15);

    bst.insert(18);
    bst.insert(17);
    bst.insert(20);
    bst.insert(6);
    bst.insert(3);
    bst.insert(7);
    bst.insert(13);
    bst.insert(9);
    bst.insert(4);
    bst.insert(2);

    assert_eq!(bst.search(25), None);
}

#[test]
fn minimum() {
    let mut bst = Tree::new(15);

    bst.insert(18);
    bst.insert(17);
    bst.insert(20);
    bst.insert(6);
    bst.insert(3);
    bst.insert(7);
    bst.insert(13);
    bst.insert(9);
    bst.insert(4);
    bst.insert(2);

    assert_eq!(bst.minimum(), 2);
}

#[test]
fn maximum() {
    let mut bst = Tree::new(15);

    bst.insert(18);
    bst.insert(17);
    bst.insert(20);
    bst.insert(6);
    bst.insert(3);
    bst.insert(7);
    bst.insert(13);
    bst.insert(9);
    bst.insert(4);
    bst.insert(2);

    assert_eq!(bst.maximum(), 20);
}

#[test]
fn minimum_root() {
    let bst = Tree::new(15);

    assert_eq!(bst.minimum(), 15);
}

#[test]
fn maximum_root() {
    let bst = Tree::new(15);

    assert_eq!(bst.maximum(), 15);
}

#[test]
fn test_succesors() {
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

    assert_eq!(Some(5), tree.succesor(2));
    assert_eq!(Some(15), tree.succesor(13));
    assert_eq!(Some(16), tree.succesor(15));
    assert_eq!(Some(12), tree.succesor(9));
    assert_eq!(None, tree.succesor(38));
}
