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


    struct Cacher<T>
        where T: Fn(u32) -> u32 {
        query: T,
        value: Option<u32>,
    }

    impl<T> Cacher<T>
        where
            T: Fn(u32) -> u32 {
        fn new(query: T) -> Cacher<T> {
            Cacher {
                query,
                value: None,
            }
        }

        // 先查询缓存值 `self.value`，若不存在，则调用 `query` 加载
        fn value(&mut self, args: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.query)(args);
                    self.value = Some(v);
                    v
                }
            }
        }
    }
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

