use std::convert::From;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::fmt;

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}


impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}


struct Circle {
    radius: i32
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

fn main() {
    let num = Number::from(30);
    println!("My number is {:?}", num);

    let my_str = "hello";
    let my_string = String::from(my_str);

    let int = 5;
    // 试试删除类型说明
    let num: Number = int.into();
    println!("My number is {:?}", num);


    // TryFrom

    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // TryInto

    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));

    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());


    "5".parse().unwrap();

    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!{"Sum: {:?}", sum};
}
