#![allow(dead_code)]


// 手动迭代必须将迭代器声明为 mut 可变，
// 因为调用 next 会改变迭代器其中的状态数据（当前遍历的位置等），
// 而 for 循环去迭代则无需标注 mut，因为它会帮我们自动完成
fn main() {
    // fn factory() -> Box<dyn Fn(i32) -> i32> {
    //     let num = 5;
    //
    //     Box::new(|x| x + num)
    // }
    //
    // let f = factory();
    //
    // let answer = f(1);
    // assert_eq!(6, answer);


    let arr = [1, 2, 3];
    let mut arr_iter = arr.into_iter();

    assert_eq!(arr_iter.next(), Some(1));
    assert_eq!(arr_iter.next(), Some(2));
    assert_eq!(arr_iter.next(), Some(3));
    assert_eq!(arr_iter.next(), None);



    // let s = String::new();
    //
    // let update_string =  || println!("{}",s);
    //
    // exec(update_string);
    // exec1(update_string);
    // exec2(update_string);
}

// fn exec<'a, F: FnOne(&'a str) -> String>(mut f: F) {
//     f("hello");
// }


// 所有的闭包都自动实现了 FnOnce 特征，
// 因此任何一个闭包都至少可以被调用一次
// fn exec<F: FnOnce()>(f: F) {
//     f()
// }

// 没有移出所捕获变量的所有权的闭包自动实现了 FnMut 特征
fn exec1<F: FnMut()>(mut f: F) {
    f()
}

// 不需要对捕获变量进行改变的闭包自动实现了 Fn 特征
fn exec2<F: Fn()>(f: F) {
    f()
>>>>>>> 264546c9f59cddee24705ba4e09c2fee89e3d702
}
