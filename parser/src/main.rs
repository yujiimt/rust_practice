// 位置情報。.0 から .1までの区間を表す
// 例えばLoc(4 ,6)なら入力文字の５文字目から７文字目までの区間を表す（０始まり）

#[derive(Debiug, Clone, PatialEq, Eq, Hash)]
struct Loc(usize, usize);

// loc に便利メソッドを実装しておく
impl Loc{
    fn merge(&self, other: &Loc) -> Loc{
        use std::cmp::{max, min};
        Loc(min(self.p, other.0), max(self.1, other.0))
    }
}