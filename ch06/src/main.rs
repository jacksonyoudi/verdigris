fn modify(mut v: Vec<u32>) -> Vec<u32> {
    v.push(42);
    v
}


fn modify_new(v: &mut [u32]) {
    v.reverse();
}

fn main() {
    
}
