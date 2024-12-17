#![allow(dead_code)]
use super::renderer::Renderer;
use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;

pub enum Block {
    Span,
    Block,
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Style {
    H1,
    H2,
    H3,
    H4,
    H5,
    Normal,
    Blockquote,
}

pub enum Children {
    Text(TextNode),
    Node(Node),
}

#[allow(non_snake_case)]
pub struct Node {
    pub _key: String,
    pub _type: Block,
    pub children: Vec<Children>,
    pub markDefs: Vec<MarkDef>,
    pub style: Style,
}

pub struct MarkDef(HashMap<String, String>);

pub struct TextNode {
    pub _key: String,
    pub _type: String,
    pub text: String,
    pub marks: Vec<String>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn portabletest() {
        let text = TextNode {
            _key: "key".to_string(),
            _type: "text".to_string(),
            marks: vec![],
            text: "Hello World".to_string(),
        };
        let text2 = TextNode {
            _key: "key".to_string(),
            _type: "text".to_string(),
            marks: vec![],
            text: "Hello World".to_string(),
        };

        let h2 = Node {
            _key: "key".to_string(),
            style: Style::H2,
            _type: Block::Block,
            children: vec![Children::Text(text)],
            markDefs: vec![],
        };
        let h1 = Node {
            _key: "key".to_string(),
            style: Style::H1,
            _type: Block::Block,
            children: vec![Children::Text(text2), Children::Node(h2)],
            markDefs: vec![],
        };

        let body = vec![h1];
        let mut renderer = Renderer::new(body);
        let result = renderer.render();
        println!("{}", result);
    }
}
