// 位置情報。.0 から .1までの区間を表す
// 例えばLoc(4 ,6)なら入力文字の５文字目から７文字目までの区間を表す（０始まり）

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Loc(usize, usize);

// loc に便利メソッドを実装しておく
impl Loc{
    fn merge(&self, other: &Loc) -> Loc{
        use std::cmp::{max, min};
        Loc(min(self.0, other.0), max(self.1, other.0))
    }

}

// アノテーション。値に様々なデータを持たせたもの。ここではLocを持たせている
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
type Token = Annot<TokenKind>;

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
        Self::new(TokenKind::Asterisk, loc)
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum LexErrorKind{
    InvalidChar(char),
    Eof,
}

type LexError = Annot<LexErrorKind>;

impl LexError{
    fn invalid_char(c: char, loc: Loc) -> Self{
        LexError::new(LexErrorKind::InvalidChar(c), loc)
    }
    fn eof(loc: Loc) -> Self{
        LexError::new(LexErrorKind::Eof, loc)
    }
}

// 字句解析
fn lex(input: &str) -> Result<Vec<Token>, LexError>{
    // 解析結果を保存するベクタ
    let mut tokens = Vec::new();
    //入力
    let input = input.as_bytes();
    //位置を管理する値
    let mut pos = 0;
    // サブレキサを読んだ後 pos を更新するマクロ
    macro_rules! lex_a_token{
        ($lexer:expr) => {{
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
            b' '|b'\n'| b'\t' => {
                let((), p) = skip_spaces(input, pos)?;
                pos = p;
            }
            //  それ以来が来たらエラー
            b => return Err(LexError::invalid_char(b as char, Loc(pos, pos + 1))),
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
    //入力が期待するものでなければエラー
    if input[pos] != b {
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
    consume_byte(input, start, b'+').map(|(_, end)| (Token::plus(Loc(start, end)), end))
}

fn lex_minus(input: &[u8], start:usize) -> Result<(Token, usize), LexError>{
    consume_byte(input, start, b'-').map(|(_, end)| (Token::minus(Loc(start, end)), end))
}

fn lex_asterisk(input: &[u8], start: usize) -> Result<(Token, usize), LexError>{
    consume_byte(input, start, b'*').map(|(_, end)|(Token::asterisk(Loc(start, end)), end))
}
fn lex_slash(input: &[u8], start: usize) -> Result<(Token, usize), LexError> {
    consume_byte(input, start, b'/').map(|(_, end)| (Token::slash(Loc(start, end)), end))
}
fn lex_lparen(input: &[u8], start: usize) -> Result<(Token, usize), LexError> {
    consume_byte(input, start, b'(').map(|(_, end)| (Token::lparen(Loc(start, end)), end))
}
fn lex_rparen(input: &[u8], start: usize) -> Result<(Token, usize), LexError> {
    consume_byte(input, start, b')').map(|(_, end)| (Token::rparen(Loc(start, end)), end))
}
fn lex_number(input: &[u8], mut pos: usize) -> Result<(Token, usize), LexError> {
    use std::str::from_utf8;

    let start = pos;
    // recognize_many を使って数値を読み込む
        
    let end = recognize_many(input, start, |b|b"1234567890".contains(&b));
    // 入力に数字が続くかぎり位置を進める
    let n = from_utf8(&input[start..end])
    // start..end の構成からfrom_utf8は常に成功する
    .unwrap()
    .parse()
    //同じく構成からfrom_utf8は常に成功するからunwrapしても安全
    .unwrap();
    Ok((Token::number(n, Loc(start, end)), end))
}

fn skip_spaces(input: &[u8], pos: usize) -> Result<((), usize), LexError>{
    // recognize_manyを使って空白を飛ばす
    let pos = recognize_many(input, pos, |b| b"\n\t".contains(&b));
    Ok(((), pos))
}
    fn recognize_many(input: &[u8], mut pos:usize, mut f: impl FnMut(u8) ->bool) -> usize{
    while pos < input.len() && f(input[pos]){
        pos += 1;
    }
    pos
}

#[test]
fn test_lexer(){
    assert_eq!(
        lex("1 + 2 * 3 - -10"),
        Ok(vec![
            Token::number(1, Loc(0, 1)),
            Token::plus(Loc(2,3)),
            Token::number(2, Loc(4,5)),
            Token::asterisk(Loc(6, 7)),
            Token::number(3, Loc(8, 9)),
            Token::minus(Loc(10, 11)),
            Token::minus(Loc(12, 13)),
            Token::number(10, Loc(13, 15)),
        ])
    )
}

use std::io;

// プロンプトを表示し、ユーザーの入力を促す
fn prompt(s: &str) -> io::Result<()>{
    use std::io::{stdout, Write};
    let stdout = stdout();
    let mut stdout = stdout.lock();
    stdout.write(s.as_bytes())?;
    stdout.flush()
}


/// AST を表すデータ型
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AstKind{
    //数値
    Num(u64),
    // 単項演算
    UniOp{op: UniOp, e: Box<Ast>},
    // 二項演算
    BinOp{op: BinOp, l:Box<Ast>, r:Box<Ast>},
}

type Ast = Annot<AstKind>;

//ヘルパメソッドを定義しておく
impl Ast{
    fn num(n: u64, loc:Loc) -> Self{
        //impl<T> Annot<T>で実装したnewを呼ぶ
        Self::new(AstKind::Num(n), loc)
    }
    fn uniop(op:Uniop, e: Ast, loc:Loc) -> Self{
        Self::new(AstKind::UniOp {op, e:Box::new(e)}, loc)
    }
    fn binop(op: BinOp, l: Ast, r: Ast, loc: Loc) -> Self{
        Self::new(
            AstKind::Binop{
            op,
            l: Box::new(l),
            r: Box::new(r),
            },
            loc
        )
    }
}

// 単行演算子を表すデータ型
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum UniOpKind{
    // 正号
    Plus,
    // 負号
    Minus,
}

type UniOp = Annot<UniOpKind>;

impl UniOp{
    fn plus(loc: Loc) -> Self{
        Self::new(UniOpKind::Plus, loc)
    }
    fn minus(loc: Loc) -> Self{
        Self::new(UniOpKind::Minus, loc)
    }
}

//二項演算子を表すデータ型
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum BinOpKind{
    // 加算
    Add,
    // 減産
    Sub,
    // 乗算
    Mult,
    // 徐算
    Div,
}

type BinOp = Annot<BinOpKind>;

impl BinOp{
    fn add(loc: Loc) -> Self{
        Self::new(BinOpKind::Add, loc)
    }

    fn sub(loc: Loc) -> Self{
        Self::new(BinOpKind::sub, loc)
    }

    fn mult(loc: Loc) -> Self{
        Self::new(BinOpKind::mult, loc)
    }

    fn div(loc: Loc) -> Self{
        Self::new(BinOpKind::Div, loc)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum ParseError{
    //予期しないトークンがきた
    UnexpectedToken(Token),
    //式を期待していたのに式でないものがきた
    NotExpression(Token),
    //演算子を期待していたのに演算子でないものがきた
    NotOperator(Token),
    //括弧が閉じられていない
    UnclosedOpenParen(Token),
    //式の解析が終わったのにまだトークンが残っている
    RedundantExpression(Token),
    //パース途中入力で終わった
    Eof
}

fn parse(tokens: Vec<Token>)-> Result<Ast, ParseError>{
    // 入力をイテレーターにし、Peekableにする
    let mut tokens = tokens.into_iter().peekable();
    // その後parse_exprを読んでエラー処理をする
    let ret = parse_expr(&mut tokens)?;
    match tokens.next(){
        Some(tok) => Err(ParseError::RedundantExpression(tok)),
        None => Ok(ret),
    }
}

fn parse_expr<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, ParseError>
where
    Tokens: Iterator<Item = Token>,
    {
        // parse_exprはparse_expr3を呼ぶだけ
        parse_expr3(tokens)
    }

use std::iter::Peekable;

fn parse_expr3<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, ParseError>
where
    Tokens: Iterator<Item = Token>,
    {
        //最初にEXPR3（”＋”｜”ー”）EXPR2を試す
        // まずはEXPR3をパースし
        match parse_expr3(tokens){
            // 失敗したらparse_expr2にフォールバック
            Err(_) => parse_expr2(tokens),
            Ok(e) => {
                // peekで先読みして　
                match tokens.peek().map(|tok|tok.value){
                    Some(TokenKind::Plus) | Some(TokenKind::Minus)=>{
                        let op = match tokens.next().unwrap(){
                            //Tokenは型エイリアンだがパターンマッチにも使える
                            Token{
                                //パターンマッチはネスト可能
                                value: TokenKind::Plus,
                                loc,
                            } => BinOp::add(loc),
                            Token{
                                value: TokenKind::Minus,
                                loc
                            } => BinOp::sun(loc)
                            // 入力が＋かーであることは確認したのでそれ以外はありえない
                            _ => unreachable!()
                        };
                        let r = parse_expr2(tokens)?;
                        //結果は加減
                        let loc = e.loc.merge(&r.loc);
                        Ok(Ast::binop(op, e, r, loc))
                    }
                    // それ以外はエラー。エラーの種類で処理を分ける
                    Some(_) => Err(ParseError::UnexpectedToken(tokens.next().unwrap())),
                        None => Err(ParseError::Eof),
                }
            }
        }
    }




fn main(){
    use std::io::{stdin, BufRead, BufReader};

    let stdin = stdin();
    let stdin = stdin.lock();
    let stdin = BufReader::new(stdin);
    let mut lines = stdin.lines();

    loop {
        prompt(">").unwrap();
        // ユーザーの入力を取得する
        if let Some(Ok(line)) = lines.next(){
            // 字句解析を行う
            let token = lex(&line);
            println!("{:?}", token);
        }else{
            break;
        }
    }
}