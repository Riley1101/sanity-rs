#[cfg(test)]
mod tests {
    use crate::portabletext::renderer::ToHTML;
    use crate::portabletext::{Children, CodeNode, PortableTextNode, TextNode, HTML};

    #[test]
    fn empty_blocks() {
        let input = r#"
        {
  "_key": "R5FvMrjo",
  "_type": "block",
  "children": [],
  "markDefs": [],
  "style": "normal"
}
        "#;
        let node = serde_json::from_str::<PortableTextNode>(input);
        assert_eq!(node.unwrap().html(), "<p></p>");
    }

    #[test]
    fn a_single_span() {
        let input = r#"
    {
  "_key": "R5FvMrjo",
  "_type": "block",
  "children": [
    {
      "_key": "cZUQGmh4",
      "_type": "span",
      "marks": [],
      "text": "Plain text."
    }
  ],
  "markDefs": [],
  "style": "normal"
}
    "#;
        let node = serde_json::from_str::<PortableTextNode>(input);
        assert_eq!(node.unwrap().html(), "<p>Plain text.</p>");
    }

    #[test]
    fn multiple_span() {
        let input = r#"
        {
  "_key": "R5FvMrjo",
  "_type": "block",
  "children": [
    {
      "_key": "cZUQGmh4",
      "_type": "span",
      "marks": [],
      "text": "Span number one. "
    },
    {
      "_key": "toaiCqIK",
      "_type": "span",
      "marks": [],
      "text": "And span number two."
    }
  ],
  "markDefs": [],
  "style": "normal"
}
    "#;

        let node = serde_json::from_str::<PortableTextNode>(input);
        assert_eq!(
            node.unwrap().html(),
            "<p>Span number one. And span number two.</p>"
        );
    }

    #[test]
    fn basic_mark_single_span() {
        let input = r#"
        {
  "_key": "R5FvMrjo",
  "_type": "block",
  "children": [
    {
      "_key": "cZUQGmh4",
      "_type": "span",
      "marks": [
        "code"
      ],
      "text": "sanity"
    },
    {
      "_key": "toaiCqIK",
      "_type": "span",
      "marks": [],
      "text": " is the name of the CLI tool."
    }
  ],
  "markDefs": [],
  "style": "normal"
}
    "#;
        let node = serde_json::from_str::<PortableTextNode>(input);
        assert_eq!(
            node.unwrap().html(),
            "<p><code>sanity</code> is the name of the CLI tool.</p>"
        );
    }

    #[test]
    fn basic_mark_multiple_adjacent_span() {
        let input = r#"
{
  "_key": "R5FvMrjo",
  "_type": "block",
  "children": [
    {
      "_key": "cZUQGmh4",
      "_type": "span",
      "marks": [
        "strong"
      ],
      "text": "A word of"
    },
    {
      "_key": "toaiCqIK",
      "_type": "span",
      "marks": [
        "strong"
      ],
      "text": " warning;"
    },
    {
      "_key": "gaZingA",
      "_type": "span",
      "marks": [],
      "text": " Sanity is addictive."
    }
  ],
  "markDefs": [],
  "style": "normal"
}        
"#;
        let node = serde_json::from_str::<PortableTextNode>(input);
        assert_eq!(
            node.unwrap().html(),
            "<p><strong>A word of</strong><strong> warning;</strong> Sanity is addictive.</p>"
        );
    }

    #[test]
    fn basic_mark_multiple_nested_marks() {
        let input = r#"
        {
  "_key": "R5FvMrjo",
  "_type": "block",
  "children": [
    {
      "_key": "cZUQGmh4",
      "_type": "span",
      "marks": [
        "strong"
      ],
      "text": "A word of "
    },
    {
      "_key": "toaiCqIK",
      "_type": "span",
      "marks": [
        "strong",
        "em"
      ],
      "text": "warning;"
    },
    {
      "_key": "gaZingA",
      "_type": "span",
      "marks": [],
      "text": " Sanity is addictive."
    }
  ],
  "markDefs": [],
  "style": "normal"
}
        "#;
        let node = serde_json::from_str::<PortableTextNode>(input);
        assert_eq!(
            node.unwrap().html(),
            "<p><strong>A word of </strong><strong><em>warning;</em></strong> Sanity is addictive.</p>"
        );
    }

