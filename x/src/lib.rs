#![allow(unused)]
fn a(a: u32) -> u32 {
    a + 32
}
fn main() {
    let mut b = 32;
    let c  = a(b);
    b = b + 2;
    

    println!("{}", b);
}
