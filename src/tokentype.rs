// ## Token types

// The assignment of fine-grained, information-carrying type objects
// allows the tokenizer to store the information it has about a
// token in a way that is very cheap for the parser to look up.

// All token type variables start with an underscore, to make them
// easy to recognize.

// The `beforeExpr` property is used to disambiguate between regular
// expressions and divisions. It is set on all token types that can
// be followed by an expression (thus, a slash after them would be a
// regular expression).
//
// The `startsExpr` property is used to check if the token ends a
// `yield` expression. It is set on all token types that either can
// directly start an expression (like a quotation mark) or can
// continue an expression (like the body of a string).
//
// `isLoop` marks a keyword as starting a loop, which is important
// to know when parsing a label, in order to allow or disallow
// continue jumps to that label.

#[derive(PartialEq)]
pub struct TokenType {
  label: String,
  keyword: String,
  beforeExpr: bool,
  startsExpr: bool,
  isLoop: bool,
  isAssign: bool,
  prefix: bool,
  postfix: bool,
  binop: Option<usize>,
}

#[derive(Default)]
struct TokenTypeConfig {
  keyword: String,
  beforeExpr: bool,
  startsExpr: bool,
  isLoop: bool,
  isAssign: bool,
  prefix: bool,
  postfix: bool,
  binop: Option<usize>,
}

impl TokenType {
  fn new(label: String, conf: TokenTypeConfig) -> Self {
    TokenType {
      label,
      keyword: conf.keyword,
      beforeExpr: conf.beforeExpr,
      startsExpr: conf.startsExpr,
      isLoop: conf.isLoop,
      isAssign: conf.isAssign,
      prefix: conf.prefix,
      postfix: conf.postfix,
      binop: conf.binop,
    }
  }

  pub fn eof() -> Self {
    TokenType::new(String::from("eof"), Default::default())
  }

  // Punctuation token types.
  pub fn comma() -> Self {
    TokenType::new(
      String::from(","),
      TokenTypeConfig {
        beforeExpr: true,
        ..Default::default()
      },
    )
  }
}
