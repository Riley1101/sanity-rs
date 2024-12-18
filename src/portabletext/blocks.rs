#![allow(dead_code)]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;

#[derive(Debug, Serialize, Deserialize)]
pub enum Block {
    Span,
    Block,
}

#[derive(Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum Style {
    H1,
    H2,
    H3,
    H4,
    H5,
    Normal,
    Blockquote,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Children {
    Text(TextNode),
    Node(Node),
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    pub _key: String,
    pub _type: Block,
    pub children: Vec<Children>,
    pub style: Style,
}

pub trait Render {
    fn html(&self) -> String;
}

impl Render for Node {
    fn html(&self) -> String {
        let mut result = String::from("");
        let tag = match &self.style {
            Style::H1 => "h1",
            Style::H2 => "h2",
            Style::H3 => "h3",
            Style::H4 => "h4",
            Style::H5 => "h5",
            Style::Normal => "p",
            Style::Blockquote => "blockquote",
        };
        for child in &self.children {
            match child {
                Children::Text(text) => {
                    result.push_str(&format!("<{}>{}</{}>", tag, text.text, tag));
                }
                Children::Node(node) => {
                    result.push_str(&node.html());
                }
            }
        }
        result
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.style)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextNode {
    pub _key: String,
    pub _type: String,
    pub text: String,
    pub marks: Vec<String>,
}
