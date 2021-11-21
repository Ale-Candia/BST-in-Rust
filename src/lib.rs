use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

struct Node<T>
where
    T: Clone + PartialOrd + Display,
{
    value: T,
    right: Option<Tree<T>>,
    left: Option<Tree<T>>,
    parent: Option<Tree<T>>,
}

impl<T> Node<T>
where
    T: Clone + PartialOrd + Display,
{
    fn new(value: T) -> Self {
        Node {
            value,
            right: None,
            left: None,
            parent: None,
        }
    }

    fn new_with_parent(value: T, parent: SharedPointer<Node<T>>) -> Self {
        Node {
            value,
            right: None,
            left: None,
            parent: Some(Tree { root: parent }),
        }
    }
}

enum ChildType {
    RightChild,
    LeftChild,
    Root,
}

fn get_child_type<T>(u: SharedPointer<Node<T>>) -> ChildType
where
    T: Clone + PartialOrd + Display,
{
    let u_node = (*(u)).borrow();

    match u_node.parent {
        Some(ref var) => {
            let parent_node = (*(var.root)).borrow();

            match parent_node.right {
                Some(ref right_child) => {
                    let right_child_node = (*(right_child.root)).borrow();

                    // If u.parent.right.value = u.value then u is a right child otherwise
                    // it has to be the left child and the parent has two childs
                    if right_child_node.value == u_node.value {
                        ChildType::RightChild
                    } else {
                        ChildType::LeftChild
                    }
                }
                // If the node has a parent, and the parent doesn't have a right child then the
                // node is a left child
                None => ChildType::LeftChild,
            }
        }
        // If the node has no parent then it is a root node
        None => ChildType::Root,
    }
}

type SharedPointer<T> = Rc<RefCell<T>>;

pub struct Tree<T>
where
    T: Clone + PartialOrd + Display,
{
    root: SharedPointer<Node<T>>,
}

fn is_right_child<T>(child: SharedPointer<Node<T>>, y: SharedPointer<Node<T>>) -> bool
where
    T: Clone + PartialOrd + Display,
{
    // Check if the given child is a right child of the node y
    //
    // Considering self and y as nodes they need to satisfy the following
    // Conditions:
    //      self.parent.value == y.value
    //      y.right.value == self.value
    //
    // If the above conditions are true then self is a right child of y

    // Get the node values inorder to compare them
    let child_parent_value: T;
    let y_right_child_value: T;

    let child_node = (*child).borrow();
    let y_node = (*y).borrow();
    match child_node.parent {
        Some(ref child_parent) => {
            let child_parent_node = (*child_parent.root).borrow();
            child_parent_value = child_parent_node.value.clone();
        }
        // If child doesn't have a parent then it isn't a right child of y
        None => { return false; }
    }
    match y_node.right {
        Some(ref y_right_child) => {
            let y_right_child_node = (*y_right_child.root).borrow();
            y_right_child_value = y_right_child_node.value.clone();
        } 
        // If y doesn't have a right child then child isn't its right child
        None => { return false; }
    }

    if child_parent_value == y_node.value && y_right_child_value == child_node.value {
        return true;
    }

    false
}

