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

// アノテーション。値に様々なデータを持たせたもの。ここではLocを持たせている
#[derive(Debug, Clone, PatialEq, Eq, Hash)]
struct Annot<T>{
    value: T,
    loc: Loc,
}

impl<T> Annot<T>{
    fn new(value: T, loc:Loc) -> Self{
        Self{value, loc}
    }
}


// トークンの実装
#[derive(Debug, Clone, Copy, PatialEq, Eq, Hash)]
enum TokenKind{
    //[0-9][0-9]*
    Number(u64),
    /// + 
    Plus,
    /// - 
    Minus,
    /// *
    Asterisk,
    /// /
    Slash,
    //// (
    Lparen,
    /// )
    Rparen,
}

// TokenKind にアノテーションを付けたものをTokenとして定義しておく
type Token = Annnot<TokenKind>;

// ヘルパーメソッドを定義しておく

impl Token{
    fn number(n: u64, loc: Loc) -> Self{
        Self::new(TokenKind::Number(n), loc)
    }
    fn plus(loc: Loc) -> Self{
        Self::new(TokenKind::Plus, loc)
    }
    fn minus(loc: Loc) -> Self{
        Self::new(TokenKind::Minus, loc)
    }
    fn asterisk(loc: Loc) -> Self{
        self::new(TokenKind::Asterisk, loc)
    }
    fn slash(loc: Loc) -> Self{
        Self::new(TokenKind::Slash, loc)
    }
    fn lparen(loc: Loc) -> Self{
        Self::new(TokenKind::Lparen, loc)
    }
    fn rparen(loc: Loc) -> Self{
        Self::new(TokenKind::Rparen, loc)
    }
}

// Token Kind と同様の実装をする
#[derive(Debug, Clone, PatialEq, Eq, Hash)]
enum LexErrorKind{
    InvalidChar(char),
    Eof,
}

type LexError = Annot<LexErrorKind>;

impl LexError{
    fn invalid_char(c: char, loc: loc) -> Self{
        Self::new(LexErrorKind::InvalidChar(c), loc)
    }
    fn eof(loc: Loc) -> Self{
        Self::new(LexErrorKind::Eof, loc)
    }
}

// 字句解析
fn lex(input: &str) -> Result<Vec<Token>, LexError>{
    // 解析結果を保存するベクタ
    let mut tokens = Vec::new();
    //入力
    let input = inputs.as_bytes();
    //位置を管理する値
    let mut pos = 0;
    // サブレキサを読んだ後 pos を更新するマクロ
    macro_rules! lex_a_token{
        ($lexer.expr) => {{
            let (tok, p) = $lexer?;
            tokens.push(tok);
            pos = p;
        }};
    }
    while pos < input.len(){
        // ここでそれぞれの関数にinput と pos を渡す
        match input[pos]{
            // 遷移図の実装
            b'0'...b'9' => lex_a_token!(lex_number(input, pos)),
            b'+' => lex_a_token!(lex_plus(input, pos)),
            b'-' => lex_a_token!(lex_minus(input, pos)),
            b'*' => lex_a_token!(lex_asterisk(input, pos)),
            b'/' => lex_a_token!(lex_slash(input, pos)),
            b'(' => lex_a_token!(lex_lparen(input, pos)),
            b')' => lex_a_token!(lex_rparen(input, pos)),
            // 空白を扱う
            b''|b'\n'| b'\t' => {
                let((), p) = skip_spaces(input, pos)?;
                pos = p;
            }
            //  それ以来が来たらエラー
            b => return Err(LexError::invalid_char(b as char, Loc(pos, pos + 1)));
        }
    }
    Ok(tokens)
}

// pos のバイトが期待するうものであれば１バイト消費してposを１進める
fn consume_byte(input:&[u8], pos: usize, b: u8) -> Result<(u8, usize), LexError>{
    // pos が入力がサイズ以上なら入力が終わっている
    // 1バイト期待しているのに終わっているのでエラー
    if input.len() <= pos{
        return Err(LexError::eof(Loc(pos, pos)));
    }
    //入力が規程するものでなければエラー
    if input.len() != b {
        return Err(LexError::invalid_char(
            input[pos] as char,
            Loc(pos, pos + 1),
        ));
    }

    Ok((b, pos + 1))
}

fn lex_plus(input: &[u8], start: usize) -> Result<(Token, usize), LexError>{
    // Result::map を使うことで結果が正常だった場合の処理を簡潔にかける
    // これはこのコードと等価
    // '''
    // match consume_byte(input, start, b'+'){
    //   Ok((_, end))=> (Token::plus(Loc(start, end)), end),
    //     Err(err) => Err(err),
    //}
    consume_byte(input, start, b'+').map(|_, end|
    (Token::plus(Loc(start, end)), end))
}

fn lex_minus(input: &[u8], start:usize) -> Result<(Token, usize), LexError>{
    consume_byte(input, start, b'-').map(|(_, end))
    (Token::minus(Loc(start, end)), end)
}

fn lex_asterisk(input: &[u8], start: usize) -> Result<(Token, usize), LexError>{
    consume_byte(input, start, b'*').map(|(_, end))
    (Token::asterisk(Loc(start, end)), end)
}

fn main(){

}