    #[test]
    fn link_mark_defs() {
        let input = r#"
        {
  "_key": "R5FvMrjo",
  "_type": "block",
  "children": [
    {
      "_key": "cZUQGmh4",
      "_type": "span",
      "marks": [],
      "text": "A word of warning; "
    },
    {
      "_key": "toaiCqIK",
      "_type": "span",
      "marks": [
        "someLinkId"
      ],
      "text": "Sanity"
    },
    {
      "_key": "gaZingA",
      "_type": "span",
      "marks": [],
      "text": " is addictive."
    }
  ],
  "markDefs": [
    {
      "_type": "link",
      "_key": "someLinkId",
      "href": "https://sanity.io/"
    }
  ],
  "style": "normal"
}
      "#;

        let node = serde_json::from_str::<PortableTextNode>(input);
        assert_eq!(
            node.unwrap().html(),
            "<p>A word of warning; <a href=\"https://sanity.io/\">Sanity</a> is addictive.</p>"
        );
    }

    #[test]
    fn plane_header_block() {
        let input = r###"
        {
  "_key": "R5FvMrjo",
  "_type": "block",
  "children": [
    {
      "_key": "cZUQGmh4",
      "_type": "span",
      "marks": [],
      "text": "Dat heading"
    }
  ],
  "markDefs": [],
  "style": "h2"
}
        "###;
        let node = serde_json::from_str::<PortableTextNode>(input);
        assert_eq!(node.unwrap().html(), "<h2>Dat heading</h2>");
    }

    #[test]
    fn messay_block_with_link() {
        let input = r###"
    {
      "_type": "block",
      "_key": "anything",
      "children": [
        {
          "_key": "a1ph4",
          "_type": "span",
          "marks": [
            "zomgLink"
          ],
          "text": "Sanity"
        },
        {
          "_key": "b374",
          "_type": "span",
          "marks": [],
          "text": " can be used to power almost any "
        },
        {
          "_key": "ch4r1i3",
          "_type": "span",
          "marks": [
            "zomgLink",
            "strong",
            "em"
          ],
          "text": "app"
        },
        {
          "_key": "d3174",
          "_type": "span",
          "marks": [
            "em",
            "zomgLink"
          ],
          "text": " or website"
        },
        {
          "_key": "ech0",
          "_type": "span",
          "marks": [],
          "text": "."
        }
      ],
      "markDefs": [
        {
          "_key": "zomgLink",
          "_type": "link",
          "href": "https://sanity.io/"
        }
      ],
      "style": "blockquote"
    }
    "###;
        let node = serde_json::from_str::<PortableTextNode>(input);
        assert_eq!(
            node.unwrap().html(),
            r###"<blockquote><a href="https://sanity.io/">Sanity</a> can be used to power almost any <a href="https://sanity.io/"><strong><em>app</em></strong></a><em><a href="https://sanity.io/"> or website</a></em>.</blockquote>"###
        );
    }

    #[test]
    fn simple_a_p_tag() {
        let input = r###"
{
  "_type": "block",
  "_key": "e054183e050c",
  "style": "normal",
  "markDefs": [
    {
      "_type": "link",
      "_key": "1544d13f0bd3",
      "href": "https://apple.com"
    }
  ],
  "children": [
    {
      "_type": "span",
      "_key": "9993d9fa4266",
      "text": "Hello",
      "marks": [
        "1544d13f0bd3",
        "code",
        "strong"
      ]
    }
  ]
}
   "###;

        let node = serde_json::from_str::<PortableTextNode>(input);
        assert_eq!(
            node.unwrap().html(),
            r###"<p><a href="https://apple.com"><code><strong>Hello</strong></code></a></p>"###
        );
    }

