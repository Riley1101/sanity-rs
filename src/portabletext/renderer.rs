use std::collections::HashMap;

use super::blocks::{Children, PortableTextNode, Style, HTML};

type Callback = fn(&PortableTextNode) -> String;

/// Structure to convert portable text nodes to HTML.
///
/// This struct holds the input nodes and a configuration map that maps styles to callbacks.
/// The callbacks are functions that handle the styling of the corresponding nodes.
pub struct ToHTML {
    /// Input vector of portable text nodes.
    pub input: Vec<PortableTextNode>,
    /// Configuration map of styles to callbacks.  The key is the style, and the value is a function to apply the style.
    pub config: HashMap<Style, Callback>,
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
    println!("tag: {:?}", tag);
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
        println!("==================================");
        println!("input: {:?}", self.input);
        println!("==================================");

        self.input.iter().fold(String::new(), |mut result, node| {
            let is_list = node
                .extra
                .get("listItem")
                .and_then(|v| v.as_str())
                .unwrap_or("p");

            println!("==================================");
            println!("\n {}", node.html());
            println!("==================================");

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
