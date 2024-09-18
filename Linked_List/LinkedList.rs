use std::fmt;

// Define a Node struct in Rust
struct Node {
    data: i32,
    next: Option<Box<Node>>,  // Use Option<Box<Node>> to handle the pointer to the next node
}

// Implement functions for the Node struct
impl Node {
    // Constructor to create a new Node
    fn new(data: i32) -> Node {
        Node {
            data,
            next: None,  // Initially, the next pointer is None (null)
        }
    }
}

// Implementing the Display trait for pretty printing
impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Node {{ data: {}, next: {:?} }}", self.data, self.next)
    }
}

fn main() {
    // Create a new Node with data = 10
    let y = Box::new(Node::new(10));

    // Print the data and next fields of the node
    println!("Node data: {}", y.data);
    println!("Next node: {:?}", y.next);  // Print None since there's no next node
}
