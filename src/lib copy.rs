mod front_of_hosting{
    pub mod hosting{
        pub fn add_to_waiting_list(){

        }
        pub fn seat_at_table(){

        }
    }
    pub mod server{
        use super::hosting;

        fn take_order(){

        }
        fn save_order(){

        }
        fn take_payment(){
        }
    }
}
fn eat_at_restaurant(){
    crate::front_of_hosting::hosting::add_to_waiting_list();
    front_of_hosting::hosting::seat_at_table();
}