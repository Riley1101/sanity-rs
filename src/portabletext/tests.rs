#[cfg(test)]
mod basic {
    use crate::portabletext::blocks::{PortableTextNode, HTML};
    use serde_json;

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
}
