use std::fmt::Debug;
use linked_list_node::LinkedListNode;
pub mod linked_list_node;
#[derive(Debug)]

pub struct LinkedList<T>
where T:Clone
{
    pub Head:Option<LinkedListNode<T>>
}

impl<T> LinkedList<T> 
where T:Clone+ Debug
{
    pub fn new ()->Self
    {
        Self { Head: None }
    }
    pub fn AddFirst(&mut self ,value:T)
    {
        let mut new_node = LinkedListNode::new(value);
        match self.Head {
            None => self.Head = Some(new_node),
            Some(_) => {
              let current_head = self.Head.take().unwrap();
              new_node.Next = Some(Box::new(current_head));
              self.Head = Some(new_node);
            }
          }
    }
    pub fn AddLast(&mut self,value:T)
    {
        let mut new_node = linked_list_node::LinkedListNode::new(value);

        let mut temp = self.Head.as_mut().unwrap();
        while temp.Next !=None {
            let a = temp.Next.as_mut().unwrap();

            temp = temp.Next.as_mut().unwrap();
        }
        temp.Next = Some(Box::new(new_node)); 
    
    }
    pub fn AddAfter(&mut self, value:T,node:Option<Box<LinkedListNode<T>>>){

        let mut new_node = LinkedListNode::new(value);
        let mut temp = self.Head.as_mut().unwrap();
        while temp.Next!=node {
            temp = temp.Next.as_mut().unwrap();
        }
        new_node.Next = node.unwrap().Next;
        temp.Next.as_mut().unwrap().Next = Some(Box::new(new_node));


    }

    pub fn AddBefor(&mut self,value:T,node:Option<Box<LinkedListNode<T>>>){
        
            let mut new_node = LinkedListNode::new(value);
            let mut temp = self.Head.as_mut().unwrap();
            while temp.Next!=node {
                temp = temp.Next.as_mut().unwrap();
            }
            new_node.Next = node;
            temp.Next = Some(Box::new(new_node));
        
    }


    
}