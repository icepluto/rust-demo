use std::{sync::mpsc, thread, time::Duration};

fn main(){
    let (tx,rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move ||{
        let msg = vec![
            String::from("hello "),
            String::from("world! "),
            String::from("welcome "),
            String::from("to "),
            String::from("here"),
        ];
        for i in msg{
            tx.send(i).unwrap();
            thread::sleep(Duration::from_millis(1000));
        }
    });
    thread::spawn(move ||{
        let msg = vec![
            String::from(":hello "),
            String::from(":world! "),
            String::from(":welcome "),
            String::from(":to "),
            String::from(":here"),
        ];
        for i in msg{
            tx1.send(i).unwrap();
            thread::sleep(Duration::from_millis(1000));
        }
    });
    for rec in rx{
        println!("{:?}",rec);
    }

}