impl<T> Tree<T>
where
    T: Clone + PartialOrd + Display,
{
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

    fn from(root: SharedPointer<Node<T>>) -> Self {
        Tree {
            root
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
        } else {
            match node.right {
                Some(ref mut child) => child.insert(new_value),
                None => node.right = Some(Tree::new_with_parent(new_value, Rc::clone(&self.root))),
            }
        }
    }

    pub fn delete(&mut self, value: T) {
        // Since we are using Rc, we can delete a node by dropping all the pointers to that node

        match self.get_node(&value) {
            Some(subtree) => {
                // subtree is an Rc to the node we want to delete
                let node = (*(subtree)).borrow();

                match node.left {
                    Some(ref left_child) => {
                        match node.right {
                            Some(ref right_child) => {
                                // Subtree has left and right child
                                let y = right_child.get_minimum();

                                if !is_right_child(Rc::clone(&y), Rc::clone(&subtree)) {
                                    // If y is not the right child of subtree(the node to delete)
                                    let y_node = (*(y)).borrow();
                                    match y_node.right {
                                        Some(ref y_right_child) => {
                                            self.transplant(Rc::clone(&y), Rc::clone(&y_right_child.root));
                                        },
                                        None => {
                                            self.drop_parent_pointer(Rc::clone(&y));
                                        }

                                    }
                                    drop(y_node); // Drop the value to borrow it as mutable

                                    let mut y_node_mut =  (*(y)).borrow_mut();
                                    y_node_mut.right = Some(Tree::from(Rc::clone(&right_child.root)));
                                    
                                    let mut right_child_node = (*right_child.root).borrow_mut();
                                    right_child_node.parent = Some(Tree::from(Rc::clone(&y)));
                                }
                                self.transplant(Rc::clone(&subtree), Rc::clone(&y));
                                let mut y_node = (*(y)).borrow_mut();

                                y_node.left = Some(Tree::from(Rc::clone(&left_child.root)));
                                let mut left_child_node = (*left_child.root).borrow_mut();
                                left_child_node.parent = Some(Tree::from(Rc::clone(&y)));

                            }
                            None => {
                                // Subtree has only left child
                                self.transplant(Rc::clone(&subtree), Rc::clone(&left_child.root));
                            }
                        }
                    }
                    None => {
                        match node.right {
                            Some(ref right_child) => {
                                // Subtree has only right child
                                self.transplant(Rc::clone(&subtree), Rc::clone(&right_child.root));
                            },
                            None => {
                                // Subtree is a leaf just delete the node (make its parent point to
                                // None)
                                self.drop_parent_pointer(Rc::clone(&subtree));
                            }
                        }
                    }
                }
            }
            None => println!("ERROR: Attempt to delete unexisting element {}", value),
        }
    }

    fn drop_parent_pointer(&mut self, u: SharedPointer<Node<T>>) {
        //    Drops the parent pointer thats pointing to u
        //
        // If the u is a root node:
        //     This is a tree deletion(Based on where this function is called in the delete
        //     function)
        //
        match get_child_type(Rc::clone(&u)) {
            ChildType::Root => {
                // delete the tree
                drop(self);
            }
            ChildType::LeftChild => {
                let u_node = (*(u)).borrow();

                if let Some(ref u_parent) = u_node.parent {
                    let mut u_parent_node = (*(u_parent.root)).borrow_mut();
                    u_parent_node.left = None;
                }
            }
            ChildType::RightChild => {
                let u_node = (*(u)).borrow();

                if let Some(ref u_parent) = u_node.parent {
                    let mut u_parent_node = (*(u_parent.root)).borrow_mut();
                    u_parent_node.right = None;
                }
            }
        }
    }

    fn transplant(&mut self, u: SharedPointer<Node<T>>, v: SharedPointer<Node<T>>) {
        //         Make a transplant to delete a node
        //
        // If the u is a root node:
        //     Then make v the root node and set its parent pointer to None
        //
        // Else if u is a left child:
        //     Set v as the left chld of the parent of u
        //
        // Else (if u is a right child):
        //     Set v as the rigth child of the parent of u
        //
        // In each case update the parent node of v, which is the child of u
        match get_child_type(Rc::clone(&u)) {
            ChildType::Root => {
                self.root = Rc::clone(&v);

                let mut v_node = (*(v)).borrow_mut();
                v_node.parent = None;
            }
            ChildType::LeftChild => {
                let u_node = (*(u)).borrow();

                if let Some(ref u_parent) = u_node.parent {
                    let mut u_parent_node = (*(u_parent.root)).borrow_mut();

                    if let Some(ref mut u_parent_left_child) = u_parent_node.left {
                        // Make v the left child of the parent of u
                        u_parent_left_child.root = Rc::clone(&v);
                    }
                    // Update the parent pointer of v to be the parent of u
                    let mut v_node = (*(v)).borrow_mut();
                    if let Some(ref mut v_node_parent) = v_node.parent {
                        v_node_parent.root = Rc::clone(&u_parent.root);
                    }
                }
            }
            ChildType::RightChild => {
                let u_node = (*(u)).borrow();

                if let Some(ref u_parent) = u_node.parent {
                    let mut u_parent_node = (*(u_parent.root)).borrow_mut();

                    if let Some(ref mut u_parent_right_child) = u_parent_node.right {
                        // Make v the right child of the parent of u
                        u_parent_right_child.root = Rc::clone(&v);
                    }
                    // Update the parent pointer of v to be the parent of u
                    let mut v_node = (*(v)).borrow_mut();
                    if let Some(ref mut v_node_parent) = v_node.parent {
                        v_node_parent.root = Rc::clone(&u_parent.root);
                    }
                }
            }
        }
    }

    pub fn inorder_tree_walk(&self) {
        let node = (*(self.root)).borrow();

        if let Some(ref child) = node.left {
            child.inorder_tree_walk();
        }
        print!("{} ", node.value);
        if let Some(ref child) = node.right {
            child.inorder_tree_walk();
        }
    }

    pub fn minimum(&self) -> T {
        let node = (*(self.root)).borrow();

        match node.left {
            Some(ref child) => child.minimum(),
            None => node.value.clone(),
        }
    }

    fn get_minimum(&self) -> SharedPointer<Node<T>> {
        // Returns a shared pointer to the minimum Node
        // in the tree
        let node = (*(self.root)).borrow();

        match node.left {
            Some(ref child) => child.get_minimum(),
            None => Rc::clone(&self.root),
        }
    }

    pub fn maximum(&self) -> T {
        let node = (*(self.root)).borrow();

        match node.right {
            Some(ref child) => child.maximum(),
            None => node.value.clone(),
        }
    }

    pub fn search(&self, value: T) -> Option<T> {
        let node = (*(self.root)).borrow();

        if value == node.value {
            return Some(value);
        }

        if value < node.value {
            match node.left {
                Some(ref child) => {
                    return child.search(value);
                }
                None => return None,
            }
        }

        match node.right {
            Some(ref child) => {
                return child.search(value);
            }
            None => return None,
        }
    }

    fn get_node(&self, value: &T) -> Option<SharedPointer<Node<T>>> {
        // Returns an Rc to the node requested or None
        let node = (*(self.root)).borrow();

        if *value == node.value {
            return Some(Rc::clone(&self.root));
        }

        if *value < node.value {
            match node.left {
                Some(ref child) => {
                    return child.get_node(&value);
                }
                None => return None,
            }
        }

        match node.right {
            Some(ref child) => {
                return child.get_node(&value);
            }
            None => return None,
        }
    }

    pub fn succesor(&self, value: T) -> Option<T> {
        match self.get_node(&value) {
            Some(ref node_pointer) => {
                let node = (*node_pointer).borrow();
                match &node.right {
                    Some(ref right_child) => return Some(right_child.minimum()),

                    None => {
                        let mut y: SharedPointer<Node<T>>;
                        let mut x: SharedPointer<Node<T>>;
                        // This value is used to acces the node information without borrowing y
                        let mut aux: SharedPointer<Node<T>>;

                        match &node.parent {
                            Some(ref node_parent) => {
                                x = Rc::clone(node_pointer);
                                y = Rc::clone(&node_parent.root);
                                aux = Rc::clone(&y);

                                while is_right_child(Rc::clone(&x), Rc::clone(&y)) {
                                    x = Rc::clone(&y);
                                    let y_node = (*aux).borrow();
                                    match &y_node.parent {
                                        Some(ref y_parent) => {
                                            // We are able to mutate y because we use the aux
                                            // variable to evaluate the parent node
                                            y = Rc::clone(&y_parent.root);
                                        }
                                        None => return None,
                                    }
                                    // Drop the value and now update the aux variable
                                    drop(y_node);
                                    aux = Rc::clone(&y);
                                }
                                let y_node = (*y).borrow();
                                return Some(y_node.value.clone());
                            }
                            None => {
                                // The node doesn't have parent and also doesn't have right child
                                // so it doesn't have a succesor
                                return None;
                            }
                        }
                    }
                }
            }
            None => None,
        }
    }
}
