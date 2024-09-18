#[derive(Debug)]
struct Stack<T> {
    elements: Vec<T>,  // The underlying storage for stack elements
}

impl<T> Stack<T> {
    // Create a new empty stack
    fn new() -> Stack<T> {
        Stack { elements: Vec::new() }
    }

    // Push a new element onto the stack
    fn push(&mut self, value: T) {
        self.elements.push(value);
    }

    // Pop an element from the stack (if not empty), returning an Option
    fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    // Peek at the top element of the stack without removing it
    fn peek(&self) -> Option<&T> {
        self.elements.last()
    }

    // Check if the stack is empty
    fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    // Get the size of the stack
    fn size(&self) -> usize {
        self.elements.len()
    }
}

fn main() {
    // Create a stack of integers
    let mut stack = Stack::new();

    // Push elements onto the stack
    stack.push(10);
    stack.push(20);
    stack.push(30);

    // Peek at the top element
    if let Some(top) = stack.peek() {
        println!("Top element: {}", top);  // Output: Top element: 30
    }

    // Pop elements from the stack
    println!("Popped element: {:?}", stack.pop());  // Output: Popped element: Some(30)
    println!("Popped element: {:?}", stack.pop());  // Output: Popped element: Some(20)

    // Check if the stack is empty
    println!("Is the stack empty? {}", stack.is_empty());  // Output: Is the stack empty? false

    // Print the size of the stack
    println!("Stack size: {}", stack.size());  // Output: Stack size: 1

    // Pop the last element
    println!("Popped element: {:?}", stack.pop());  // Output: Popped element: Some(10)
    
    // Now the stack should be empty
    println!("Is the stack empty? {}", stack.is_empty());  // Output: Is the stack empty? true
}
