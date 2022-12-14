#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}
use std::rc::Rc;

use crate::List::{Cons, Nil};
fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(8, Rc::new(Nil)))));
    println!("{}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("{} {:?}", Rc::strong_count(&a),b);
    {
        let c = Cons(4, Rc::clone(&a));
        println!("{} {:?}", Rc::strong_count(&a),c);
    }
    println!("{}", Rc::strong_count(&a));
}
