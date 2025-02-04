#[cfg(test)]
mod basic {
    use crate::portabletext::blocks::{Node, Render};
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
        let node = serde_json::from_str::<Node>(input);
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
        let node = serde_json::from_str::<Node>(input);
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

        let node = serde_json::from_str::<Node>(input);
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
        let node = serde_json::from_str::<Node>(input);
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
        let node = serde_json::from_str::<Node>(input);
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
        let node = serde_json::from_str::<Node>(input);
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

        let node = serde_json::from_str::<Node>(input);
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
        let node = serde_json::from_str::<Node>(input);
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
        let node = serde_json::from_str::<Node>(input);
        assert_eq!(
            node.unwrap().html(),
            r###"<blockquote><a href="https://sanity.io/">Sanity</a> can be used to power almost any <a href="https://sanity.io/"><em><strong>app</strong> or website</em></a>.</blockquote>"###
        );
    }
}
