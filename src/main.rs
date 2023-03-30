use no_panic::no_panic;

fn main() {
    println!("Hello, world!");
    i_am_very_safe(2);
}

#[no_panic]
fn i_am_very_safe(i: i32) -> i32 {
    2i32 / i
}