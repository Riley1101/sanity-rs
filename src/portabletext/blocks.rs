#![allow(dead_code)]
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::collections::HashMap;
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

#[derive(Debug, Serialize, Deserialize)]
pub struct CodeNode {
    _key: String,
    _type: String,
    code: String,
    language: String,
}

impl Render for CodeNode {
    fn html(&self) -> String {
        format!(
            "<pre><code class=\"language-{}\">{}</code></pre>",
            self.language, self.code
        )
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Children {
    #[serde(alias = "span")]
    Span(TextNode),
    #[serde(alias = "block")]
    Block(Node),

    #[serde(alias = "code")]
    Code(CodeNode),
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
            Children::Code(code) => {
                state.serialize_field("code", &code.code)?;
                state.serialize_field("language", &code.language)?;
            }
        }
        state.end()
    }
}

#[derive(Debug)]
pub struct MarkResult {
    lhs: String,
    rhs: String,
}

#[derive(Debug)]
enum MarkType {
    Code,
    Link,
    Strong,
    Em,
    Unknown,
}

#[derive(Debug)]
pub struct Mark {
    _type: MarkType,
}

impl Mark {
    pub fn new(_type: String) -> Self {
        let _type = match _type.as_str() {
            "code" => MarkType::Code,
            "em" => MarkType::Em,
            "strong" => MarkType::Strong,
            _ => MarkType::Unknown,
        };
        Self { _type }
    }
    pub fn render(&self, mark_def: &MarkDefs) -> MarkResult {
        let extra = &mark_def.extra;
        match self._type {
            MarkType::Code => {
                let language = if let Some(language) = extra.get("language") {
                    language
                } else {
                    ""
                };
                MarkResult {
                    lhs: format!("<pre><code class=\"language-{}\">", language),
                    rhs: format!("</code></pre>"),
                }
            }
            MarkType::Unknown => {
                let href = extra.get("href");

                MarkResult {
                    lhs: format!("<a href=\"{}\">", href.unwrap()),
                    rhs: format!("</a>"),
                }
            }
            MarkType::Em => MarkResult {
                lhs: format!("<em>"),
                rhs: format!("</em>"),
            },
            MarkType::Strong => MarkResult {
                lhs: format!("<strong>"),
                rhs: format!("</strong>"),
            },
            _ => MarkResult {
                lhs: String::new(),
                rhs: String::new(),
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarkDefs {
    pub _key: String,
    pub _type: String,
    #[serde(flatten)]
    pub extra: HashMap<String, String>,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    pub _key: String,
    pub _type: String,
    pub children: Vec<Children>,
    #[serde(alias = "markDefs")]
    pub mark_defs: Vec<MarkDefs>,
    pub style: Style,
}

pub trait Render {
    fn html(&self) -> String;
}

impl Render for Node {
    fn html(&self) -> String {
        let mut result = String::new();
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
            let mark_defs = &self.mark_defs;
            match child {
                Children::Span(text) => {
                    let mut marks_clone = text.marks.clone();
                    let mut wrapped_text = text.text.clone();
                    while let Some(mark) = marks_clone.pop() {
                        let mark_s = Mark::new(mark.clone());
                        match mark_s._type {
                            MarkType::Unknown => {
                                let def_borrowed = mark_defs
                                    .iter()
                                    .find(|mark_def| mark_def._key == mark)
                                    .unwrap();
                                let mark_result = mark_s.render(&def_borrowed);
                                wrapped_text = format!(
                                    "{}{}{}",
                                    mark_result.lhs, wrapped_text, mark_result.rhs
                                );
                            }
                            _ => {
                                wrapped_text = format!("<{}>{}</{}>", mark, wrapped_text, mark);
                            }
                        }
                    }
                    result.push_str(&wrapped_text);
                }
                Children::Block(node) => {
                    result.push_str(&node.html());
                }
                Children::Code(code) => {
                    result.push_str(&format!(
                        "<pre><code class=\"language-{}\">{}</code></pre>",
                        code.language, code.code
                    ));
                }
            }
        }
        result = format!("<{}>{}</{}>", tag, result, tag);
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
    fn serialize_portable_text_array() {
        let result = r###"
        [
            {
                "_key": "ead5dbb19902",
              "children": [
                {
                  "_type": "span",
                  "marks": [],
                  "text": "My 5 year old setup, I can't live without as a software developer.",
                  "_key": "23a6e67f091d0"
                }
              ],
              "markDefs": [],
              "_type": "block",
              "style": "normal"
           },
           {
      "_key": "ead5dbb19902",
      "children": [
        {
          "_type": "span",
          "marks": [],
          "text": "My 5 year old setup, I can't live without as a software developer.",
          "_key": "23a6e67f091d0"
        }
      ],
      "markDefs": [],
      "_type": "block",
      "style": "normal"
    },
    {
      "_key": "3b00a41060fb",
      "children": [
        {
          "_type": "span",
          "marks": [],
          "text": "I am a huge fan of customizing my workflows and setup. I love the ability to code fast, ability to find/consume information without thinking to much and the ability to navigate within my operating system with my muscle memory.",
          "_key": "cafac127eece0"
        }
      ],
      "markDefs": [],
      "_type": "block",
      "style": "normal"
    },
    {
      "_key": "21898aa4a1a1",
      "children": [
        {
          "_type": "span",
          "marks": [],
          "text": "I have always love the joy of tweaking my Ubuntu to tailor my needs from shortcuts, themes, applets to desktop environment. But everything changed once I learnt about tiling window managers.",
          "_key": "b35ab08c59bc0"
        }
      ],
      "markDefs": [],
      "_type": "block",
      "style": "normal"
    },
    {
      "_key": "1db652715268",
      "markDefs": [],
      "_type": "block",
      "style": "normal",
      "children": [
        {
          "_type": "span",
          "marks": [],
          "_key": "09a785ba12c0",
          "text": ""
        }
      ]
    }
       ]
    "###;
        let deserialized: Result<Vec<Node>, serde_json::Error> = serde_json::from_str(result);
        assert!(deserialized.is_ok());
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
            },
            {
              "_type": "block",
              "style": "normal",
              "_key": "feda0bc195f3",
              "markDefs": [],
              "children": [
                {
                  "_type": "span",
                  "marks": [],
                  "text": "Let’s take a look at how you can traverse a tree by visiting each node in a particular order. Traversing a tree can be done using different methods, depending on what you want to achieve. Two common traversal algorithms are depth-first search (DFS) and breadth-first search (BFS). DFS goes as deep as possible in a branch before coming back to check other branches. BFS checks all nodes at the same level before moving to the next level.",
                  "_key": "01bc35fb9ea60"
                }
              ]
            }
        ],
        "_type": "block",
        "style": "normal",
        "_key": "5dd024df8602",
        "markDefs": []
}
"###;

        let deserialized: Result<Node, serde_json::Error> = serde_json::from_str(result);
        assert!(deserialized.is_ok());
    }

    #[test]
    fn mark_defs_test() {
        let mark_defs_content = r###"
        {
            "_key": "ead5dbb19902",
            "children": [
              {
                "_type": "span",
                "marks": [],
                "text": "My 5 year old setup, I can't live without as a software developer.",
                "_key": "23a6e67f091d0"
              }
            ],
            "markDefs": [
              {
                "_key": "ead5dbb19902",
                "_type": "strong",
                "color": "red",
                "font-size": "12px",
                "font-weight": "bold"
              }
            ],
            "_type": "block",
            "style": "normal"
          }
        "###;

        let deserialized: Result<Node, serde_json::Error> = serde_json::from_str(mark_defs_content);
        assert!(deserialized.is_ok());
    }

    #[test]
    fn sample_block() {
        let query = r###"
         {
            "_key": "ead5dbb19902",
            "children": [
              {
                "_type": "span",
                "marks": [],
                "text": "My 4 years",
                "_key": "23a6e67f091d0"
              },
              {
                "_key": "ead5dbb19902",
                "_type": "block",
                "style": "normal",
                "markDefs": [],
                "children": [
                    {
                        "_type": "block",
                        "markDefs": [],
                        "_key": "5dd024df8602",
                        "style": "normal",
                        "children": [
                          {
                            "_type": "span",
                            "marks": [],
                            "text": "Apple of my balls",
                            "_key": "b199ef00ef3a0"
                          }
                        ]
                    }
                ]
              }
            ],
            "markDefs": [
              {
                "_key": "ead5dbb19902",
                "_type": "strong",
                "color": "red",
                "font-size": "12px",
                "font-weight": "bold"
              }
            ],
            "_type": "block",
            "style": "normal"
          }
    "###;
        let deserialized: Result<Node, serde_json::Error> = serde_json::from_str(query);
        assert!(deserialized.is_ok());
    }

    #[test]
    fn render_an_article() {
        let query = r###"
         [
    {
      "_key": "ead5dbb19902",
      "_type": "block",
      "children": [
        {
          "_key": "23a6e67f091d0",
          "_type": "span",
          "marks": [],
          "text": "My 5 year old setup, I can't live without as a software developer."
        },
        {
          "_key": "ead5dbb19902",
          "_type": "code",
          "code": "fn main() { println!(\"Hello, world!\"); }",
          "language": "rust"
        }
      ],
      "markDefs": [],
      "style": "normal"
    },
    {
      "_key": "3b00a41060fb",
      "_type": "block",
      "children": [
        {
          "_key": "cafac127eece0",
          "_type": "span",
          "marks": [],
          "text": "I am a huge fan of customizing my workflows and setup. I love the ability to code fast, ability to find/consume information without thinking to much and the ability to navigate within my operating system with my muscle memory."
        },
        {
          "_key": "ead5dbb19902",
          "_type": "code",
          "code": "fn main() { println!(\"Hello, world!\"); }",
          "language": "rust"
        }
      ],
      "markDefs": [],
      "style": "normal"
    },
    {
      "_key": "21898aa4a1a1",
      "_type": "block",
      "children": [
        {
          "_key": "b35ab08c59bc0",
          "_type": "span",
          "marks": [],
          "text": "I have always love the joy of tweaking my Ubuntu to tailor my needs from shortcuts, themes, applets to desktop environment. But everything changed once I learnt about tiling window managers."
        },
        {
          "_key": "ead5dbb19902",
          "_type": "code",
          "code": "fn main() { println!(\"Hello, world!\"); }",
          "language": "rust"
        }
      ],
      "markDefs": [],
      "style": "normal"
    },
    {
      "_key": "1db652715268",
      "_type": "block",
      "children": [
        {
          "_key": "09a785ba12c0",
          "_type": "span",
          "marks": [],
          "text": ""
        }
      ],
      "markDefs": [],
      "style": "normal"
    }
  ]
        "###;
        let deserialized: Result<Vec<Node>, serde_json::Error> = serde_json::from_str(query);
        assert!(deserialized.is_ok());
    }

    #[test]
    fn serialize_code() {
        let result = r###"
        {
          "_key": "ead5dbb19902",
          "_type": "code",
          "code": "fn main() { println!(\"Hello, world!\"); }",
          "language": "rust"
        }
        "###;
        let deserialized: Result<CodeNode, serde_json::Error> = serde_json::from_str(result);
        assert!(deserialized.is_ok());
    }
}
