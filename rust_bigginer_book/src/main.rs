struct Dove;
struct Duck;


impl Tweet for Dove{
    fn tweet(&self){
        println!("Cool!");
    }
}

impl Tweet for Duck{
    fn tweet(&self){
        println!("Quack!");
    }
}

fn main(){
    let dove = Dove{};
    dove.tweet();
    dove.tweet_twice();
    dove.about();
}