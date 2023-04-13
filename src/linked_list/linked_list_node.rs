#[derive(Debug)]
pub struct LinkedListNode<T>
where T:Clone + PartialEq
{
    pub Value:Option<T>,
    pub Next:Option<Box<LinkedListNode<T>>>
}


impl<T> LinkedListNode<T>
where T:Clone +PartialEq
{
   pub fn new(value:T)->Self{
    Self { Value: Some(value), Next: None }
   }
}
impl<T> PartialEq for LinkedListNode<T> 
where T:Clone + PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        self.Value == other.Value
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}


impl<T> Clone for LinkedListNode<T>
where T:Clone + PartialEq
{
    fn clone(&self) -> Self {
        Self { Value: self.Value.clone(), Next: self.Next.clone() }
    }
}
