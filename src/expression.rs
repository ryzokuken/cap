// A recursive descent parser operates by defining functions for all
// syntactic elements, and recursively calling those, each function
// advancing the input stream and returning an AST node. Precedence
// of constructs (for example, the fact that `!x[1]` means `!(x[1])`
// instead of `(!x)[1]` is handled by the fact that the parser
// function that parses unary prefix operators is called first, and
// in turn calls the function that parses `[]` subscripts — that
// way, it'll receive the node for `x[1]` already parsed, and wraps
// *that* in the unary operator node.
//
// Acorn uses an [operator precedence parser][opp] to handle binary
// operator precedence, because it is much more compact than using
// the technique outlined above, which uses different, nesting
// functions to specify precedence, for all of the ten binary
// precedence levels that JavaScript defines.
//
// [opp]: http://en.wikipedia.org/wiki/Operator-precedence_parser

use crate::location;
use crate::locutil;
use crate::lval;
use crate::node;
use crate::parseutil;
use crate::state;
use crate::tokentype;

use location::ParserLocation;
use lval::ParserLval;
use node::ParserNode;
use parseutil::ParserParseUtil;

pub trait ParserExpression {
  // ### Expression parsing

  // These nest, from the most general expression type at the top to
  // 'atomic', nondivisible expression types at the bottom. Most of
  // the functions will simply let the function(s) below them parse,
  // and, *if* the syntactic construct they handle is present, wrap
  // the AST node that the inner parser gave them in another node.

  /// Parse a full expression. The optional arguments are used to
  /// forbid the `in` operator (in for loops initalization expressions)
  /// and provide reference for storing '=' operator inside shorthand
  /// property assignment in contexts where both object expression
  /// and object pattern might appear (so it's possible to raise
  /// delayed syntax error at correct position).
  fn parseExpression(
    self,
    noIn: Option<bool>,
    refDestructuringErrors: Option<parseutil::DestructuringErrors>,
  ) -> node::Node;

  /// Parse an assignment expression. This includes applications of
  /// operators like `+=`.
  fn parseMaybeAssign(
    self,
    noIn: Option<bool>,
    refDestructuringErrors: Option<parseutil::DestructuringErrors>,
    afterLeftParse: Option<fn(state::Parser, node::Node, usize, locutil::Position)>,
  ) -> node::Node;

  /// Parse a ternary conditional (`?:`) operator.
  fn parseMaybeConditional(
    self,
    noIn: Option<bool>,
    refDestructuringErrors: Option<parseutil::DestructuringErrors>,
  ) -> node::Node;

  /// Start the precedence parser.
  fn parseExprOps(
    self,
    noIn: Option<bool>,
    refDestructuringErrors: Option<parseutil::DestructuringErrors>,
  ) -> node::Node;

  /// Parse binary operators with the operator precedence parsing
  /// algorithm. `left` is the left-hand side of the operator.
  /// `minPrec` provides context that allows the function to stop and
  /// defer further parser to one of its callers when it encounters an
  /// operator that has a lower precedence than the set it is parsing.
  fn parseExprOp(
    self,
    left: node::Node,
    leftStartPos: usize,
    leftStartLoc: locutil::Position,
    minPrec: isize,
    noIn: bool,
  ) -> node::Node;

  /// Parse unary operators, both prefix and postfix.
  fn parseMaybeUnary(
    self,
    refDestructuringErrors: Option<parseutil::DestructuringErrors>,
    sawUnary: bool,
  ) -> node::Node;

  fn parseYield(
    self,
    noIn: Option<bool>
  ) -> node::Node;

  fn parseAwait(self) -> node::Node;
}

