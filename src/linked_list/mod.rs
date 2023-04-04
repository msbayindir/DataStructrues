pub mod linked_list_node;
#[derive(Debug)]

pub struct LinkedList<T>
where T:Clone
{
    pub Head:Option<Box<linked_list_node::LinkedListNode<T>>>
}

impl<T> LinkedList<T> 
where T:Clone
{
    pub fn new ()->Self
    {
        Self { Head: None }
    }
    pub fn AddFirst(&mut self ,value:T)
    {
        let mut newNode = linked_list_node::LinkedListNode::new(value);
        //Some(Box::new(newNode));
        newNode.Next = self.Head.clone();
        self.Head = Some(Box::new(newNode));




    }
}