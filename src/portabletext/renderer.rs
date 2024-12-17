use std::collections::HashMap;

use super::blocks::{Block, Children, Node, Render, Style, TextNode};

type Callback = fn(&Node) -> String;

pub struct Renderer {
    input: Vec<Node>,
    config: HashMap<Style, Callback>,
}

impl Renderer {
    pub fn new(input: Vec<Node>) -> Self {
        Renderer {
            input,
            config: HashMap::new(),
        }
    }

    pub fn add(&mut self, style: Style, callback: Callback) -> &mut Self {
        self.config.insert(style, callback);
        self
    }

    pub fn render(&mut self) -> String {
        let mut result = String::from("");

        for node in &self.input {
            let callback = self.config.get(&node.style).unwrap();
            let rendered = callback(node);
            result.push_str(&rendered);
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn render_headings() {
        let text = TextNode {
            _key: "key".to_string(),
            _type: "text".to_string(),
            marks: vec![],
            text: "well".to_string(),
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
        let result = Renderer::new(body)
            .add(Style::H1, |node| node.html())
            .add(Style::Normal, |node| node.html())
            .render();
        assert_eq!("<h1>Hello World</h1><h2>well</h2>", result);
    }

    #[test]
    fn render_a_span() {
        let text = TextNode {
            _key: "key".to_string(),
            _type: "text".to_string(),
            marks: vec![],
            text: "lorem is cool and i love it".to_string(),
        };

        let text2 = TextNode {
            _key: "key".to_string(),
            _type: "text".to_string(),
            marks: vec![],
            text: "this is a quote".to_string(),
        };

        let blockquote = Node {
            _key: "key".to_string(),
            style: Style::Blockquote,
            _type: Block::Block,
            children: vec![Children::Text(text2)],
            markDefs: vec![],
        };

        let paragraph = Node {
            _key: "key".to_string(),
            style: Style::Normal,
            _type: Block::Block,
            children: vec![Children::Text(text), Children::Node(blockquote)],
            markDefs: vec![],
        };

        let body = vec![paragraph];
        let result = Renderer::new(body)
            .add(Style::H1, |node| node.html())
            .add(Style::Normal, |node| node.html())
            .add(Style::Blockquote, |node| node.html())
            .render();
        assert_eq!(
            "<p>lorem is cool and i love it</p><blockquote>this is a quote</blockquote>",
            result
        );
    }
}
