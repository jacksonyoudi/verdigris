<<<<<<< HEAD
/*
into_iter 会夺走所有权
iter 是借用
iter_mut 是可变借用
 */

fn main() {
    let values = vec![1, 2, 3];

    for v in values.into_iter().into_iter().into_iter() {
        println!("{}",v)
    }
}
