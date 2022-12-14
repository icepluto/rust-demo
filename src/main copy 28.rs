fn main() {
    let x = 12;
    let res = |z| z == x;
    let y = 12;
    assert_eq!(res(y), true)
}
