use std::{thread, time::Duration};

fn main(){
    let intensity = 12;
    let num = ex_cal(intensity);
    println!("3秒到了 {}",num);
}
fn ex_cal(intensity:u32)->u32{
    println!("3 secs later");
    thread::sleep(Duration::new(3, 0));
    intensity
}