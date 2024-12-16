#![allow(dead_code)]

use super::renderer::html;

pub enum Style {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Normal,
    Blockquote,
}

pub enum Block {
    Text(String),
    Block(Box<Node>),
}

pub struct Node {
    pub style: Style,
    pub _type: Block,
    pub children: Option<Box<Node>>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn portabletest() {
        let heading = Node {
            style: Style::H1,
            _type: Block::Text("Hello, World!".to_string()),
            children: None,
        };

        println!("{}", html(heading));
    }
}
