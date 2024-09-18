#[derive(Debug)]

struct Node<T>{
    data: T,
    next: Option<Box<Node<T>>
}

#[derive(Debug)]

struct LinkedList<T> {
    head:Option<Box<Node<T>>,

}
impl <T> LinkedList{
    fn new() -> LinkedList  {
        LinkedList{
            head: None,
                    }

    }
        
fn push_front(&mut self, data: T) {
    let new_node = Box::new(Node { data,
        next:self.head.take() });
    }
    self.head = Some(new_node);
}

fn push_back(&mut self, data: T) {
    let new_node = Box::new(Node { 
        data,
        next: None 
    });

    if let Some(ref mut last) = self.head {
        while let Some(ref next) = last.next {
            last = next;
        }
        last.next = Some(new_node);
    } else {
        self.head = Some(new_node);
    }
}

fn main(){
 let mut l1 = LinkedList::new();
}