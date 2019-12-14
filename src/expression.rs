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

pub trait ParserExpression {
  fn parseExpression(
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
    let expr = self.maybeAssign(noIn, refDestructuringErrors);
    if self.r#type == tokentype::TokenType::comma() {
      let node = self.startNodeAt(startPos, startLoc);
      // TODO: finish this
    }
    expr
  }
}
