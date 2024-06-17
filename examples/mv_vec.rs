use macros::my_vec;

fn main() {
    let v1 = my_vec!(1, 2, 3,);
    vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    // let v1 = my_vec![1;9];
    println!("Hello, world!{:?}", v1);
}