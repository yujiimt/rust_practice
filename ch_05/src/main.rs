fn main(){
    let i1 = 42; // i32型
    let f1 = i1 as f64 / 2.5; // i32型からf64型へキャスト

    let c1 = 'a';
    assert_eq!(97, c1 as u32);
    // char型からu32型へキャスト

    let i2 = 300;
    let u1 = i2 as u8;

    assert_eq!(44, u1);
}
