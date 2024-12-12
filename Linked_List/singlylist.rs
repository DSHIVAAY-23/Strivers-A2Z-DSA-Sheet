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
    }

    // Insert a new element at the head of the list
    fn insert_at_head(&mut self, data: i32) {
        let new_node = Box::new(Node {
            data,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match self.head.take() {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    // Insert a new element at the tail of the list
    fn insert_at_tail(&mut self, data: i32) {
        let new_node = Box::new(Node { data, next: None });

        // If the list is empty, set the new node as the head
        if self.head.is_none() {
            self.head = Some(new_node);
            return;
        }

        // Otherwise, traverse to the end of the list
        let mut current = &mut self.head;
        while let Some(ref mut node) = current {
            if node.next.is_none() {
                node.next = Some(new_node); // Set the new node at the end
                return;
            }
            current = &mut node.next;
        }
    }
 // Insert a new element at a specific position (1-based index)
 fn insert_at_position(&mut self, data: i32, position: usize) {
    if position == 1 {
        // Insert at head if position is 1
        self.insert_at_head(data);
        return;
    }

    let mut new_node = Box::new(Node { data, next: None });
    let mut current = &mut self.head;
    let mut index = 1;

    // Traverse the list until we find the position to insert
    while let Some(ref mut node) = current {
        if index == position - 1 {
            new_node.next = node.next.take(); // Set new node's next to the current node's next
            node.next = Some(new_node);       // Insert the new node
            return;
        }
        current = &mut node.next;
        index += 1;
    }

    // If the position is greater than the length of the list, do nothing
    if position > index {
        println!("Position {} is out of bounds!", position);
    }
}
    
    // Print the entire linked list
    fn print_list(&self) {
        let mut curr = &self.head;
        while let Some(ref node) = curr {
            print!("{} -> ", node.data);
            curr = &node.next;
        }
        println!("None");
    }

    // Delete node at a specific position (1-based index)
    fn delete_at_position(&mut self, position: usize) {
        if position == 0 {
            return; // Invalid position, do nothing
        }

        let mut index = 1;
        let mut current = &mut self.head;

        // Special case: deleting the head node
        if position == 1 {
            if let Some(mut node) = self.head.take() {
                self.head = node.next.take(); // Move head to the next node
            }
            return;
        }

        // Traverse to the node just before the one we want to delete
        while let Some(ref mut node) = current {
            if index == position - 1 {
                // If the next node exists, delete it by skipping over it
                if let Some(mut node_to_delete) = node.next.take() {
                    node.next = node_to_delete.next.take(); // Adjust the pointer to skip the node
                }
                return;
            }
            current = &mut node.next;
            index += 1;
        }
    }
}

fn main() {
    let mut list = LinkedList::new();

    // Insert elements
    list.insert_at_head(10);
    list.print_list();

    list.insert_at_head(20);
    list.insert_at_head(30);
    list.print_list();
// Insert elements at the tail
list.insert_at_tail(40);
list.insert_at_tail(50);
list.print_list();


    // Insert at a specific position
    list.insert_at_position(25, 2);
    list.print_list();

    list.insert_at_position(5, 1); // Insert at head
    list.print_list();

    list.insert_at_position(60, 7); // Insert at tail (position greater than current size)
    list.print_list();
}
