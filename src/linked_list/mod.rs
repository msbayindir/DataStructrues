use std::fmt::Debug;
use linked_list_node::LinkedListNode;
pub mod linked_list_node;
#[derive(Debug)]

pub struct LinkedList<T>
where T:Clone + PartialEq
{
    pub Head:Option<LinkedListNode<T>>
}

impl<T> LinkedList<T> 
where T:Clone+ Debug+PartialEq
{
    pub fn new ()->Self
    {

        Self { Head: None }
    }
    //-----------------------------------Add-----------------------------------------
    pub fn add_first(&mut self ,value:T)
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
    pub fn add_last(&mut self,value:T)
    {
        let mut new_node = linked_list_node::LinkedListNode::new(value);

        let mut temp = self.Head.as_mut().unwrap();
        while temp.Next !=None {


            temp = temp.Next.as_mut().unwrap();
        }
        temp.Next = Some(Box::new(new_node)); 
    
    }
    pub fn add_after(&mut self, value:T,node:Option<Box<LinkedListNode<T>>>){

        let mut new_node = LinkedListNode::new(value);
        let mut temp = self.Head.as_mut().unwrap();
        while temp.Next!=node {
            temp = temp.Next.as_mut().unwrap();
        }
        new_node.Next = node.unwrap().Next;
        temp.Next.as_mut().unwrap().Next = Some(Box::new(new_node));


    }

    pub fn add_befor(&mut self,value:T,node:Option<Box<LinkedListNode<T>>>){
        
            let mut new_node = LinkedListNode::new(value);
            let mut temp = self.Head.as_mut().unwrap();
            while temp.Next!=node {
                temp = temp.Next.as_mut().unwrap();
            }
            new_node.Next = node;
            temp.Next = Some(Box::new(new_node));
        
    }
    //-----------------------------------Add-----------------------------------------

    //-----------------------------------Remove--------------------------------------
    pub fn remove_last(&mut self)->Option<Box<LinkedListNode<T>>>{
        if self.Head == None {
            panic!("Head is null, You can'd remove ");
        }
        if  self.Head.as_mut().unwrap().Next ==None {
            let removedNode = Some(Box::new(self.Head.clone().unwrap()));
            self.Head = None;
            return removedNode;
        }
        let mut temp = self.Head.as_mut().unwrap();
        while temp.Next != None {
            if temp.Next.as_mut().unwrap().Next ==None {
                break;
            }
            temp = temp.Next.as_mut().unwrap();
        };
        let removedNode = temp.Next.clone();
        temp.Next = None;
        removedNode

    }
    pub fn remove_first(&mut self)->Option<LinkedListNode<T>>{
       let temp = self.Head.clone();
       self.Head = Some(*self.Head.clone().unwrap().Next.unwrap());
       temp
        
    }
    //-----------------------------------Remove--------------------------------------

    //-----------------------------------Find--------------------------------------
    pub fn find(&mut self,value:T)->LinkedListNode<T>
    {
        let mut temp = self.Head.as_mut().unwrap();
        
        while temp.Value != Some(value.clone()){
            temp = temp.Next.as_mut().unwrap();
         }
         temp.clone()
    }
}