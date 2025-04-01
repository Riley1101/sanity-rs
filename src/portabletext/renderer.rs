use std::collections::HashMap;

use super::blocks::{Children, PortableTextNode, Style, HTML};

type Callback = fn(&PortableTextNode) -> String;

pub struct ToHTML {
    input: Vec<PortableTextNode>,
    config: HashMap<Style, Callback>,
}

fn default_callback(node: &PortableTextNode) -> String {
    let mut result = String::from("");
    let mut tag = match &node.style {
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
    let is_list = node
        .extra
        .get("listItem")
        .map_or("p", |v| v.as_str().unwrap());

    if is_list == "number" || is_list == "bullet" {
        tag = "li";
    }

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
        let mut list_stack = vec![];

        self.input.iter().fold(String::new(), |mut result, node| {
            let is_list = node
                .extra
                .get("listItem")
                .and_then(|v| v.as_str())
                .unwrap_or("p");

            let style = node.style.as_ref().unwrap_or(&Style::Normal);

            let closing_tag_needed = !list_stack.is_empty() && is_list == "p";

            if is_list == "bullet" || is_list == "number" {
                if closing_tag_needed {
                    while let Some(tag) = list_stack.pop() {
                        result.push_str("</");
                        result.push_str(if tag == "bullet" { "ul" } else { "ol" });
                        result.push_str(">");
                    }
                }
                if list_stack.last().copied() != Some(is_list) {
                    result.push_str(if is_list == "bullet" { "<ul>" } else { "<ol>" });
                    list_stack.push(is_list);
                }
            } else if closing_tag_needed {
                while let Some(tag) = list_stack.pop() {
                    result.push_str("</");
                    result.push_str(if tag == "bullet" { "ul" } else { "ol" });
                    result.push_str(">");
                }
            }

            let callback = self.config.get(style).copied().unwrap_or(default_callback);
            result.push_str(callback(node).as_str());

            result
        })
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

// let is_list = node
//     .extra
//     .get("listItem")
//     .map_or("p", |v| v.as_str().unwrap());
// let style = node.style.as_ref().unwrap_or(&Style::Normal);
//
// if is_list == "bullet" {
//     if list_stack.is_empty() {
//         result.push_str("<ul>");
//     }
//     list_stack.push(is_list);
// } else if is_list == "number" {
//     println!("{:?}", is_list);
//     if list_stack.is_empty() {
//         result.push_str("<ol>");
//     }
//     list_stack.push(is_list);
// } else {
//     if !list_stack.is_empty() {
//         result.push_str("</");
//         let tag = match list_stack.pop() {
//             Some("bullet") => "ul",
//             Some("number") => "ol",
//             _ => "lia",
//         };
//         result.push_str(tag);
//         result.push_str(">");
//     }
// }
//
// let callback = match self.config.get(style) {
//     Some(callback) => *callback,
//     None => default_callback,
// };
// result.push_str(callback(node).as_str());
