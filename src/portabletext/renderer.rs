use std::collections::HashMap;

use super::blocks::{Node, Style};

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
