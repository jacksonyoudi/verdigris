fn main() {
    println!("Hello, world!");

    let i = 20;
    let mut o = 5;
    compute(&i, &mut o);

    let x = " hello".to_string();
    join(&x);


    let bx = Box::new("hello");
    let y = x;
}


fn compute(input: &u32, output: &mut u32) {
    if *input > 10 {
        *output = 1;
    }

    if *input > 5 {
        *output *= 2;
    }
}


fn join(s: &String) -> String {
    // move occurs because `*s` has type `String`, which does not implement the `Copy` trait
    // let append = *s;
    "Hello".to_string() + &append
}