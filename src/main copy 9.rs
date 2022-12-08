fn main(){
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}",six);
    println!("{:?}",none);
}
fn plus_one(o:Option<i32>)->Option<i32>{
    match o {
        None=>None,
        Some(i)=>Some(i+1)
    }
}