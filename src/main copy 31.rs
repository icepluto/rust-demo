fn main() {
    let v = vec![1, 2, 3];
    let v1: Vec<_> = v.iter().map(|x| x + 1).collect();
    println!("{:?}", v1);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    style: String,
    size: u32,
}
fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|x| x.size == shoe_size).collect()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                style: String::from("nake"),
                size: 40,
            },
            Shoe {
                style: String::from("lining"),
                size: 40,
            },
            Shoe {
                style: String::from("sanxing"),
                size: 42,
            },
            Shoe {
                style: String::from("boot"),
                size: 40,
            },
            Shoe {
                style: String::from("boot"),
                size: 43,
            },
            Shoe {
                style: String::from("boot"),
                size: 42,
            },
        ];
        let my_size = shoes_in_size(shoes, 42);
        assert_eq!(
            my_size,
            vec![
                Shoe {
                    style: String::from("sanxing"),
                    size: 42,
                },
                Shoe {
                    style: String::from("boot"),
                    size: 42,
                },
            ]
        );
    }
}
