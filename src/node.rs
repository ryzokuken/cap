use crate::locutil;
use crate::options;
use crate::state;

#[derive(Clone)]
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
    pub expressions: Vec<Node>,
}

impl Node {
    fn new(parser: state::Parser, pos: usize, loc: Option<locutil::Position>) -> Self {
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
            expressions: Vec::new(),
        };
        if parser.options.locations {
            node.loc = Some(locutil::SourceLocation::new(&parser, loc, None));
        }
        // TODO: finish this.
        node
    }
}

pub trait ParserNode {
    fn startNode(self) -> Node;
    fn startNodeAt(self, pos: usize, loc: Option<locutil::Position>) -> Node;
    fn finishNode(self, node: Node, r#type: String) -> Node;
    fn finishNodeAt(
        self,
        node: Node,
        r#type: String,
        pos: usize,
        loc: Option<locutil::Position>,
    ) -> Node;
}

impl ParserNode for state::Parser {
    fn startNode(self) -> Node {
        Node::new(self, self.start, self.startLoc)
    }

    fn startNodeAt(self, pos: usize, loc: Option<locutil::Position>) -> Node {
        Node::new(self, pos, loc)
    }

    fn finishNode(self, node: Node, r#type: String) -> Node {
        finishNodeAt(self, node, r#type, self.lastTokEnd, self.lastTokEndLoc)
    }

    fn finishNodeAt(
        self,
        node: Node,
        r#type: String,
        pos: usize,
        loc: Option<locutil::Position>,
    ) -> Node {
        finishNodeAt(self, node, r#type, pos, loc)
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
        node.loc.unwrap().end = loc;
    }
    if parser.options.ranges {
        node.range.unwrap().1 = pos;
    }
    node
}
