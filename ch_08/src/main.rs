fn main(){
    use std::mem::size_of;

    println!("{}", size_of::<&i32>());
    println!("{}", size_of::<&str>());
}