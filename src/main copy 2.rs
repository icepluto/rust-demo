
fn main(){
    let mut str0 = String::from("this is a string");
    let l = get_str(&mut str0);
    println!("{}'s lenght is {}",str0,l);
}
fn get_str(s:&mut String)->usize{
    println!("{}",s);
    s.len()
}