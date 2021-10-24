use std::rc::Rc;
use std::cell::RefCell;
use std::fmt::Display;

struct Node<T: PartialOrd + Display> {
    value: T,
    right: Option<Tree<T>>,
    left: Option<Tree<T>>,
    parent: Option<Tree<T>>,
}

impl<T: PartialOrd + Display> Node<T> {
    fn new(value: T) -> Self {
        Node{
            value, 
            right: None,
            left: None,
            parent: None,
        }
    }

    fn new_with_parent(value: T, parent: SharedPointer<Node<T>>) -> Self {
        Node{
            value, 
            right: None,
            left: None,
            parent: Some(Tree{
                root: parent,
            }),
        }
    }
}

type SharedPointer<T> = Rc<RefCell<T>>;

pub struct Tree<T: PartialOrd + Display> {
    root: SharedPointer<Node<T>>,
}

impl<T: PartialOrd + Display> Tree<T> {
    pub fn new(value: T) -> Self {
        Tree {
            root: Rc::new(RefCell::new(Node::new(value))),
        }
    }
    
    fn new_with_parent(value: T, parent: SharedPointer<Node<T>>) -> Self {
        Tree {
            root: Rc::new(RefCell::new(Node::new_with_parent(value, parent))),
        }
    }

    pub fn insert(&mut self, new_value: T) {
        // Get the node inside the root pointe (borrow() returns a ref<'_, Node<T>>)
        let mut node = (*(self.root)).borrow_mut();

        // If the newly inserted value is smaller than the value of the current node insert it in
        // the left subtree, otherwise insert it in the right subtree
        if node.value > new_value {
            match node.left {
                Some(ref mut child) => child.insert(new_value),
                None => node.left = Some(Tree::new_with_parent(new_value, Rc::clone(&self.root))),
            }
        }
        else {
            match node.right {
                Some(ref mut child) => child.insert(new_value),
                None => node.right = Some(Tree::new_with_parent(new_value, Rc::clone(&self.root))),
            }
        }
    }

    pub fn inorder_tree_walk(&self) {
        let node = (*(self.root)).borrow();
        
        if let Some(ref child) = node.left {
            child.inorder_tree_walk();
        }
        println!("{} ", node.value);
        if let Some(ref child) = node.right {
            child.inorder_tree_walk();
        }
    }
}
