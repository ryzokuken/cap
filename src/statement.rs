use crate::node;
use crate::options;
use crate::state;
use crate::tokentype;

use std::collections::HashMap;

use node::ParserNode;

// ### Statement parsing

// Parse a program. Initializes the parser, reads any number of
// statements, and wraps them in a Program node.  Optionally takes a
// `program` argument.  If present, the statements will be appended
// to its body instead of creating a new node.

pub trait ParserStatement {
    fn parseTopLevel(self, node: node::Node) -> node::Node;
}

impl ParserStatement for state::Parser {
    fn parseTopLevel(self, node: node::Node) -> node::Node {
        let exports: HashMap<String, bool> = HashMap::new();
        if node.body.is_none() {
            node.body = Some(Vec::new());
        }
        while self.r#type != tokentype::TokenType::eof() {
            let stmt = self.parseStatement(None, true, exports);
            node.body.expect("").push(stmt);
        }
        if self.inModule {
            for name in self.undefinedExports.keys() {
                self.raiseRecoverable(
                    self.undefinedExports[name].start,
                    format!("Export #{} is not defined", name),
                );
            }
        }
        self.adaptDirectivePrologue(node.body);
        self.next();
        if self.options.ecmaVersion >= options::EcmaVersion::Ecma6 {
            node.sourceType = Some(self.options.sourceType);
        }
        self.finishNode(node, String::from("Program"))
    }
}
