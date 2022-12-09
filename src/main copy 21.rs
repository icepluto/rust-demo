fn main() {
    let point1 = Point{
        x:10,
        y:12,
    };
    let point2 = Point{
        x:"apple",
        y:"pear",
    };
    let res = point1.mixup(point2);
    println!("{:?}",res);
}
#[derive(Debug)]
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}
impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
