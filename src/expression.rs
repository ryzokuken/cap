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

use crate::node;
use crate::parseutil;
use crate::state;
use crate::tokentype;

use node::ParserNode;
use parseutil::ParserParseUtil;

pub trait ParserExpression {
  // ### Expression parsing

  // These nest, from the most general expression type at the top to
  // 'atomic', nondivisible expression types at the bottom. Most of
  // the functions will simply let the function(s) below them parse,
  // and, *if* the syntactic construct they handle is present, wrap
  // the AST node that the inner parser gave them in another node.

  // Parse a full expression. The optional arguments are used to
  // forbid the `in` operator (in for loops initalization expressions)
  // and provide reference for storing '=' operator inside shorthand
  // property assignment in contexts where both object expression
  // and object pattern might appear (so it's possible to raise
  // delayed syntax error at correct position).
  fn parseExpression(
    self,
    noIn: Option<bool>,
    refDestructuringErrors: Option<parseutil::DestructuringErrors>,
  ) -> node::Node;

  // Parse an assignment expression. This includes applications of
  // operators like `+=`.
  fn parseMaybeAssign(
    self,
    noIn: Option<bool>,
    refDestructuringErrors: Option<parseutil::DestructuringErrors>,
  ) -> node::Node;
}

impl ParserExpression for state::Parser {
  fn parseExpression(
    self,
    noIn: Option<bool>,
    refDestructuringErrors: Option<parseutil::DestructuringErrors>,
  ) -> node::Node {
    let startPos = self.pos;
    let startLoc = self.startLoc;
    let expr = self.parseMaybeAssign(noIn, refDestructuringErrors);
    if self.r#type == tokentype::TokenType::comma() {
      let node = self.startNodeAt(startPos, startLoc);
      node.expressions = [expr].to_vec();
      while self.eat(tokentype::TokenType::comma()) {
        node
          .expressions
          .push(self.parseMaybeAssign(noIn, refDestructuringErrors))
      }
      return self.finishNode(node, String::from("SequenceExpression"));
    }
    expr
  }
}
