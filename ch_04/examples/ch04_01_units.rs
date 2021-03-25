// 戻り値の型を省略。値を返さない関数の戻り値はユニット型
fn hello(){
    println!("Hello")
}

// 関数を呼び出し。戻り値に変数retを束縛する
let ret = hello();

assert_eq!(ret, ());

// size_of::<型>は、その型がメモリ上で占める大きさをバイト数で返す
assert_eq!(std::mem::size_of::<()>(),0);