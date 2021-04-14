use std::env;
use std::fs::File;
use std::io::BufReader;

//lib クレートに分離したものを使う
use wordcount::count;


fn main(){
    // 1.コマンドラインで指定されたオプションを読み込む
    let filename = env::args().nth(1).expect("1 argument FILENAME required");
    // 2.指定されたファイルを開く
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(&file);

    //3.ファイルから１行ずつ読み込む
    let freqs = count(reader, Default::default());
    println!("{:?}", freqs);
}