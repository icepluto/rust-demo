enum Coin{
    One,
    Two,
    Five,
}
fn main(){
    let r:Option<i32> = Some(12);
    let some_char:Option<char> = Some('e');
    println!("{:?}{:?}",r,some_char);
    let o = Coin::One;
    let t = Coin::Two;
    let f = Coin::Five;
    get_coin(o);
    get_coin(t);
    get_coin(f);
}
fn get_coin(c:Coin)->u8{
    match c {
        Coin::One=>{
            println!("{} yuan",1);
            1
        },
        Coin::Two=>{
            println!("{} yuan",2);
            2
        },
        Coin::Five=>{
            println!("{} yuan",5);
            5
        }
    }
}