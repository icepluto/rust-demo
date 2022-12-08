use std::collections::HashMap;

fn main() {
    let str = "a hello world wonderful world a a b c b a";
    let mut map = HashMap::new();
    for word in str.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:#?}", map);
}
