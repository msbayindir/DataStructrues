pub struct DoublyLinkedNode<T>{
    Next:Option<Box<DoublyLinkedNode<T>>>,
    Prev:Option<Box<DoublyLinkedNode<T>>>,
    Value:T
}

impl<T> DoublyLinkedNode<T> {
    pub fn new(value:T)->DoublyLinkedNode<T>{
        Self { Next: None, Prev: None, Value: T }
    }
    
}