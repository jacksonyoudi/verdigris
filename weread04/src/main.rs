fn main() {
    let x: &str = "hello world";
    println!("{:?}", x);

    let mut str: String = String::from("test");


    str.push_str("xix");

    take_ownership(str);

    // let xx: String = String::from("xx");
    // let lg: (String, usize) = calculate_length(xx);
    // println!("{:?}", lg);


    let x1: &String = &String::from("heelo");
    let i: usize = get_size(x1);
    println!("{:?}", i);

    let word: usize = first_word(x1);
    println!("{:?}", word);
}

// Rust在变量离开作用域时，会调用一个叫作drop的特殊函数
// 在C++中，这种在对象生命周期结束时释放资源的模式有时也被称作资源获取即初始化（Resource Acquisition Is Initialization， RAII)

fn take_ownership(some_string: String) {
    println!("{:?}", some_string);
}


// 生命周期， 和输入保持一致
// fn calculate_length(s: String) -> (String, usize) {
//     let s: usize = s.len();
//     (s, length)
// }


fn get_size(s: &String) -> usize {
    s.len()
}


fn first_word(s: &String) -> usize {
    let bytes: &[u8] = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}


























