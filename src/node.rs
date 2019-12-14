use crate::locutil;
use crate::options;
use crate::state;

pub struct Node {
    r#type: String,
    pub start: usize,
    end: usize,
    loc: Option<locutil::SourceLocation>,
    sourceFile: Option<String>,
    range: Option<(usize, usize)>,
    pub body: Option<Vec<Node>>,
    pub local: Option<Box<Node>>,
    pub sourceType: Option<options::SourceType>,
}

impl Node {
    fn new(parser: state::Parser, pos: usize, loc: Option<locutil::SourceLocation>) -> Self {
        let node = Node {
            r#type: String::from(""),
            start: pos,
            end: 0,
            loc: None,
            sourceFile: None,
            range: None,
            body: None,
            local: None,
            sourceType: None,
        };
        if parser.options.locations {
            // TODO(ryzokuken): Talk to acorn maintainers about this.
            node.loc = loc;
        }
        // TODO: finish this.
        node
    }
}

pub trait ParserNode {
    fn startNode(self) -> Node;
    fn finishNode(self, node: Node, r#type: String) -> Node;
}

impl ParserNode for state::Parser {
    fn startNode(self) -> Node {
        Node::new(
            self,
            self.start,
            Some(locutil::SourceLocation::from_parser(self)),
        )
    }

    fn finishNode(self, node: Node, r#type: String) -> Node {
        finishNodeAt(self, node, r#type, self.lastTokEnd, self.lastTokEndLoc)
    }
}

// Finish an AST node, adding `type` and `end` properties.
fn finishNodeAt(
    parser: state::Parser,
    node: Node,
    r#type: String,
    pos: usize,
    loc: Option<locutil::Position>,
) -> Node {
    node.r#type = r#type;
    node.end = pos;
    if parser.options.locations {
        node.loc.unwrap().end = loc.unwrap();
    }
    if parser.options.ranges {
        node.range.unwrap().1 = pos;
    }
    node
}
