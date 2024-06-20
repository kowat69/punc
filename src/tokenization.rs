
mod lex;

use lex::*;

use std::cell::RefCell;
use std::default::Default;
use std::fmt;

#[derive(PartialEq, Debug)]
pub enum TKind{
    Figure,
    String,
    Reserved,
    Keyword,
}
pub struct Token<'a>{
    pub kind: TKind,
    pub value: & 'a str,
}
impl<'a> Token<'a>{
    pub fn new(_kind: TKind, _value: & 'a str) -> Self{
        Self{kind: _kind, value: _value}
    }
    pub fn is_equal(&self, kind: Option<TKind>, op: Option<&str>) -> bool{
        if (kind.is_none() || self.kind == kind.expect( "" )) &&
           ( op.is_none() || self.value == op.expect("")){
            return true
        }
        false
    }
}
#[derive(Default)]
pub struct Tokenizer<'a>{
    contents: & 'a str,
}
impl<'a> Tokenizer<'a>{
    pub fn new(m_contents: & 'a str) -> Self{
        Self{contents: m_contents}
    }
    pub fn tokenize(&self) -> Vec<Token<'a>>{
        let mut tokens = Vec::new();
        let mut contents = self.contents;
        while contents.len() > 0{
            if let Ok((s, _)) = ignore_space_or_return(contents){
                contents = s;
                continue;
            }
            if let Ok((s, _)) = ignore_comment(contents){
                contents = s;
                continue;
            }
            let token:Token;
            if let Ok((s, word)) = get_figure(contents){
                contents = s;
                token = Token::new(TKind::Figure, word);
            } else
            if let Ok((s, word)) = get_word(contents){
                contents = s;
                token = Token::new(TKind::Keyword, word);

            }else
            if let Ok((s, word)) = get_string(contents){
                contents = s;
                token = Token::new(TKind::String, word);
            }else
            if let Ok((s, word)) = get_reserved(contents){
                contents = s;
                token = Token::new(TKind::Reserved, word);
            }else{
                panic!();
            }
            tokens.push(token);
        }
        tokens
    }
}
