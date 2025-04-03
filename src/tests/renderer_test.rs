#[cfg(test)]
mod renderer_test {
    use crate::portabletext::renderer::ToHTML;
    use crate::portabletext::{Children, PortableTextNode, Style, TextNode, HTML};
    use std::collections::HashMap;

    #[test]
    fn renderer_default_renderer() {
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
    fn renderer_render_a_span() {
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

    #[test]
    fn renderer_block_with_link() {
        let input = r#"
        {
  "_key": "R5FvMrjo",
  "_type": "block",
  "children": [
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

        let node = serde_json::from_str::<PortableTextNode>(input).unwrap();

        let body = vec![node];
        let result = ToHTML::new(body).render();
        println!("result: {:?}", result);
    }
}
