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
        "code",
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
        println!("{:?}", node.unwrap().html());
        // assert_eq!(
        //     node.unwrap().html(),
        //     "<p><code>sanity</code> is the name of the CLI tool.</p>"
        // );
    }
}
