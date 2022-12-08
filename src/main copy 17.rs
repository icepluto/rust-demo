use std::{io::{self, Read}, fs::File};

fn main(){
    let username = read_usrename_from_file();
    println!("{:?}",username);
}
fn read_usrename_from_file()->Result<String,io::Error>{
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(ff)=>ff,
        Err(e)=>return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_)=>Ok(s),
        Err(e)=>Err(e),
    }
}