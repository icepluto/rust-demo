fn main(){
    println!("12");
    let apple = Fruit::Apple;
    let pear = Fruit::Pear;
    let peach = Fruit::Peach;
    println!("{} is here",choose_one(&apple));
    choose_one(&apple);
    choose_one(&pear);
    choose_one(&peach);
}
#[derive(Debug)]
enum Fruit{
    Apple,
    Pear,
    Peach,
}
fn choose_one(f:&Fruit)->&'static str{
    match f {
        Fruit::Apple => {
            println!("this is a apple");
            "apple"
        },
        Fruit::Pear=>{
            println!("this is {:?}",f);
            ""
        },
        _ => {
            "other fruit"
        },
    }
}