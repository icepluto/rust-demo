use std::collections::HashMap;

fn main(){
    let mut scores = HashMap::new();
    scores.insert(String::from("apple"), 10);
    scores.insert(String::from("pear"), 15);
    scores.insert(String::from("peach"), 20);
    for (k,v) in &scores{
        println!("{} : {}",k,v)
    }
    let team1 = String::from("applee");
    let score = scores.get(&team1);
    println!("{:?}",score);
    match scores.get(&team1){
        Some(t)=>println!("{}分",t),
        None=>println!("{} has no number",team1),
    }

}