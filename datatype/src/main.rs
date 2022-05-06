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
}


struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

