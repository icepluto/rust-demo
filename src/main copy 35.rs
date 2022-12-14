use std::{thread, time::Duration};

fn main(){
    let handle = thread::spawn(||{
        for i in 1..10{
            println!("{}",i);
            thread::sleep(Duration::from_millis(300));
        }
    });
    handle.join().unwrap();
    for i in 1..5{
        println!(":{}",i);
        thread::sleep(Duration::from_millis(300));
    };
}