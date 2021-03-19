// この関数は引数を２倍した値を返す
fn double(n: i32) -> i32{
    n + n
}

//この関数は引数の絶対値を返す
fn abs(n: i32) -> i32 {
    if n >= 0 {n} else {-n}
}

//変数に型注釈として関数ポインタ型を指定することで関数名から関数ポインタを得られる
let mut f: fn(i32) -> i32 = double;
assert_eq!(f(-42), -84);


f = abs;
assert_eq!(f(-42), 42);