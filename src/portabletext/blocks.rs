use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize};
use serde_json::Value;
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

impl HTML for CodeNode {
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
    Block(PortableTextNode),
    #[serde(alias = "code")]
    Code(CodeNode),
    #[serde(untagged)]
    Unknown(String),
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
            Children::Unknown(_) => println!("Unknown field"),
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
    Strong,
    Em,
    Unknown,
    Underline,
    StrikeThrough,
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
            "strike-through" => MarkType::StrikeThrough,
            "underline" => MarkType::Underline,
            _ => MarkType::Unknown,
        };
        Self { _type }
    }

    pub fn render(&self, mark_def: &MarkDefs) -> MarkResult {
        let extra = &mark_def.extra;
        println!("Mark: {:?}", self._type);
        match self._type {
            MarkType::StrikeThrough => MarkResult {
                lhs: String::from("<del>"),
                rhs: String::from("</del>"),
            },
            MarkType::Underline => MarkResult {
                lhs: String::from("<u>"),
                rhs: String::from("</u>"),
            },
            MarkType::Em => MarkResult {
                lhs: String::from("<em>"),
                rhs: String::from("</em>"),
            },
            MarkType::Strong => MarkResult {
                lhs: String::from("<strong>"),
                rhs: String::from("</strong>"),
            },
            MarkType::Code => MarkResult {
                lhs: String::from("<code>"),
                rhs: String::from("</code>"),
            },
            MarkType::Unknown => {
                if let Some(href) = extra.get("href") {
                    MarkResult {
                        lhs: format!("<a href=\"{}\">", href),
                        rhs: format!("</a>"),
                    }
                } else {
                    MarkResult {
                        lhs: String::new(),
                        rhs: String::new(),
                    }
                }
            }
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

#[derive(Debug, Serialize, Deserialize)]
pub struct PortableTextNode {
    pub _key: String,
    pub _type: String,
    pub children: Option<Vec<Children>>,
    pub style: Option<Style>,
    #[serde(alias = "markDefs")]
    pub mark_defs: Option<Vec<MarkDefs>>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

pub trait HTML {
    fn html(&self) -> String;
}

impl HTML for PortableTextNode {
    fn html(&self) -> String {
        let mut result = String::new();

        let tag = match &self.style {
            Some(style) => match style {
                Style::H1 => "h1",
                Style::H2 => "h2",
                Style::H3 => "h3",
                Style::H4 => "h4",
                Style::H5 => "h5",
                Style::Normal => "p",
                Style::Blockquote => "blockquote",
            },
            None => return String::new(),
        };

        let children = match &self.children {
            Some(children) => children,
            None => return String::new(),
        };

        for child in children {
            let mark_defs = &self.mark_defs;
            let mark_defs = match mark_defs {
                Some(mark_defs) => mark_defs,
                None => &Vec::new(),
            };
            match child {
                Children::Span(text) => {
                    let mut marks_clone = text.marks.clone();
                    let mut wrapped_text = text.text.clone();
                    while let Some(mark) = marks_clone.pop() {
                        let mark_s = Mark::new(mark.clone());
                        let def_borrowed = if let Some(def) =
                            mark_defs.iter().find(|mark_def| mark_def._key == mark)
                        {
                            def
                        } else {
                            &MarkDefs {
                                _key: String::new(),
                                _type: String::new(),
                                extra: HashMap::new(),
                            }
                        };

                        let mark_result = mark_s.render(&def_borrowed);
                        wrapped_text =
                            format!("{}{}{}", mark_result.lhs, wrapped_text, mark_result.rhs);
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
                Children::Unknown(_) => {
                    println!("Unknown field");
                }
            }
        }
        result = format!("<{}>{}</{}>", tag, result, tag);
        result
    }
}

impl Display for PortableTextNode {
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
