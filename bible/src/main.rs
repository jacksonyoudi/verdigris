#![allow(unused)]

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct Post {
    pub title: String,
    // 标题
    pub author: String,
    // 作者
    pub content: String, // 内容
}


impl Summary for Post {
    fn summarize(&self) -> String {
        todo!()
    }
}


fn main() {
    let s1 = "hello";

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    println!("{}", user1.active);
    dbg!(&user2);
// 下面这行会报错
//     println!("{:?}", user1);
}


fn calculate_length(s: &str) -> (&str, usize) {
    let length = s.len(); // len() 返回字符串的长度

    (s, length)
}