    #[test]
    fn all_basic_marks() {
        let input = r###"
    {
  "_key": "R5FvMrjo",
  "_type": "block",
  "children": [
    {
      "_key": "a",
      "_type": "span",
      "marks": [
        "code"
      ],
      "text": "code"
    },
    {
      "_key": "b",
      "_type": "span",
      "marks": [
        "strong"
      ],
      "text": "strong"
    },
    {
      "_key": "c",
      "_type": "span",
      "marks": [
        "em"
      ],
      "text": "em"
    },
    {
      "_key": "d",
      "_type": "span",
      "marks": [
        "underline"
      ],
      "text": "underline"
    },
    {
      "_key": "e",
      "_type": "span",
      "marks": [
        "strike-through"
      ],
      "text": "strike-through"
    },
    {
      "_key": "f",
      "_type": "span",
      "marks": [
        "dat-link"
      ],
      "text": "link"
    }
  ],
  "markDefs": [
    {
      "_key": "dat-link",
      "_type": "link",
      "href": "https://www.sanity.io/"
    }
  ],
  "style": "normal"
}
    "###;

        let node = serde_json::from_str::<PortableTextNode>(input);
        assert_eq!(
            node.unwrap().html(),
            r###"<p><code>code</code><strong>strong</strong><em>em</em><u>underline</u><del>strike-through</del><a href="https://www.sanity.io/">link</a></p>"###
        );
    }

    #[test]
    fn marks_all_the_way() {
        let result = r###"
    {
  "_type": "block",
  "_key": "block",
  "style": "normal",
  "children": [
    {
      "_key": "a1ph4",
      "_type": "span",
      "marks": [
        "mark1",
        "em",
        "mark2"
      ],
      "text": "Sanity"
    },
    {
      "_key": "b374",
      "_type": "span",
      "marks": [
        "mark2",
        "mark1",
        "em"
      ],
      "text": " FTW"
    }
  ],
  "markDefs": [
    {
      "_key": "mark1",
      "_type": "highlight",
      "thickness": "1"
    },
    {
      "_key": "mark2",
      "_type": "highlight",
      "thickness": "3"
    }
  ]
}
    "###;
        let node = serde_json::from_str::<PortableTextNode>(result);
        assert_eq!(node.unwrap().html(), "<p><em>Sanity</em><em> FTW</em></p>");
    }

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

