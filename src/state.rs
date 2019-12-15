use crate::expression;
use crate::locutil;
use crate::node;
use crate::options;
use crate::scope;
use crate::scopeflags;
use crate::statement;
use crate::tokencontext;
use crate::tokenize;
use crate::tokentype;
use crate::whitespace;

use std::collections::HashMap;

use expression::ParserExpression;
use node::ParserNode;
use scope::ParserScope;
use scopeflags::Flags;
use statement::ParserStatement;
use tokenize::ParserTokenize;

// TODO(ryzokuken): Figure this out.
// pub trait Tokenizer: std::iter::Iterator<Item = tokenize::Token> {
//     fn getToken(&self) -> tokenize::Token;
// }

// impl<T> Tokenizer for T where T: tokenize::ParserTokenize {}

pub struct Parser {
    pub options: options::Options,
    pub input: String,

    /// Used to signal to callers of `readWord1` whether the word
    /// contained any escape sequences. This is needed because words with
    /// escape sequences must not be interpreted as keywords.
    pub containsEsc: bool,

    pub pos: usize,
    pub lineStart: usize,
    pub curLine: usize,

    /// The context stack is used to superficially track syntactic
    /// context to predict whether a regular expression is allowed in a
    /// given position.
    pub context: Vec<tokencontext::TokContext>,
    pub exprAllowed: bool,

    pub r#type: tokentype::TokenType,
    pub undefinedExports: HashMap<String, node::Node>,
    pub inModule: bool,
    pub lastTokStart: usize,
    pub lastTokEnd: usize,
    pub lastTokStartLoc: Option<locutil::Position>,
    pub lastTokEndLoc: Option<locutil::Position>,
    pub start: usize,
    pub end: usize,
    pub startLoc: Option<locutil::Position>,
    pub endLoc: Option<locutil::Position>,

    pub value: Option<String>,
    /// Used to signify the start of a potential arrow function
    pub potentialArrowAt: isize,
    /// Scope tracking for duplicate variable names (see scope.rs)
    pub scopeStack: Vec<scope::Scope>,

    pub strict: bool,
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
            r#type: tokentype::TokenType::eof(),
            undefinedExports: HashMap::new(),
            inModule: false,
            lastTokStart: 0,
            lastTokEnd: 0,
            lastTokStartLoc: None,
            lastTokEndLoc: None,
            start: 0,
            end: 0,
            startLoc: None,
            endLoc: None,
            value: None,
            potentialArrowAt: -1,
            exprAllowed: true,
            scopeStack: Vec::new(),
            strict: false,
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
        parser.start = parser.pos;
        parser.end = parser.pos;
        parser.lastTokStart = parser.pos;
        parser.lastTokEnd = parser.pos;
        parser.inModule = options.sourceType != options::SourceType::Module;
        parser.strict = parser.inModule || parser.strictDirective(parser.pos);
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
        parser.parseExpression(None, None)
    }

    pub fn tokenizer(input: String, options: Option<options::Options>) -> Parser {
        Parser::new(options.unwrap_or_default(), input, None)
    }

    pub fn inGenerator(self) -> bool {
        (self.currentVarScope().unwrap().flags & Flags::Generator) > Flags::Zero
    }

    pub fn inAsync(self) -> bool {
        (self.currentVarScope().unwrap().flags & Flags::Async) > Flags::Zero
    }
}
