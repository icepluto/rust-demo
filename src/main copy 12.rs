fn main() {
    let s = String::from("apple");
    let mut s1 = String::from("red ");
    s1.push_str(&s);
    println!("i like {:?}", &s1);
    let s2 = String::from("hello ");
    let s3 = String::from("world!");
    let s4 = s2 + &s3;
    println!("{}", s4);
    let s5 = String::from("a");
    let s6 = String::from("b");
    let s7 = String::from("c");
    let s8 = format!("{},{},{}", s5, s6, s7);
    println!("{}", s8);
}
