struct Point<T> {
    x: T,
    y: T,
}
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &T{
        &self.y
    }
}
fn main() {
    let point = Point { x: 10, y: 12 };
    println!("{}", point.x());
    println!("{}", point.y());
}
