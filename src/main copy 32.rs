use std::ops::Deref;

fn main(){
    let box1 = MyBox::new(5);
    println!("{:?}",box1);
    println!("{}",*box1);
}
#[derive(Debug)]
struct MyBox<T>(T);
impl <T>MyBox<T> {
    fn new(x:T)->MyBox<T>{
        MyBox(x)
    }
}
impl <T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
    
}
