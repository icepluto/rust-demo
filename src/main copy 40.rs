fn main(){
    
}

#[macro_export]
macro_rules! vec {
    ($($x:expr),*) => {
        {
            let mut vec_list = Vec::new();
            $(
                vec_list.push($x);
            )*
            vec_list
        }
    };
}