#![allow(dead_code)]
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::fmt::Display;
use std::hash::Hash;

#[derive(Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum Style {
    #[serde(alias = "h1")]
    H1,
    #[serde(alias = "h2")]
    H2,
    #[serde(alias = "h3")]
    H3,
    #[serde(alias = "h4")]
    H4,
    #[serde(alias = "h5")]
    H5,
    #[serde(alias = "normal")]
    Normal,
    #[serde(alias = "blockquote")]
    Blockquote,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Children {
    #[serde(alias = "span")]
    Span(TextNode),
    #[serde(alias = "block")]
    Block(Node),
}

impl Serialize for Children {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("children", 2)?;
        match self {
            Children::Span(text) => {
                state.serialize_field("text", &text.text)?;
            }
            Children::Block(node) => {
                state.serialize_field("node", &node)?;
            }
        }
        state.end()
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    pub _key: String,
    pub _type: String,
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
                Children::Span(text) => {
                    result.push_str(&format!("<{}>{}</{}>", tag, text.text, tag));
                }
                Children::Block(node) => {
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

impl PartialEq for TextNode {
    fn eq(&self, other: &Self) -> bool {
        self._key == other._key && self.text == other.text
    }
}

#[cfg(test)]
mod test {
    use super::Render;
    use super::*;
    #[test]
    fn serialize_text_node() {
        let text = TextNode {
            _key: "key".to_string(),
            _type: "text".to_string(),
            marks: vec![],
            text: "lorem is cool and i love it".to_string(),
        };

        let serialized = serde_json::to_string(&text).unwrap();
        let result =
            r###"{"_key":"key","_type":"text","text":"lorem is cool and i love it","marks":[]}"###;
        assert_eq!(result, serialized);

        let deserialized: TextNode = serde_json::from_str(result).unwrap();
        assert_eq!(text, deserialized);
    }

    #[test]
    fn serialize_portable_span_node() {
        let result = r###"
{
  "children": [
    {
      "_key": "12",
      "text": "lorem is cool and i love it",
      "_type": "span",
      "marks": []
    },
    {
      "_key": "12",
      "text": "lorem is cool and i love it",
      "_type": "span",
      "marks": []
    }
  ],
  "_type": "block",
  "style": "normal",
  "_key": "5dd024df8602",
  "markDefs": []
}
"###;

        let deserialized: Node = serde_json::from_str(result).unwrap();
        deserialized.children.iter().for_each(|child| {
            if let Children::Span(text) = child {
                assert_eq!(text.text, "lorem is cool and i love it");
            }
        });
    }

    #[test]
    fn serialize_portable_block() {
        let result = r###"
{
          "children": [
            {
              "_type": "span",
              "marks": [],
              "text": "Trees are like models of hierarchical data, such as file systems, family trees, and organizational charts. In my laGst articles, I talked about the basics of building trees and binary search trees, which are important data structures in computer science. If you haven’t read them check these out,",
              "_key": "b199ef00ef3a0"
            },

{
          "children": [
            {
              "_type": "span",
              "marks": [],
              "text": "Trees are like models of hierarchical data, such as file systems, family trees, and organizational charts. In my laGst articles, I talked about the basics of building trees and binary search trees, which are important data structures in computer science. If you haven’t read them check these out,",
              "_key": "b199ef00ef3a0"
            }
          ],
          "_type": "block",
          "style": "normal",
          "_key": "5dd024df8602",
          "markDefs": []
}
          ],
          "_type": "block",
          "style": "normal",
          "_key": "5dd024df8602",
          "markDefs": []
}
"###;

        let deserialized: Node = serde_json::from_str(result).unwrap();
        println!("{:?}", deserialized.html());
    }
}
