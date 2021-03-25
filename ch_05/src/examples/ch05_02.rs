fn main(){
    let v1 = vec![false, true, false]; 
    // Vec<bool>型
    let v2 = vec![0.0, -1.0, 1.0, 0.5];
    // Vec<f64>型
    // 長さ１００のベクタを作り、0i32で初期化する
    // 要素の型はclone トレイトを実装していなければならない
    let v3 = vec![0; 100];
    assert_eq!(v3.len(), 100);
    

    //ベクタは入れ子にできる。子の要素数はそれぞれが異なってもかまわない
    let v4 = vec![vec!["a", "b", "c"], vec!["d"]];
    //Vec<Vec<char>>型

    //ベクタは同じ型の要素の要素の並び。異なる型の要素は持てない
   // let v5 = vec![false, "a"];
    // -> error[E0308]: mismatched types
    let mut  v6 = vec!["a", "b", "c"];
    
    // Var<char>型
    v6.push("d"); //最後尾に値を追加
    v6.push("e");

    assert_eq!(v6.pop(), Some("e"));

    v6.insert(1, "f");

    // インデックス１の箇所にfを追加

    assert_eq!(v6.remove(2), "b");
    //  インデックス２の要素を削除。戻り値は削除した値
    assert_eq!(v6, ["a", "f", "c", "d"]);

    let mut v7 = vec!["g", "h"];
    // 別のベクタv7を作成
    v6.append(&mut v7);
    // v6の最後尾にv７の前要素を追加
    assert_eq!(v6, ["a", "f", "c", "d", "g", "h"]);
 //   assert_eq!(v7, []);
    // v7の要素はv６に移動したため空になった

    let a8 = ["i", "j"];
    //配列a8を作成
    v6.extend_from_slice(&a8);

    //v6の最後尾にa8の前要素を追加

    
    assert_eq!(v6, ["a", "f", "c", "d", "g", "h", "i", "j"]);
    assert_eq!(a8, ["i", "j"]);

}