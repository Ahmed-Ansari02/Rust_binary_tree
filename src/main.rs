use std::fmt::Display;

struct Node<T: PartialEq + PartialOrd + Display> {
    data: T,
    right: Option<Box<Node<T>>>,
    left: Option<Box<Node<T>>>,
}

impl<T: PartialEq + PartialOrd + Display> Node<T> {
    fn new(data: T) -> Node<T> {
        Node {
            data: data,
            left: None,
            right: None,
        }
    }

    fn add(&mut self, data: T) {
        let mut curr_node = self;
        if curr_node.data < data {
            match &mut curr_node.right {
                Some(val) => val.add(data),
                None => curr_node.right = Some(Box::new(Node::new(data))),
            }
        } else if curr_node.data > data {
            match &mut curr_node.left {
                Some(val) => val.add(data),
                None => curr_node.left = Some(Box::new(Node::new(data))),
            }
        } else {
            return;
        }
    }
    fn get(&self, data: T) -> Option<T> {
        let curr_node = self;
        if curr_node.data < data {
            match &curr_node.right {
                Some(val) => val.get(data),
                None => None,
            }
        } else if curr_node.data > data {
            match &curr_node.left {
                Some(val) => val.get(data),
                None => None,
            }
        } else {
            return Some(data);
        }
    }
}

fn main() {
    let mut tree = Node::new(1);
    for value in [1, 5, 9, 20, 7, 3, 0, 7, 8].iter() {
        tree.add(*value);
    }

    in_order(&tree);
}

fn in_order<T: PartialEq + PartialOrd + Display>(node: &Node<T>) {
    match &node.left {
        Some(left_node) => in_order(&left_node),
        None => (),
    }
    println!("value of node: {}", node.data);
    match &node.right {
        Some(right_node) => in_order(&right_node),
        None => (),
    }
}
