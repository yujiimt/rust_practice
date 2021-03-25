let t1 = (3, "birds".to_string()); 
// (i32, string)型のタプル/。スタックに置かれる
let mut b1 = Box::new(t1);
// Box ポインタを作る。タプルがヒープに移動する
(*b1).0 += 1; 
// *で参照外し
assert_eq!(*b1, (4, "birds".to_string()));
println!("{:?}", &t1)
}
