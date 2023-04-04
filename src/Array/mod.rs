pub struct Array<T>{
    pub innerArray:Option<[T;2]>,
    pub Count:i32,
    pub Lenght:i32

}


impl<T> Array<T>{

    pub fn new ()->Array<T>{
            Array{ innerArray:None , Count: 0, Lenght: 2 }
    }

    pub fn add (&mut self,value:T){
       let a =  self.innerArray.unwrap();
    }

}