impl ParserExpression for state::Parser {
  fn parseExpression(
    self,
    noIn: Option<bool>,
    refDestructuringErrors: Option<parseutil::DestructuringErrors>,
  ) -> node::Node {
    let startPos = self.pos;
    let startLoc = self.startLoc;
    let expr = self.parseMaybeAssign(noIn, refDestructuringErrors, None);
    if self.r#type == tokentype::TokenType::comma() {
      let node = self.startNodeAt(startPos, startLoc);
      node.expressions = [expr].to_vec();
      while self.eat(tokentype::TokenType::comma()) {
        node
          .expressions
          .push(self.parseMaybeAssign(noIn, refDestructuringErrors, None))
      }
      return self.finishNode(node, String::from("SequenceExpression"));
    }
    expr
  }

  fn parseMaybeAssign(
    self,
    noIn: Option<bool>,
    refDestructuringErrors: Option<parseutil::DestructuringErrors>,
    afterLeftParse: Option<fn(state::Parser, node::Node, usize, locutil::Position)>,
  ) -> node::Node {
    if self.isContextual("yield") {
      if self.inGenerator() {
        return self.parseYield(noIn);
      } else {
        // The tokenizer will assume an expression is allowed after
        // `yield`, but this isn't that kind of yield
        self.exprAllowed = false;
      }
    }

    let mut ownDestructuringErrors = false;
    let mut oldParenAssign = -1;
    let mut oldTrailingComma = -1;
    let mut oldShorthandAssign = -1;
    if refDestructuringErrors.is_some() {
      oldParenAssign = refDestructuringErrors.unwrap().parenthesizedAssign;
      oldTrailingComma = refDestructuringErrors.unwrap().trailingComma;
      oldShorthandAssign = refDestructuringErrors.unwrap().shorthandAssign;
      refDestructuringErrors.unwrap().parenthesizedAssign = -1;
      refDestructuringErrors.unwrap().trailingComma = -1;
      refDestructuringErrors.unwrap().shorthandAssign = -1;
    } else {
      refDestructuringErrors = Some(parseutil::DestructuringErrors::new());
      ownDestructuringErrors = true;
    }

    let startPos = self.start;
    let startLoc = self.startLoc;
    if self.r#type == tokentype::TokenType::parenL() || self.r#type == tokentype::TokenType::name()
    {
      self.potentialArrowAt = self.start as isize;
    }
    let mut left = self.parseMaybeConditional(noIn, refDestructuringErrors);
    if afterLeftParse.is_some() {
      afterLeftParse.unwrap()(self, left, startPos, startLoc.unwrap());
    }
    if self.r#type.isAssign {
      let node = self.startNodeAt(startPos, startLoc);
      node.operator = self.value;
      node.left = if self.r#type == tokentype::TokenType::eq() {
        Some(Box::new(
          self
            .toAssignable(Some(left), false, refDestructuringErrors)
            .unwrap(),
        ))
      } else {
        Some(Box::new(left))
      };
      if !ownDestructuringErrors {
        refDestructuringErrors.unwrap().reset();
      }
      refDestructuringErrors.unwrap().shorthandAssign = -1;
      self.checkLVal(left);
      self.next();
      node.right = Some(Box::new(self.parseMaybeAssign(noIn, None, None)));
      return self.finishNode(node, String::from("AssignmentExpression"));
    } else if ownDestructuringErrors {
      self.checkExpressionErrors(refDestructuringErrors, true);
    }
    if oldParenAssign > -1 {
      refDestructuringErrors.unwrap().parenthesizedAssign = oldParenAssign;
    }
    if oldTrailingComma > -1 {
      refDestructuringErrors.unwrap().trailingComma = oldTrailingComma;
    }
    if oldShorthandAssign > -1 {
      refDestructuringErrors.unwrap().shorthandAssign = oldShorthandAssign;
    }
    left
  }

  fn parseMaybeConditional(
    self,
    noIn: Option<bool>,
    refDestructuringErrors: Option<parseutil::DestructuringErrors>,
  ) -> node::Node {
    let startPos = self.start;
    let startLoc = self.startLoc;
    let expr = self.parseExprOps(noIn, refDestructuringErrors);
    if self.checkExpressionErrors(refDestructuringErrors, false) {
      return expr;
    }
    if self.eat(tokentype::TokenType::question()) {
      let node = self.startNodeAt(startPos, startLoc);
      node.test = Some(Box::new(expr));
      node.consequent = Some(Box::new(self.parseMaybeAssign(None, None, None)));
      self.expect(tokentype::TokenType::colon());
      node.alternate = Some(Box::new(self.parseMaybeAssign(noIn, None, None)));
      return self.finishNode(node, String::from("ConditionalExpression"));
    }
    expr
  }

  fn parseExprOps(
    self,
    noIn: Option<bool>,
    refDestructuringErrors: Option<parseutil::DestructuringErrors>,
  ) -> node::Node {
    let startPos = self.start;
    let startLoc = self.startLoc;
    let expr = self.parseMaybeUnary(refDestructuringErrors, false);
    if self.checkExpressionErrors(refDestructuringErrors, false) {
      return expr;
    }
    if expr.start == startPos && expr.r#type == "ArrowFunctionExpression" {
      expr
    } else {
      self.parseExprOp(expr, startPos, startLoc.unwrap(), -1, noIn.unwrap())
    }
  }

  fn parseExprOp(
    self,
    left: node::Node,
    leftStartPos: usize,
    leftStartLoc: locutil::Position,
    minPrec: isize,
    noIn: bool,
  ) -> node::Node {
    let prec = self.r#type.binop;
    if prec.is_some()
      && (!noIn || self.r#type != tokentype::TokenType::_in())
      && prec.unwrap() as isize > minPrec
    {
      let logical = self.r#type == tokentype::TokenType::logicalOR()
        || self.r#type == tokentype::TokenType::logicalAND();
      let op = self.value;
      self.next();
      let right = self.parseExprOp(
        self.parseMaybeUnary(None, false),
        self.start,
        self.startLoc.unwrap(),
        prec.unwrap() as isize,
        noIn,
      );
      let node = self.buildBinary(leftStartPos, leftStartLoc, left, right, op, logical);
      return self.parseExprOp(node, leftStartPos, leftStartLoc, minPrec, noIn);
    }
    left
  }

  fn parseMaybeUnary(
    self,
    refDestructuringErrors: Option<parseutil::DestructuringErrors>,
    sawUnary: bool,
  ) -> node::Node {
    let startPos = self.start;
    let startLoc = self.startLoc;
    let mut expr: node::Node;

    if self.isContextual("await")
      && (self.inAsync() || (!self.inFunction() && self.options.allowAwaitOutsideFunction))
    {
      expr = self.parseAwait();
      sawUnary = true;
    } else if self.r#type.prefix {
      let node = self.startNode();
      let update = self.r#type == tokentype::TokenType::incDec();
      node.operator = self.value;
      node.prefix = Some(true);
      self.next();
      node.argument = Some(Box::new(self.parseMaybeUnary(None, true)));
      self.checkExpressionErrors(refDestructuringErrors, true);
      if update {
        self.checkLVal(node.argument);
      } else if self.strict
        && node.operator.unwrap() == String::from("delete")
        && node.argument.unwrap().r#type == "Identifier"
      {
        self.raiseRecoverable(
          node.start,
          String::from("Deleting local variable in strict mode"),
        );
      } else {
        sawUnary = false;
      }
      expr = self.finishNode(
        node,
        String::from(if update {
          "UpdateExpression"
        } else {
          "UnaryExpression"
        }),
      )
    } else {
      expr = self.parseExprSubscripts(refDestructuringErrors);
      if self.checkExpressionErrors(refDestructuringErrors, false) {
        return expr;
      }
      while self.r#type.postfix && !self.canInsertSemicolon() {
        let node = self.startNodeAt(startPos, startLoc);
        node.operator = self.value;
        node.prefix = Some(false);
        node.argument = Some(Box::new(expr));
        self.checkLVal(expr);
        self.next();
        expr = self.finishNode(node, String::from("UpdateExpression"));
      }
    }

    if !sawUnary && self.eat(tokentype::TokenType::starstar()) {
      self.buildBinary(
        startPos,
        startLoc,
        expr,
        self.parseMaybeUnary(None, false),
        "**",
        false,
      )
    } else {
      expr
    }
  }

  fn parseYield (
    self,
    noIn: Option<bool>
  ) -> node::Node {
    if !self.yieldPos {
        self.yieldPos = self.start;
    }

    let node = self.startNode();
    self.next();
    if self.r#type == tokentype::TokenType::semi() || self.canInsertSemiColon() || (self.r#type != tokentype::TokenType::star() && !self.r#type.startsExpr) {
      node.delegate = false;
      node.argument = None;
    } else {
      node.delegate = self.eat(tokentype::TokenType::star());
      node.argument = Some(Box::new(self.parseMaybeAssign(noIn, None, None)));
    }
    self.finishNode(node, String::from("YieldExpression"))
  }

  fn parseAwait (self) -> node::Node {
      if !self.awaitPos {
          self.awaitPos = self.start;
      }

      let node = self.startNode();
      self.next();
      node.argument = Some(Box::new(self.parseMaybeUnary(None, true)));
      self.finishNode(node, String::from("AwaitExpression"))
  }
}
