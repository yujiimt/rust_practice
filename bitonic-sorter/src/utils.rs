use rand::{Rng, SeedableRng};
use rand::distributions::Standard;
use rand_pcg::Pcg64Mcg;


//pub fn new_u32_vec(n: usize) -> Vec<u32>{
    // RNG を初期化する。再現性を持たせるため毎回同じシード値を使う
  //  let mut rng = Pcg64Mcg::from_seed([0; 16]);
    // n個の要素が格納できるようベクタを初期化する　
  //  let mut v = Vec::with_capacity(n);
    // 0から n - 1までの合計n回、繰り返し乱数を生成し、ベクタに追加する
    // (0 から n - 1 の数列は使わないので、　_  で受けとることで、すぐに破棄している)
 //   for _ in 0..n{
 //       v.push(rng.sample(&Standard));
  //  }
    //  ベクタを返す
 //   v
//}

//  なんらかの値を繰り返し生成し、ベクタのような入れものに収集する
// イテレータとコレクタという概念を使う

pub fn new_u32_vec(n: usize) -> Vec<u32>{
    let mut rng = Pcg64Mcg::from_seed([0; 16]);
    // rng.sample_iter()は乱数を無限に生成するイテレータを返す
    // take(n)は元のイテレータから最初のn要素だけ取り出すイテレータを返すcollect()はイテレータから値を収集して、ベクタやハッシュマップのような
    // コレクションに格納する
    rng.sample_iter(&Standard).take(n).collect()

}

pub fn is_sorted_ascending<T: Ord>(x: &[T]) -> bool{
    // windows(2)はスライスから２要素ずつ値を取り出す
    // 新しいイテレータを返す。例えば元が[1, 2, 3, 4]なら
    // [1, 2], [2, 3],[3, 4]を順に返す
    // all(...)はイテレータから値を取り出し、クロージャーに渡す
    // クロージャがfalseを返したら、　そこで処理を打ち切りfalseを返す
    // クロージャがtrueを返している間は、イテレータから次の値を取り出し
    //　クロージャーが一度もfalseを返さなかったら、all(...) は trueを返す
    x.windows(2).all(|pair| pair[0] <= pair[1])
}

pub fn is_sorted_descending<T: Ord>(x: &[T]) -> bool{
    x.windows(2).all(|pair| pair[0] >= pair[1])
}

