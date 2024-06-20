use crate::Token;
use crate::TKind;
use std::cell::RefCell;
#[derive(Debug)]
pub enum NKind{
    Integer,
    Add,
    Sub,
    Mul,
    Div,
    NULL,
}
#[derive(Debug)]
pub struct Node<'a>{
    pub lhs: Option<Box<Node<'a>>>,
    pub rhs: Option<Box<Node<'a>>>,
    pub value: Option<& 'a str>,
    pub kind: NKind,
}

impl<'a> Node<'a>{
    pub fn new(_type: NKind, _lhs: Option<Box<Node<'a>>>, _rhs: Option<Box<Node<'a>>>, _value: Option<& 'a str>) -> Self{
        Self{kind: _type, lhs : _lhs, rhs: _rhs, value: _value}
    }
    pub fn create_lrv(ntype: NKind, lhs: Box<Node<'a>>, rhs: Box<Node<'a>>, value: & 'a str) ->Box<Self>{
        Box::new(Node::new(ntype, Some(lhs), Some(rhs), Some(value)))
    }
    pub fn create_lr(ntype: NKind, lhs: Box<Node<'a>>, rhs: Box<Node<'a>>) ->Box<Self>{
        Box::new(Node::new(ntype, Some(lhs), Some(rhs), None))
    }
    pub fn create_null(ntype: NKind) ->Box<Self>{
        Box::new(Node::new(ntype, None, None, None))
    }
    pub fn debug(&self){
        if let Some(lhs) = &self.lhs{
            lhs.debug();
        }
        if let Some(rhs) = &self.rhs{
            rhs.debug();
        }
        println!("{:?} {}", self.kind, if self.value.is_some() { self.value.expect("")} else{""});

    }
}


pub struct Parser<'a>{
    tokens: Vec<Token<'a>>,
    contents: & 'a str,
    count: RefCell<usize>,
}
impl<'a> Parser<'a>{
    pub fn new( _tokens :Vec<Token<'a>>, _contents: & 'a str) -> Self{
        Self{tokens: _tokens, contents: _contents, count: RefCell::new(0)}
    }
    pub fn run(&self) -> Box<Vec<Box<Node<'a>>>>{
        let mut nodes = Box::<Vec<Box<Node<'a>>>>::default();
        while self.tokens.len() > *self.count.borrow(){
            nodes.push(self.expr());
        }
        nodes
    }
    fn consume(&self, kind: Option<TKind>, op: Option<&str>) -> Option<&Token<'a>>{
        let mut count = self.count.borrow_mut();
        if self.tokens.len() <= *count {
            return None
        }
        let mut token =& self.tokens[*count];
        if token.is_equal(kind, op){
            *count += 1;
            Some(token)
        }else{
            None
        }
    }
    fn expect(&self, kind: Option<TKind>, op:Option< &str>) -> & 'a str{
        let mut count = self.count.borrow_mut();
        if self.tokens.len() <= *count {
            panic!("");
        }
        let mut token = &self.tokens[*count];
        if token.is_equal(kind, op){
            *count += 1;
            token.value
        }else {
            panic!("");
        }
    }
    fn expr(&self) -> Box<Node<'a>>{
        let mut node = self.mul();
        loop{
            if let Some(token) = self.consume(Some(TKind::Reserved), Some("+") ) {
                node = Node::create_lrv(NKind::Add, node, self.mul(), token.value);
            }else if let Some(token) = self.consume(Some(TKind::Reserved), Some("-")) {
                node = Node::create_lrv(NKind::Sub, node, self.mul(), token.value);
            }
            else{
                return node
            }
        }
    }
    fn mul(&self) -> Box<Node<'a>>{
        let mut node = self.primary();
        loop{
            if let Some(token) = self.consume(Some(TKind::Reserved), Some("*") ) {
                node = Node::create_lrv(NKind::Mul, node, self.mul(), token.value);
            }else if let Some(token) = self.consume(Some(TKind::Reserved), Some("/")){
                node = Node::create_lrv(NKind::Div, node, self.mul(), token.value);
            }else{
                return node;
            }
            }
    }
    fn primary(&self) -> Box<Node<'a>>{
        let value = self.expect(Some(TKind::Figure), None);
        Box::new(Node::new(NKind::Integer, None, None, Some(value)))
    }
    

}
