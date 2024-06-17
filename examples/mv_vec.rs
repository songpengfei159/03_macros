use macros::my_vec;

fn main() {
    let v1 = my_vec!(1, 2, 3);
    println!("Hello, world!{:?}", v1);
}