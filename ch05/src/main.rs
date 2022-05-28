use std::rc::Rc;

#[derive(Debug, Copy, Clone)]
struct A {
    a: i32,
    b: u32,
}

fn foo(s: String) -> String {
    let w = "world".to_string();
    s + &w
}

fn main() {
    println!("Hello, world!");

    let x = Box::new(5);
    // let y = x;
    println!("{:?}", x);

    let a = A { a: 1, b: 2 };
    // copy
    let b = a;
    println!("{:?}", b);

    let mut s = "hello".to_string();
    s += " world";
    println!("{:?}", s);

    let ss = foo(s);
    // println!("{:?}", s)


    let rc = Rc::new(45);
    let y1 = rc.clone();
    
}
