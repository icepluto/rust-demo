fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let res = lonest(string1.as_str(), string2);
    println!("{}",res);
}
fn lonest<'a>(x: &'a str, y: &'a str) -> String{
    let res = String::from("abcdf");
    res
}
