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

// half-ass code

// todo type for 'afterLeftParse'
pub trait ParseMaybeAssign {
    fn parseMaybeAssign(self, noIn: Option<bool>, refDestructuringErrors: parseutil::DestructuringErrors, afterLeftParse) -> node::Node;
}

impl ParseMaybeAssign for state::Parser {
    fn parseMaybeAssign(self, noIn: Option<bool>, refDestructuringErrors: parseutil::DestructuringErrors, afterLeftParse: ) -> node::Node {
        if self.isContextual("yield") {
            if self.inGenerator {
                self.parseYield(noIn)
            }
        } else {
            // todo has to return same type as if
            // self.exprAllowed = false
        }

        let ownDestructuringErrors = false;
        let mut oldParenAssign = -1;
        let oldTrailingComma = -1;
        let oldShorthandAssign = -1;

        if refDestructuringErrors {
            oldParenAssign = refDestructuringErrors.parenthesizedAssign;
            oldTrailingComma = refDestructuringErrors.trailingComma;
            oldShorthandAssign = refDestructuringErrors.shorthandAssign
            refDestructuringErrors.parenthesizedAssign = -1;
            refDestructuringErrors.trailingComma = -1;
            refDestructuringErrors.shorthandAssign = -1;
        } else {
            // todo don't know what type of struct this is ?
            refDestructuringErrors = DestructuringErrors(); 
            ownDestructuringErrors = true;
        }

        let startPos = self.start;
        let startLoc = self.startLoc;
        if self.r#type == tokentype::TokenType::parenL() ||self.r#type == tokentype::TokenType::name() {
            self.potentialArrowAt = self.start;
        }
        let mut left = self.parseMaybeConditional(noIn, refDestructuringErrors);
        if afterLeftParse {
            left = afterLeftParse.call(self, left, startPos, startLoc);
        }
        // todo condition check 'isAssign' ?
        if self.r#type.isAssign {
            let node = self.startNodeAt(startPos, startLoc)
            node.operator = self.value;
            node.left = if self.r#type == tokentype::TokenType::eq { self.toAssignable(left, false, refDestructuringErrors) } else { left };

            if !ownDestructuringErrors {
                //todo don't know what this is
                DestructuringErrors.call(refDestructuringErrors)
            }
            refDestructuringErrors.shorthandAssign = -1
            self.checkLVal(left);
            self.next();
            node.right = self.parseMaybeAssign(noIn);
            self.finishNode(node, "AssignmentExpression")
        } else {
            if ownDestructuringErrors {
                self.checkExpressionErrors(refDestructuringErrors, true);
            }
        }
        if oldParenAssign > -1 {
            refDestructuringErrors.parenthesizedAssign = oldParenAssign;
        }
        if oldTrailingComma > -1 {
            refDestructuringErrors.trailingComma = oldTrailingComma;
        }
        if oldShorthandAssign > -1 {
            refDestructuringErrors.shorthandAssign = oldShorthandAssign;
        }
        left
    }
}
