#[derive(Debug)]
pub struct LinkedListNode<T>
where T:Clone
{
    pub Value:Option<T>,
    pub Next:Option<Box<LinkedListNode<T>>>
}


impl<T> LinkedListNode<T>
where T:Clone
{
   pub fn new(value:T)->Self{
    Self { Value: Some(value), Next: None }
   }
}


impl<T> Clone for LinkedListNode<T>
where T:Clone
{
    fn clone(&self) -> Self {
        Self { Value: self.Value.clone(), Next: self.Next.clone() }
    }
}
