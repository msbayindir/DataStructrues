
mod linked_list;
use linked_list::LinkedList;






fn main() {

        let mut a = linked_list::LinkedList::new();
        a.AddFirst(1);
        a.AddFirst(2);
        a.AddFirst(3);
        a.AddFirst(4);
        a.AddBefor(5, a.Head.clone().unwrap().Next.unwrap().Next);
        a.AddAfter(6, a.Head.clone().unwrap().Next.unwrap().Next);
        //3-2-1
       let b =  a.RemoveLast();
       let b =  a.RemoveLast();
       a.RemoveLast();
       a.RemoveLast();
      let b = a.RemoveLast();
      let b = a.RemoveLast();
      a.AddFirst(123);
      a.AddFirst(124);
      a.AddFirst(125);
      a.AddFirst(126);


      




       

        
        // a.AddAfter(4,a.Head.clone().unwrap().Next);
        // a.AddAfter(5,a.Head.clone().unwrap().Next);

    
       
        println!("{:?}",a);
        println!("-----------------------------------------------------------");
        println!("-----------------------------------------------------------");
        println!("-----------------------------------------------------------");
        println!("{:?}",b.unwrap().Value);


        



   

}
