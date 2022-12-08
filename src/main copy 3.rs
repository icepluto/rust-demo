fn main(){
    let mut s0 = String::from("dont love apple");
    let l = f_s(&s0);
    s0.clear();
    println!("{} {}",s0,l);
}
fn f_s(s:&String)->usize{
    let bytes = s.as_bytes();
    for (i,&item) in bytes.iter().enumerate(){
        if item==b' '{
            return i;
        }
    }
    s.len()
}