pub mod expression;
pub mod location;
pub mod locutil;
pub mod node;
pub mod options;
pub mod parseutil;
pub mod state;
pub mod statement;
pub mod tokencontext;
pub mod tokenize;
pub mod tokentype;
pub mod whitespace;

pub fn parse(input: String, options: Option<options::Options>) -> node::Node {
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

pub fn tokenizer(input: String, options: Option<options::Options>) -> state::Parser {
    state::Parser::tokenizer(input, options)
}
