fn main() {
    println!("Hello, world!");
    let str = String::from("a string");
    take_owner_string(str);
    let num = 12;
    take_num(num);
    println!("num={}",num);
}
fn take_owner_string(s:String){
    println!("{}",s);
}
fn take_num(n:i32){
    println!("{}",n);
}
