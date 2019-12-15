use crate::location;
use crate::node;
use crate::options;
use crate::parseutil;
use crate::state;

use location::ParserLocation;
use parseutil::ParserParseUtil;

pub trait ParserLval {
  fn toAssignable(
    self,
    node: Option<node::Node>,
    isBinding: bool,
    refDestructuringErrors: Option<parseutil::DestructuringErrors>,
  ) -> Option<node::Node>;
}

impl ParserLval for state::Parser {
  fn toAssignable(
    self,
    node: Option<node::Node>,
    isBinding: bool,
    refDestructuringErrors: Option<parseutil::DestructuringErrors>,
  ) -> Option<node::Node> {
    if self.options.ecmaVersion >= options::EcmaVersion::Ecma6 && node.is_some() {
      let node = node.unwrap();
      match node.r#type.as_str() {
        "Identifier" => {
          if self.inAsync() && node.name.unwrap() == "await" {
            self.raise(
              node.start,
              String::from("Cannot use 'await' as identifier inside an async function"),
            );
          }
        }
        "ObjectPattern" | "ArrayPattern" | "RestElement" => {}
        "ObjectExpression" => {
          node.r#type = String::from("ObjectPattern");
          if refDestructuringErrors.is_some() {
            self.checkPatternErrors(refDestructuringErrors, true);
          }
          for prop in node.properties.unwrap() {
            let arg = prop.argument.unwrap();
            self.toAssignable(Some(prop), isBinding, None);
            // Early error:
            //   AssignmentRestProperty[Yield, Await] :
            //     `...` DestructuringAssignmentTarget[Yield, Await]
            //
            //   It is a Syntax Error if |DestructuringAssignmentTarget| is an |ArrayLiteral| or an |ObjectLiteral|.
            if prop.r#type == "RestElement"
              && (arg.r#type == "ArrayPattern" || arg.r#type == "ObjectPattern")
            {
              self.raise(arg.start, String::from("Unexpected token"));
            }
          }
        }
        "Property" => {
          // AssignmentProperty has type === "Property"
          if node.kind != Some(String::from("init")) {
            self.raise(
              node.key.unwrap().start,
              String::from("Object pattern can't contain getter or setter"),
            );
          }
          self.toAssignable(Some(*node.value.unwrap()), isBinding, None);
        }
        "ArrayExpression" => {
          node.r#type = String::from("ArrayPattern");
          if refDestructuringErrors.is_some() {
            self.checkPatternErrors(refDestructuringErrors, true);
          }
          self.toAssignableList(node.elements, isBinding);
        }
        "SpreadElement" => {
          node.r#type = String::from("RestElement");
          self.toAssignable(Some(*node.argument.unwrap()), isBinding, None);
          if node.argument.unwrap().r#type == "AssignmentPattern" {
            self.raise(
              node.argument.unwrap().start,
              String::from("Rest elements cannot have a default value"),
            );
          }
        }
        "AssignmentExpression" => {
          if node.operator != Some(String::from("=")) {
            self.raise(
              node.left.unwrap().end,
              String::from("Only '=' operator can be used for specifying default value."),
            );
          }
          node.r#type = String::from("AssignmentPattern");
          node.operator = None;
          self.toAssignable(Some(*node.left.unwrap()), isBinding, None);
          // TODO: clarify on the fallthrough here?
        }
        "AssignmentPattern" => {}
        "ParenthesizedExpression" => {
          self.toAssignable(
            Some(*node.expression.unwrap()),
            isBinding,
            refDestructuringErrors,
          );
        }
        "MemberExpression" if !isBinding => {}
        _ => {
          self.raise(node.start, String::from("Assigning to rvalue"));
        }
      }
    } else if refDestructuringErrors.is_some() {
      self.checkPatternErrors(refDestructuringErrors, true);
    }
    node
  }
}
