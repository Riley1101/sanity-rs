#![allow(dead_code)]
use std::cmp::Eq;
use std::hash::Hash;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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
    Block(Vec<Box<Node>>),
}

pub struct Node {
    pub style: Style,
    pub _type: Block,
    pub mark_defs: Option<String>,
}

#[cfg(test)]
mod test {
    use crate::portabletext::renderer::default_html_renderer;

    use super::*;

    #[test]
    fn portabletest() {
        let child = Node {
            style: Style::H1,
            _type: Block::Text("Emoji".to_string()),
            mark_defs: None,
        };

        let child2 = Node {
            style: Style::H1,
            _type: Block::Text("🚀".to_string()),
            mark_defs: None,
        };

        let heading = Node {
            style: Style::H1,
            _type: Block::Block(vec![Box::new(child), Box::new(child2)]),
            mark_defs: None,
        };

        assert_eq!(
            "<h1><h1>Emoji</h1><h1>🚀</h1></h1>",
            default_html_renderer(heading)
        );
    }
}
