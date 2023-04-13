
mod linked_list;
use linked_list::LinkedList;






fn main() {

        let mut a = linked_list::LinkedList::new();
        a.add_first(1);
        a.add_first(2);
        a.add_first(3);
        a.add_first(4);
        a.remove_first();
        let b = a.find(2);



        println!("{:?}",b.Value);





   

}
