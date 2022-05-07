use std::mem;

fn main() {
    println!("hello world");

    let x: i8 = 1;
    let mut y: i32 = 100;
    let y = 60;

    const MAX_NUM: u32 = 1024;

    let a: char = 'a';


    for i in 1..=5 {
        println!("{}", i)
    }

    let t: (i32, i32) = (100, 100);
    t.0;
    t.1;

    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    arr[2];
    let arr1 = [1; 5];


    let mut stu = Student {
        score: 100,
        name: "xixi",
    };

    stu.name;
    stu.score;


    let c: ColorNoParams = ColorNoParams::Red;

    match c {
        ColorNoParams::Red => println!("{:?}", ColorNoParams::Red),
        ColorNoParams::Yellow => println!("{:?}", ColorNoParams::Yellow),
        ColorNoParams::Blue => println!("{:?}", ColorNoParams::Blue),
    }

    let mut v: Vec<i32> = Vec::new();
    v.push(1);

    let m = v.get(0);

    let mut v1: Vec<i32> = Vec::with_capacity(10);

    let mut v2: Vec<i32> = vec![];

    let s1 = "hello world";
    let str = String::from("hello world");

    let s2 = str.as_str();
}


struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}


struct Student {
    name: &'static str,
    score: i32,
}

struct Color(i32, i32, i32);

#[derive(Debug)]
enum ColorNoParams {
    Red,
    Yellow,
    Blue,
}

