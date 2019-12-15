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
  pub isAssign: bool,
  pub prefix: bool,
  pub postfix: bool,
  pub binop: Option<usize>,
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

fn binop(name: &str, prec: usize) -> TokenType {
  TokenType::new(
    name,
    TokenTypeConfig {
      beforeExpr: true,
      binop: Some(prec),
      ..Default::default()
    },
  )
}

static beforeExpr: TokenTypeConfig = TokenTypeConfig {
  beforeExpr: true,
  ..Default::default()
};
static startsExpr: TokenTypeConfig = TokenTypeConfig {
  startsExpr: true,
  ..Default::default()
};

/// Map keyword names to token types.
static keywords: std::collections::HashMap<&str, TokenType> = std::collections::HashMap::new();

/// Succinct definitions of keyword token types
fn kw(name: &str, options: TokenTypeConfig) -> TokenType {
  options.keyword = String::from(name);
  let token = TokenType::new(name, options);
  keywords[name] = token;
  token
}

impl TokenType {
  fn new(label: &str, conf: TokenTypeConfig) -> Self {
    TokenType {
      label: String::from(label),
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

  pub fn name() -> Self {
    TokenType::new("name", startsExpr)
  }
  pub fn eof() -> Self {
    TokenType::new("eof", Default::default())
  }

  // Punctuation token types.
  pub fn parenL() -> Self {
    TokenType::new(
      "(",
      TokenTypeConfig {
        beforeExpr: true,
        startsExpr: true,
        ..Default::default()
      },
    )
  }
  pub fn comma() -> Self {
    TokenType::new(",", beforeExpr)
  }
  pub fn colon() -> Self {
    TokenType::new(":", beforeExpr)
  }
  pub fn question() -> Self {
    TokenType::new("?", beforeExpr)
  }
  pub fn semi() -> Self {
    TokenType::new(";", beforeExpr)
  }

  // Operators. These carry several kinds of properties to help the
  // parser use them properly (the presence of these properties is
  // what categorizes them as operators).
  //
  // `binop`, when present, specifies that this operator is a binary
  // operator, and will refer to its precedence.
  //
  // `prefix` and `postfix` mark the operator as a prefix or postfix
  // unary operator.
  //
  // `isAssign` marks all of `=`, `+=`, `-=` etcetera, which act as
  // binary operators with a very low precedence, that should result
  // in AssignmentExpression nodes.
  pub fn eq() -> Self {
    TokenType::new(
      "=",
      TokenTypeConfig {
        beforeExpr: true,
        isAssign: true,
        ..Default::default()
      },
    )
  }
  pub fn incDec() -> Self {
    TokenType::new(
      "++/--",
      TokenTypeConfig {
        prefix: true,
        postfix: true,
        startsExpr: true,
        ..Default::default()
      },
    )
  }
  pub fn logicalOR() -> Self {
    binop("||", 1)
  }
  pub fn logicalAND() -> Self {
    binop("&&", 2)
  }
  pub fn starstar() -> Self {
    TokenType::new("**", beforeExpr)
  }
  pub fn star() -> Self {
      binop("*", 10)
  }

  // Keyword token types.
  pub fn _in() -> Self {
    kw(
      "in",
      TokenTypeConfig {
        beforeExpr: true,
        binop: Some(7),
        ..Default::default()
      },
    )
  }
}
