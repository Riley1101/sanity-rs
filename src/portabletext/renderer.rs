use super::blocks::{Block, Node, Style};

pub fn html(node: Node) -> String {
    let mut result = String::new();
    match node.style {
        Style::H1 => {
            result.push_str("<h1>");
            match node._type {
                Block::Text(text) => result.push_str(&text),
                Block::Block(block) => result.push_str(&html(*block)),
            }
            result.push_str("</h1>");
        }
        Style::H2 => todo!(),
        Style::H3 => todo!(),
        Style::H4 => todo!(),
        Style::H5 => todo!(),
        Style::H6 => todo!(),
        Style::Normal => todo!(),
        Style::Blockquote => todo!(),
    }
    result
}
