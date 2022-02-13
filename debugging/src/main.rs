#[allow(dead_code)]
#[allow(unused_assignments)]

fn double_value(v: i32) -> i32 {
    v*2
}

fn main() {
    let mut x: i32 = 3;
    x = double_value(x);
    x = 42;
    println!("Test keybind")
}