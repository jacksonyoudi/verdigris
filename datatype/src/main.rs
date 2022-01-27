fn main() {
    println!("Hello, world!");

    let x: bool = true;
    let y: bool = true;
    let x = 5;
    if x > 1 {
        println!("x is bigger than 1");
    }
    // as操作符 将bool类型转成数字 0 和 1 
    assert_eq!(x as i32, 1);
    assert_eq!(y as i32, 1);
}
