// 平日を表すWeekday型を定義する
// Debugトレイとを自動導出すると"{:?}で表示できるようになる"
//  PartialEqトレイトを自動導出すると == 演算子が使えるようになる
#[derive(Debug, PartialEq)]
enum Weekday {
    // Weekday型には以下のバリアントがある
    Monday, Tuesday, Wednesday, Thursday, Friday
}

enum Month{
    // バリアントにisize型の整数値を割り当てられる
    January = 1,February = 2,March = 3,
    December = 12,
}

fn say_something(weekday: Weekday){
    if weekday == Weekday::Friday{
        println!("TGIF");
    }else{
        println!("まだ{:?}か", weekday);
        }
}

fn main(){
    say_something(Weekday::Friday);
    assert_eq!(3, Month::March as isize);
}