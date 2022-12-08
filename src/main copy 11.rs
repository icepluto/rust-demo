fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    let v1 = vec![1,2,3,4,5];
    let sec = &v1[1];
    println!("second of v1 is {}",sec);
    match v1.get(3) {
        Some(s)=>println!("print second of v1 is {}",s),
        None=>println!("there is no element"),
    }
}
