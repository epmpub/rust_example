use std::convert::From;
#[derive(Debug)]
struct Number {
    value: i32,
}

#[derive(Debug)]
enum Colors {
    Red,
    Blue,
    Yellow,
    Bad,
}

impl From<i32> for Colors {
    fn from(item: i32) -> Self {
        match item {
            1=>Colors::Blue,
            2=>Colors::Red,
            3=>Colors::Yellow,
            _=>Colors::Bad,
        }
    }
}

type NE =  Number;

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let num = Number::from(30);
    println!("My number is {:?}", num);

    let cc = Colors::from(1);
    println!("{:?}",cc);

    let cc = Colors::from(2);
    println!("{:?}",cc);

    let cc = Colors::from(3);
    println!("{:?}",cc);

    let cc = Colors::from(5);
    println!("{:?}",cc);

}


