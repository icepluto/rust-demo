use std::{thread, time::Duration};

fn main() {
    let intensity = 13;
    let date = 3;
    generate_workout(intensity, date);
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calcuation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calcuation: T) -> Cacher<T> {
        Cacher { calcuation, value: None }
    }
    fn value(&mut self,arg:u32)->u32{
        match self.value{
            Some(v)=>v,
            None=>{
                let v = (self.calcuation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, date: u32) {
    let mut ex_cal = Cacher::new(|num| {
        thread::sleep(Duration::new(2, 0));
        println!("2秒过去了");
        num
    });
    if intensity < 20 {
        println!("today's do {} pushups", ex_cal.value(intensity));
        println!("and do {} situps", ex_cal.value(intensity));
    } else {
        if date == 3 {
            println!("rest");
        } else {
            println!("today run {} mins ", ex_cal.value(intensity));
        }
    }
}
