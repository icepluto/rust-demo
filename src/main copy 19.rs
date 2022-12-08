fn main() {
    let num_list = vec![1, 2, 3, 4, 5, 6, 10, 23, 11, 23, 41, 21];
    let largest = largest(&num_list);
    println!("largest number is {}", largest);
}
fn largest<T: PartialOrd + Copy>(num: &[T]) -> T {
    let mut largest = num[0];
    for &n in num {
        if largest < n {
            largest = n
        }
    }
    largest
}
