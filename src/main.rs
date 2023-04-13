
mod linked_list;
use linked_list::LinkedList;






fn main() {

        let mut a = linked_list::LinkedList::new();
        a.AddFirst(1);
        a.AddFirst(2);
        a.AddFirst(3);
        a.AddFirst(4);
        a.RemoveFirst();
        let b = a.RemoveFirst();


        println!("{:?}",a);





   

}
