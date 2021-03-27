fn main(){
    let p1 = Box::new(10); //Box<i32>型
    let p3: *mut i32 = unsafe {std::mem::transmute(p1)};


}