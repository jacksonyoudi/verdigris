use std::thread;
use std::fmt::Debug;
use std::borrow::Cow;

#[derive(Clone)]
struct A {
    a: i32,
    b: Box<i32>,
}

static NTHREADS: i32 = 10;

// 这是主（`main`）线程
fn main() {
    let a = 10i32;
    // 提供一个 vector 来存放所创建的子线程（children）。
    let mut children = vec![];

    for i in 0..NTHREADS {
        // 启动（spin up）另一个线程
        children.push(thread::spawn(move || {
            println!("this is thread number {}", i)
        }));
    }

    for child in children {
        // 等待线程结束。返回一个结果。
        let _ = child.join();
    }

    fn match_option<T: Debug>(o: Option<T>) {
        match o {
            Some(i) => println!("{:?}", i),
            None => println!("nothing"),
        }
    }

    let duck = Duck;
    fly_static::<Duck>(duck);


    let x: Result<i32, &str> = Ok(-3);
    assert_eq!(x.is_ok(), true);
}

struct Duck;

struct Pig;

// 定义trait
trait Fly {
    fn fly(&self) -> bool;
}


// duck实现Fly
impl Fly for Duck {
    fn fly(&self) -> bool {
        return true;
    }
}

impl Fly for Pig {
    fn fly(&self) -> bool {
        return false;
    }
}


fn fly_static<T: Fly>(s: T) -> bool {
    s.fly()
}


struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x: x, y: y }
    }
}

// 静态分发
// 单态话静态分发






