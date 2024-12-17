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
    fn portabletest() {
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
            .add(Style::H1, |node| {
                println!("{:?}", node.html());
                "<h1>Hello world</h1>".to_string()
            })
            .render();
        println!("{}", result);
    }
}
