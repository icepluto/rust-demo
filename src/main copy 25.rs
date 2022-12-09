fn main(){
    let x = 12;
    let y = 10;
    println!("{}",add_two(x,y));
}
fn add_two(x:i32,y:i32)->i32{
    x + y
}

#[cfg(test)]
mod add_two{
    use super::*;
    #[test]
    fn it_add_two() {
        let x = 10;
        let y = 12;
        assert_eq!(add_two(x, y),22);
    }
}