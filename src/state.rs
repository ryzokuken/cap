use crate::node;
use crate::options;
use crate::tokencontext;
use crate::tokenize;
use crate::whitespace;

use tokenize::ParserTokenize;

// TODO(ryzokuken): Figure this out.
// pub trait Tokenizer: std::iter::Iterator<Item = tokenize::Token> {
//     fn getToken(&self) -> tokenize::Token;
// }

// impl<T> Tokenizer for T where T: tokenize::ParserTokenize {}

pub struct Parser {
    pub options: options::Options,
    pub input: String,

    // Used to signal to callers of `readWord1` whether the word
    // contained any escape sequences. This is needed because words with
    // escape sequences must not be interpreted as keywords.
    containsEsc: bool,

    pub pos: usize,
    pub lineStart: usize,
    pub curLine: usize,

    pub context: Vec<tokencontext::TokContext>,
}

// TODO(ryzokuken): do you need sourceFile?
impl Parser {
    fn new(options: options::Options, input: String, startPos: Option<usize>) -> Self {
        let parser = Parser {
            options,
            input,
            containsEsc: false,
            pos: 0,
            lineStart: 0,
            curLine: 1,
            context: vec![tokencontext::TokContext::b_stat()],
        };
        if startPos.is_some() {
            let pos = startPos.unwrap();
            parser.pos = pos;
            parser.lineStart = input[pos - 1..].rfind('\n').unwrap();
            let cline: Vec<&str> = whitespace::lineBreak
                .split(&input[..parser.lineStart])
                .collect();
            parser.curLine = cline.len();
        }
        parser
    }

    fn parse_inst(&self) -> node::Node {
        let n = self.options.program.unwrap_or(self.startNode());
        self.nextToken();
        self.parseTopLevel(n)
    }

    // TODO(ryzokuken): where do these options come from?
    pub fn parse(input: String, options: Option<options::Options>) -> node::Node {
        Parser::new(options.unwrap_or_default(), input, None).parse_inst()
    }

    pub fn parseExpressionAt(
        input: String,
        pos: usize,
        options: Option<options::Options>,
    ) -> node::Node {
        let parser = Parser::new(options.unwrap_or_default(), input, Some(pos));
        parser.nextToken();
        parser.parseExpression()
    }

    pub fn tokenizer(input: String, options: Option<options::Options>) -> Parser {
        Box::from(Parser::new(options.unwrap_or_default(), input, None))
    }
}
