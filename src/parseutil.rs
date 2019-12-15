use crate::location;
use crate::state;
use crate::tokentype;

use location::ParserLocation;

pub trait ParserParseUtil {
  /// Tests whether parsed token is a contextual keyword.
  fn isContextual(self, name: &str) -> bool;

  /// Predicate that tests whether the next token is of the given
  /// type, and if yes, consumes it as a side effect.
  fn eat(self, r#type: tokentype::TokenType) -> bool;

  /// Expect a token of a given type. If found, consume it, otherwise,
  /// raise an unexpected token error.
  fn expect(self, r#type: tokentype::TokenType) -> Result<(), location::SyntaxError>;

  /// Raise an unexpected token error.
  fn unexpected(self, pos: Option<usize>) -> Result<(), location::SyntaxError>;

  fn checkPatternErrors(self, refDetructuringErrors: Option<DestructuringErrors>, isAssign: bool);

  fn checkExpressionErrors(
    self,
    refDetructuringErrors: Option<DestructuringErrors>,
    andThrow: bool,
  ) -> bool;
}

impl ParserParseUtil for state::Parser {
  fn isContextual(self, name: &str) -> bool {
    self.r#type == tokentype::TokenType::name()
      && self.value.unwrap().as_str() == name
      && !self.containsEsc
  }

  fn eat(self, tt: tokentype::TokenType) -> bool {
    if self.r#type == tt {
      self.next();
      true
    } else {
      false
    }
  }

  fn expect(self, tt: tokentype::TokenType) -> Result<(), location::SyntaxError> {
    if self.eat(tt) {
      Ok(())
    } else {
      self.unexpected(None)
    }
  }

  fn unexpected(self, pos: Option<usize>) -> Result<(), location::SyntaxError> {
    self.raise(pos.unwrap_or(self.start), String::from("Unexpected token"))
  }

  fn checkExpressionErrors(
    self,
    refDetructuringErrors: Option<DestructuringErrors>,
    andThrow: bool,
  ) -> bool {
    if refDetructuringErrors.is_none() {
      return false;
    }
    let shorthandAssign = refDetructuringErrors.unwrap().shorthandAssign;
    let doubleProto = refDetructuringErrors.unwrap().doubleProto;
    if !andThrow {
      return shorthandAssign >= 0 || doubleProto >= 0;
    }
    if shorthandAssign >= 0 {
      self.raise(
        shorthandAssign as usize,
        String::from("Shorthand property assignments are valid only in destructuring patterns"),
      );
    }
    if doubleProto >= 0 {
      self.raiseRecoverable(
        doubleProto as usize,
        String::from("Redefinition of __proto__ property"),
      );
    }
    false
  }

  fn checkPatternErrors(self, refDetructuringErrors: Option<DestructuringErrors>, isAssign: bool) {
    if refDetructuringErrors.is_some() {
      let refDetructuringErrors = refDetructuringErrors.unwrap();
      if refDetructuringErrors.trailingComma > -1 {
        self.raiseRecoverable(
          refDetructuringErrors.trailingComma as usize,
          String::from("Comma is not permitted after the rest element"),
        );
      }
      let parens = if isAssign {
        refDetructuringErrors.parenthesizedAssign
      } else {
        refDetructuringErrors.parenthesizedBind
      };
      if parens > -1 {
        self.raiseRecoverable(parens as usize, String::from("Parenthesized pattern"));
      }
    }
  }
}

pub struct DestructuringErrors {
  pub shorthandAssign: isize,
  pub trailingComma: isize,
  pub parenthesizedAssign: isize,
  parenthesizedBind: isize,
  doubleProto: isize,
}

impl DestructuringErrors {
  pub fn new() -> Self {
    DestructuringErrors {
      shorthandAssign: -1,
      trailingComma: -1,
      parenthesizedAssign: -1,
      parenthesizedBind: -1,
      doubleProto: -1,
    }
  }

  pub fn reset(self) {
    self.shorthandAssign = -1;
    self.trailingComma = -1;
    self.parenthesizedAssign = -1;
    self.parenthesizedBind = -1;
    self.doubleProto = -1;
  }
}
