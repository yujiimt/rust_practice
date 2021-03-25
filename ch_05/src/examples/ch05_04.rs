use std::collections::HashMap;

fn main(){
    let mut m1 = HashMap::new(); 
    // または with_capacity(要素数)
    //要素を２つ追加する
    m1.insert("a", 1);
    m1.insert("b", 3);

    assert_eq!(m1.len(), 2);
    // 要素数は２

    //キーに対応する要素を取り出す
    assert_eq!(m1.get("b"), Some(&3));
    assert_eq!(m1.get("c"), None);
    //キーが存在しないのでNone

    // dが存在するならその値への参照を得る。存在しないなら”d”に対して０を
    //登録してから参照を返す
    let d = m1.entry("d").or_insert(0);
    *d += 7;

    assert_eq!(m1.get("d"), Some(&7));

    let m2 = vec![("a", 1), ("b", 3)].into_iter().collect::<HashMap<_, _>>();

}