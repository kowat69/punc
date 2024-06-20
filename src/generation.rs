use crate::Node;
use std::collections::VecDeque;
pub struct Generator<'a>{
    contents: & 'a str,
    nodes: &'a Vec<Box<Node<'a>>>,
}
impl <'a> Generator<'a>{
    pub fn new(_nodes: &'a Vec<Box<Node<'a>>>, _contents: & 'a str) -> Self{
        Self{contents: _contents, nodes: _nodes}
    }
    pub fn gen(&self) -> Vec<String>{
        let codes = Vec::<String>::default();
        for node in self.nodes.iter(){
            self._gen(&node.lhs);
            self._gen(&node.rhs);
        }
        codes
    }
    fn _gen(&self, node: &Option<Box<Node<'a>>>){
        if node.is_none() {
            return;
        }
        if let Some(node) = node{
            self._gen(&node.lhs);
            self._gen(&node.rhs);
        }
    }
}
