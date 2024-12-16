#![allow(dead_code)]

use super::blocks::{Block, Node, Style};
use std::collections::HashMap;
use std::mem;

pub struct Tag(Style);

impl Tag {
    pub fn new(style: Style) -> Self {
        Self(style)
    }
    pub fn to_html(&self, content: String) -> String {
        match self.0 {
            Style::H1 => format!("<h1>{}</h1>", content),
            Style::H2 => format!("<h2>{}</h2>", content),
            Style::H3 => format!("<h3>{}</h3>", content),
            Style::H4 => format!("<h4>{}</h4>", content),
            Style::H5 => format!("<h5>{}</h5>", content),
            Style::H6 => format!("<h6>{}</h6>", content),
            Style::Normal => format!("<p>{}</p>", content),
            Style::Blockquote => format!("<blockquote>{}</blockquote>", content),
        }
    }
}

impl Default for Tag {
    fn default() -> Self {
        Self::new(Style::Normal)
    }
}
pub fn html(node: Node) -> String {
    let mut result = String::new();
    let mut stack = vec![node]; // Stack to manage nodes

    while let Some(current_node) = stack.pop() {
        match current_node.style {
            Style::H1 => {
                result.push_str("<h1>");
                match current_node._type {
                    Block::Text(text) => result.push_str(&text),
                    Block::Block(blocks) => {
                        // Add closing tag for this node to the result later
                        result.push_str("</h1>");
                        // Push children in reverse order to process them correctly
                        for block in blocks.into_iter().rev() {
                            stack.push(*block);
                        }
                        continue; // Skip appending the closing tag immediately
                    }
                }
                result.push_str("</h1>");
            }
            Style::H2 => todo!(),
            Style::H3 => todo!(),
            Style::H4 => todo!(),
            Style::H5 => todo!(),
            Style::H6 => todo!(),
            Style::Normal => todo!(),
            Style::Blockquote => todo!(),
        }
    }

    result
}
pub fn default_html_renderer(node: Node) -> String {
    let mut result = String::new();
    match node.style {
        Style::H1 => {
            result.push_str("<h1>");
            match node._type {
                Block::Text(text) => result.push_str(&text),
                Block::Block(block) => {
                    for block in block {
                        result.push_str(&default_html_renderer(*block))
                    }
                }
            }
            result.push_str("</h1>");
        }
        Style::H2 => todo!(),
        Style::H3 => todo!(),
        Style::H4 => todo!(),
        Style::H5 => todo!(),
        Style::H6 => todo!(),
        Style::Normal => todo!(),
        Style::Blockquote => todo!(),
    }
    result
}

type ConfigMap = HashMap<Style, fn(&Node) -> String>;

pub struct PortableTextRenderer {
    content: Node,
    config: ConfigMap,
}

impl Default for PortableTextRenderer {
    fn default() -> Self {
        Self {
            content: Node {
                style: Style::Normal,
                _type: Block::Text("".to_string()),
                mark_defs: None,
            },
            config: HashMap::new(),
        }
    }
}

impl PortableTextRenderer {
    fn new(input: Node) -> Self {
        Self {
            content: input,
            config: HashMap::new(),
        }
    }

    fn add(&mut self, style: Style, config: fn(&Node) -> String) -> &mut Self {
        self.config.insert(style, config);
        self
    }

    fn render(&mut self) -> String {
        let mut result = String::new();
        let content = mem::replace(
            &mut self.content,
            Node {
                style: Style::Normal,
                _type: Block::Text("".to_string()),
                mark_defs: None,
            },
        );
        let mut stack = vec![content];
        while let Some(current_node) = stack.pop() {
            match current_node.style {
                Style::H1 => {
                    result.push_str("<h1>");
                    match current_node._type {
                        Block::Text(text) => result.push_str(&text),
                        Block::Block(blocks) => {
                            result.push_str("</h1>");
                            for block in blocks.into_iter().rev() {
                                stack.push(*block);
                            }
                            continue; // Skip appending the closing tag immediately
                        }
                    }
                    result.push_str("</h1>");
                }
                Style::H2 => todo!(),
                Style::H3 => todo!(),
                Style::H4 => todo!(),
                Style::H5 => todo!(),
                Style::H6 => todo!(),
                Style::Normal => todo!(),
                Style::Blockquote => todo!(),
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn renderer() {
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

        let mut renderer = PortableTextRenderer::new(heading);
        let result = renderer
            .add(Style::H1, |_node| String::from("<h1>") + "</h1>")
            .render();
        println!("Result: {}", result);
    }
}
