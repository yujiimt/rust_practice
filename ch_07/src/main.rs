fn main(){
    let mut p1 = Parent(1, Child(11), Child(12));
    let p2 = p1;

    println!("p2: {:?}", p2);
    println!("p1: {:?}", p1);
}