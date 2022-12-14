fn main(){
    let x1 = CustomSmarterPointer{
        x:String::from("hello1")
    };
    let x2 = CustomSmarterPointer{
        x:String::from("hello2")
    };
    let x3 = CustomSmarterPointer{
        x:String::from("hello3")
    };
    println!("customsmartpointer created!!!");
}
struct CustomSmarterPointer{
    x:String,
}

impl Drop for CustomSmarterPointer {
    fn drop(&mut self) {
        println!("droping customsmarterpointer with x:{}",self.x)
    }
}
