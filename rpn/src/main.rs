fn main(){
    // exp 変数をRPN形式の文字列に束縛
    let exp = "6.1 5.2 4.3 * + 3.4 2.5 / 1.6 * -";

    let ans = rpn(exp);

    // デバッグ時のビルド時のみ。答えが正しいかチェックする
    debug_assert_eq!("26.2840", format!("{:.4}", ans));

    println!("{} = {:.4}", exp, ans);
}

// rpn形式の文字列expを受け取り、f64型の計算結果を返す
fn rpn(exp: &str) -> f64{
    // 変数stack を空のスタックに束縛する
    //stack はミュータブルな変数で、値の変更を許す
    let mut stack = Vec::new();
    
    // exp の要素をスペースで分割し、tokenをそれらに順に束縛する
    //要素がなくなるまで繰り返す
    for token in exp.split_whitespace(){
        // tokenがf64型の数値ならスタックに積む
        if let Ok(num) = token.parse::<f64>(){
            stack.push(num);
        } else {
            // token が数値でないなら、演算子なのか調べる
            match token{
                // tokenが演算子ならapply2関数で計算する
                "+" => apply2(&mut stack,|x, y| x + y),
                "-" => apply2(&mut stack,|x, y| x - y),
                "*" => apply2(&mut stack,|x, y| x * y),
                "/" => apply2(&mut stack,|x, y| x / y),
                //tokenが演算子でないなら、エラーを起こして終了
                _ => panic!("Unknown operator: {}", token),
            }
        }
    }
    // スタックから数値を１つ取り出す。失敗したらエラーを起こして終了する
    stack.pop().expect("Stack underflow")
}


fn apply2<F>(stack: &mut Vec<f64>, fun: F)
// F型のトレイト境界
where
    F: Fn(f64, f64) -> f64,
{
    // 変数yとxをスタックの最後の2要素に束縛する
    if let (Some(y), Some(x)) = (stack.pop(), stack.pop()){
        // クロージャ fun で計算し、その結果に変数zを束縛する
        let z = fun(x, y);

        stack.push(z);
    } else {
        //スタックから要素が取り出せなかったときはエラーを起こして終了する
        panic!("Stack underflow");
    }
}