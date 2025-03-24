use std::collections::HashMap;

use super::blocks::{Children, PortableTextNode, Render, Style};

type Callback = fn(&PortableTextNode) -> String;

pub struct ToHTML {
    input: Vec<PortableTextNode>,
    config: HashMap<Style, Callback>,
}

fn default_callback(node: &PortableTextNode) -> String {
    let mut result = String::from("");
    let tag = match &node.style {
        Some(style) => match style {
            Style::H1 => "h1",
            Style::H2 => "h2",
            Style::H3 => "h3",
            Style::H4 => "h4",
            Style::H5 => "h5",
            Style::Normal => "p",
            Style::Blockquote => "blockquote",
        },
        None => "p",
    };

    let children = match &node.children {
        Some(children) => children,
        None => return result,
    };

    for child in children {
        match child {
            Children::Span(text) => {
                result.push_str(&format!("<{}>{}</{}>", tag, text.text, tag));
            }
            Children::Block(node) => {
                result.push_str(&node.html());
            }
            Children::Code(node) => {
                result.push_str(&node.html());
            }
            Children::Unknown(_) => println!("unknown field"),
        }
    }
    result
}

impl ToHTML {
    pub fn new(input: Vec<PortableTextNode>) -> Self {
        ToHTML {
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
            let style = match &node.style {
                Some(style) => style,
                None => &Style::Normal,
            };
            let callback = self.config.get(style);
            let callback = match callback {
                Some(callback) => callback,
                None => {
                    result.push_str(&default_callback(node));
                    continue;
                }
            };
            let rendered = callback(node);
            result.push_str(&rendered);
        }
        result
    }
}

#[cfg(test)]
mod test {
    use crate::portabletext::blocks::TextNode;

    use super::*;

    #[test]
    fn default_renderer() {
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

        let blockquote = PortableTextNode {
            _key: "key".to_string(),
            style: Some(Style::Blockquote),
            mark_defs: Some(vec![]),
            _type: "block".to_string(),
            children: Some(vec![Children::Span(text2)]),
            extra: HashMap::new(),
        };

        let paragraph = PortableTextNode {
            _key: "key".to_string(),
            mark_defs: Some(vec![]),
            style: Some(Style::Normal),
            _type: "span".to_string(),
            children: Some(vec![Children::Span(text)]),
            extra: HashMap::new(),
        };

        let body = vec![paragraph, blockquote];
        let result = ToHTML::new(body).render();
        assert_eq!(
            "<p>lorem is cool and i love it</p><blockquote>this is a quote</blockquote>",
            result
        );
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

        let blockquote = PortableTextNode {
            extra: HashMap::new(),
            mark_defs: Some(vec![]),
            _key: "key".to_string(),
            style: Some(Style::Blockquote),
            _type: "block".to_string(),
            children: Some(vec![Children::Span(text2)]),
        };

        let paragraph = PortableTextNode {
            extra: HashMap::new(),
            _key: "key".to_string(),
            mark_defs: Some(vec![]),
            style: Some(Style::Normal),
            _type: "block".to_string(),
            children: Some(vec![Children::Span(text)]),
        };

        let body = vec![paragraph, blockquote];
        let result = ToHTML::new(body)
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