        let deserialized: PortableTextNode = serde_json::from_str(result).unwrap();
        let children = deserialized.children.unwrap_or_default();
        children.iter().for_each(|child| {
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
        let deserialized: Result<Vec<PortableTextNode>, serde_json::Error> =
            serde_json::from_str(result);
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

        let deserialized: Result<PortableTextNode, serde_json::Error> =
            serde_json::from_str(result);
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

        let deserialized: Result<PortableTextNode, serde_json::Error> =
            serde_json::from_str(mark_defs_content);
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
        let deserialized: Result<PortableTextNode, serde_json::Error> = serde_json::from_str(query);
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
        let deserialized: Result<Vec<PortableTextNode>, serde_json::Error> =
            serde_json::from_str(query);
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

    #[test]
    fn list_node() {
        let result = r###"
[
  {
    "style": "normal",
    "_type": "block",
    "_key": "f94596b05b41",
    "markDefs": [],
    "children": [
      {
        "_key": "span",
        "_type": "span",
        "text": "Let's test some of these lists!",
        "marks": []
      }
    ]
  },
  {
    "listItem": "number",
    "style": "normal",
    "level": 1,
    "_type": "block",
    "_key": "937effb1cd06",
    "markDefs": [],
    "children": [
      {
        "_type": "span",
        "_key": "span",
        "text": "Number 1",
        "marks": []
      }
    ]
  },
  {
    "listItem": "number",
    "style": "normal",
    "level": 1,
    "_type": "block",
    "_key": "bd2d22278b88",
    "markDefs": [],
    "children": [
      {
        "_type": "span",
        "_key": "span",
        "text": "Number 2",
        "marks": []
      }
    ]
  },
  {
    "listItem": "number",
    "style": "normal",
    "level": 1,
    "_type": "block",
    "_key": "a97d32e9f747",
    "markDefs": [],
    "children": [
      {
        "_type": "span",
        "_key": "span",
        "text": "Number 3",
        "marks": []
      }
    ]
  },
  {
    "style": "normal",
    "_type": "block",
    "_key": "f94596b05b41",
    "markDefs": [],
    "children": [
      {
        "_key": "span",
        "_type": "span",
        "text": "End of the list",
        "marks": []
      }
    ]
  }
]
"###;
        let node: Result<Vec<PortableTextNode>, serde_json::Error> = serde_json::from_str(result);
        let render = ToHTML::new(node.unwrap()).render();
        // assert_eq!("<p>Let's test some of these lists!</p><ol><li>Number 1</li><li>Number 2</li><li>Number 3</li></ol><p>End of the list</p>", render);
        println!("{}", render);
    }

    #[test]
    fn bullet_list() {
        let input = r###"
        [
  {
    "listItem": "bullet",
    "style": "normal",
    "level": 1,
    "_type": "block",
    "_key": "937effb1cd06",
    "markDefs": [],
    "children": [
      {
        "_type": "span",
        "_key": "span",
        "text": "Bullet 1",
        "marks": []
      }
    ]
  },
  {
    "listItem": "bullet",
    "style": "normal",
    "level": 1,
    "_type": "block",
    "_key": "bd2d22278b88",
    "markDefs": [],
    "children": [
      {
        "_type": "span",
        "_key": "span",
        "text": "Bullet 2",
        "marks": []
      }
    ]
  },
  {
    "listItem": "bullet",
    "style": "normal",
    "level": 1,
    "_type": "block",
    "_key": "a97d32e9f747",
    "markDefs": [],
    "children": [
      {
        "_type": "span",
        "_key": "span",
        "text": "Bullet 3",
        "marks": []
      }
    ]
  },
  {
    "style": "normal",
    "_type": "block",
    "_key": "f94596b05b41",
    "markDefs": [],
    "children": [
      {
        "_key": "span",
        "_type": "span",
        "text": "Let's test some of these lists!",
        "marks": []
      }
    ]
  },
  {
    "listItem": "number",
    "style": "normal",
    "level": 1,
    "_type": "block",
    "_key": "937effb1cd06",
    "markDefs": [],
    "children": [
      {
        "_type": "span",
        "_key": "span",
        "text": "Number 1",
        "marks": []
      }
    ]
  },
  {
    "listItem": "number",
    "style": "normal",
    "level": 1,
    "_type": "block",
    "_key": "bd2d22278b88",
    "markDefs": [],
    "children": [
      {
        "_type": "span",
        "_key": "span",
        "text": "Number 2",
        "marks": []
      }
    ]
  },
  {
    "listItem": "number",
    "style": "normal",
    "level": 1,
    "_type": "block",
    "_key": "a97d32e9f747",
    "markDefs": [],
    "children": [
      {
        "_type": "span",
        "_key": "span",
        "text": "Number 3",
        "marks": []
      }
    ]
  },
  {
    "style": "normal",
    "_type": "block",
    "_key": "f94596b05b41",
    "markDefs": [],
    "children": [
      {
        "_key": "span",
        "_type": "span",
        "text": "End of the list",
        "marks": []
      }
    ]
  }
]
        "###;
        let node: Result<Vec<PortableTextNode>, serde_json::Error> = serde_json::from_str(input);
        assert!(node.is_ok());
    }
}
