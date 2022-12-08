#[derive(Debug)]
struct Reat{
    width:i32,
    height:i32,
}
fn main(){
    let mut r = Reat{
        width:12,
        height:10,
    };
    r.height=100;
    let res = com(&r);
    println!("{}",res);
    println!("{:#?}",r);
}
fn com(r:&Reat)->i32{
    r.width*r.height
}
