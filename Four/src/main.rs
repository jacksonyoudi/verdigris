use std::{thread, time};

fn main() {
    // String是
    // pub struct String {
    //     vec: Vec<u8>,
    // }
    // let x = String::from("hello, world");
    // let x = "xxx";

    // let y = x.to_string();
    // println!("{},{}", x, y);

    // let s1 = String::from("hello, world");
    // let s2 = take_ownership(s1);
    //
    // println!("{}", s2);

    // let s = give_ownership();
    // println!("{}", s);


    let s = &String::from("hello, world");

    print_str(s);

    println!("{}", s);


    let x = 1;
    let sum = |y| x + y;

    assert_eq!(3, sum(2));
}


// 可借引用
// fn take_ownership(s: String) -> String {
//     println!("{}", s);
//     s
// }

fn print_str(s: &String) -> &String {
    println!("{}", s);
    s
}

