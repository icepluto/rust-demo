fn main(){
    let s = String::from("here is my heart");
    let string0 = &s[0..3];
    let string1 = &s[5..7];
    let len = s.len();
    let string2 = &s[..len];
    println!("{}",&string0);
    println!("{}",&string1);
    println!("{}",&string2);
    let res = first_word(&s);
    println!("{}",res);
    let as_b = s.as_bytes();
    println!("as bytes{:?}",as_b);
}
fn first_word(s:&String)->&str{
    let bytes = s.as_bytes();
    for (i,&item) in bytes.iter().enumerate(){
        if item==b' '{
            return &s[..i];
        }
    }
    &s[..]
}