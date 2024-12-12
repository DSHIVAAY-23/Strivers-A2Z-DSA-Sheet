#[derive(Debug)]
struct Node {
    data: i32,
    next: Option<Box<Node>>,
}

#[derive(Debug)]
struct LinkedList {
    head: Option<Box<Node>>,
}

impl LinkedList {
    // Create a new empty LinkedList
    fn new() -> LinkedList {
        LinkedList { head: None }
    } // Insert a new element at the head of the list     
    fn insert_at_head(&mut self, data: i32) {          
        let new_node = Box::new(Node {
            data,
            next: self.head.take(),
        });
        self.head = Some(new_node); }
     pub fn pop(&mut self) -> Option<i32> {
        match self.head.take() {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.elem)
            }}}}// Print the entire linked list
 fn print_list(&self) {
    let mut curr = &self.head;
    while let Some(ref node) = curr {
        print!("{} -> ", node.data);
        curr = &node.next;
    } println!("None");}
fn main() {
    let mut list = LinkedList::new();

    // Insert elements
    list.insert_at_head(10);
    list.print_list();
}