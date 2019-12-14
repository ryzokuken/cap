use crate::state;
use crate::tokentype;

pub trait ParserParseUtil {
  /// Predicate that tests whether the next token is of the given
  /// type, and if yes, consumes it as a side effect.
  fn eat(self, r#type: tokentype::TokenType) -> bool;
}

impl ParserParseUtil for state::Parser {
  fn eat(self, r#type: tokentype::TokenType) -> bool {
    if self.r#type == r#type {
      self.next();
      true
    } else {
      false
    }
  }
}

pub struct DestructuringErrors {
  shorthandAssign: isize,
  trailingComma: isize,
  parenthesizedAssign: isize,
  parenthesizedBind: isize,
  doubleProto: isize,
}

impl DestructuringErrors {
  fn new() -> Self {
    DestructuringErrors {
      shorthandAssign: -1,
      trailingComma: -1,
      parenthesizedAssign: -1,
      parenthesizedBind: -1,
      doubleProto: -1,
    }
  }
}
