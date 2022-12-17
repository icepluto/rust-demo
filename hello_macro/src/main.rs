use hello_macro::HelloMacro;
struct PanCake;

impl HelloMacro for PanCake {
    fn hello_macro(){
        println!("this is a macro,my name is pancake!")
    }
}

fn main(){
    PanCake::hello_macro();
}