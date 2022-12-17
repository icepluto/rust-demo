fn main(){
    let list_of_number = vec![1,2,3];
    let list_of_string:Vec<String> = list_of_number.iter().map(|x| x.to_string()).collect();
    println!("{:?}",list_of_string);
}