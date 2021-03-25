let n1 = std::u8::MAX;
let n2 = 1u8;
//答えは256だがu8型では表現できない(オーバーフロー)
let n3 = n1 + n2;
println!("{}", n3);