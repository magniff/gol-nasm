mod ffi {
    extern "C" {
        pub fn factorial(value: u64) -> u64;
        pub fn add_two(a: u64, b: u64) -> u64;
    }
}

pub fn factorial(value: u64) -> u64 {
    unsafe { ffi::factorial(value) }
}

pub fn add_two(a: u64, b: u64) -> u64 {
    unsafe { ffi::add_two(a, b) }
}

fn main() {
    for value in 0..=10 {
        println!("{value}! = {result}", result = factorial(value));
    }
    println!("2 + 5 = {result}", result = add_two(2, 5));
}
