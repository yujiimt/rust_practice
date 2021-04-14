use regex::Regex;
use std::collections::HashMap;
use std::io::BufRead;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CountOption{
    Char,
    Word,
    Line
}

impl Default for CountOption{
    fn default() -> Self{
        CountOption::Word
    }
}

pub fn count(input: impl BufRead, option: CountOption) -> HashMap<String, usize>{
    let re = Regex::new(r"\w+").unwrap();
    let mut freqs = HashMap::new();

    for line in input.lines(){
        let line = line.unwrap();
        use crate::CountOption::*;
        match option{
            Char => {
                for c in line.chars(){
                    *freqs.entry(c.to_string()).or_insert(0) += 1;
                }
            }
            Word =>{
                for m in re.find_iter(&line){
                    let word = m.as_str().to_string();
                    *freqs.entry(word).or_insert(0) += 1;
                }
            }
            Line => *freqs.entry(line.to_string()).or_insert(0) += 1,
        }
    }
    freqs
}

#[test]

fn word_count_works2(){
    use std::io::Cursor;

    let mut exp = HashMap::new();
    exp.insert("aa".to_string(), 1);
    exp.insert("cc".to_string(), 1);
    exp.insert("dd".to_string(), 1);

    assert_eq!(count(Cursor::new("aa cc dd"), CountOption::Word), exp);
}

#[test]
#[should_panic]

fn word_count_do_not_contain_unknown_words(){
    use std::io::Cursor;
    
    
    count(
        Cursor::new([
            b'a',
            0xf0, 0x90, 0x80,
            0xe3, 0x81, 0x82,
        ]),
        CountOption::Word,
    );
}

