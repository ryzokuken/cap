pub mod locutil;
pub mod node;
pub mod options;
pub mod state;
pub mod tokencontext;
pub mod tokenize;

pub fn parse(input: String, options: Option<options::Options>) -> node::Node {
    // node::Node {}
    state::Parser::parse(input, options)
}

// TODO(ryzokuken): why is pos supposed to be optional?
pub fn parseExpressionAt(
    input: String,
    pos: usize,
    options: Option<options::Options>,
) -> node::Node {
    state::Parser::parseExpressionAt(input, pos, options)
}
