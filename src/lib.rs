pub mod back_to_house{
    pub struct Breakfast{
        pub toast:String,
        fruit:String,
    }
    impl Breakfast {
        pub fn summer(toast:&str)->Breakfast{
            Breakfast { toast: String::from(toast), fruit: String::from("peach") }
        }
    }
    pub fn p(){
        println!("ppp")
    }
}
pub use crate::back_to_house::p;
pub fn eat_at_restaurant(){
    let mut meal = back_to_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("i'd like {} toast please!",meal.toast);
}