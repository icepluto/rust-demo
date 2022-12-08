use std::{fs, io::ErrorKind};

fn main(){
    let f = fs::File::open("hello.txt");
    match f {
        Ok(f)=>f,
        Err(e)=>match e.kind() {
            ErrorKind::NotFound=>match fs::File::create("hello.txt"){
                Ok(fc)=>fc,
                Err(err)=>panic!("{}",err)
            },
            other_error=>panic!("other error {}",other_error)
        }
    };
}