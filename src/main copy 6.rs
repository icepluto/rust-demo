#[derive(Debug)]
struct Rect {
    width: i32,
    height: i32,
}
impl Rect {
    fn area(&self) -> i32 {
        self.width * self.height
    }
    fn perimeter(&self) -> i32 {
        2 * self.width + 2 * self.height
    }
    fn can_hold(&self, other: &Rect) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: i32) -> Rect {
        Rect {
            width: size,
            height: size,
        }
    }
}
fn main() {
    let r = Rect {
        width: 10,
        height: 100,
    };
    let r1 = Rect {
        width: 12,
        height: 99,
    };
    let r2 = Rect {
        width: 15,
        height: 110,
    };
    let squ = Rect::square(10);
    let res = r.area();
    let perimeter = Rect::perimeter(&r);
    let b = r.can_hold(&r1);
    let b1 = r2.can_hold(&r);

    println!("area is :{}", res);
    println!("周长是：{}", perimeter);

    println!("r can hold r1? {}", b);
    println!("r2 can hold r? {}", b1);

    println!("{:#?}",squ);
}
