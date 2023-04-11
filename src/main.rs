
mod linked_list;
use linked_list::LinkedList;






fn main() {

        let mut a = linked_list::LinkedList::new();
        a.AddFirst(1);
        a.AddFirst(2);
        a.AddFirst(3);
        //3-2-1

        
        a.AddAfter(4,a.Head.clone().unwrap().Next);
        a.AddAfter(5,a.Head.clone().unwrap().Next);

    
       
        println!("{:?}",a);



   

}
