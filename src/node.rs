use crate::locutil;
use crate::options;
use crate::state;

#[derive(Clone)]
pub struct Node {
    pub r#type: String,
    pub start: usize,
    pub end: usize,
    loc: Option<locutil::SourceLocation>,
    sourceFile: Option<String>,
    range: Option<(usize, usize)>,
    pub body: Option<Vec<Node>>,
    pub local: Option<Box<Node>>,
    pub sourceType: Option<options::SourceType>,
    pub expressions: Vec<Node>,

    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
    pub operator: Option<String>,
    pub name: Option<String>,
    pub properties: Option<Vec<Node>>,
    pub argument: Option<Box<Node>>,
    pub kind: Option<String>,
    pub value: Option<Box<Node>>,
    pub key: Option<Box<Node>>,
    pub elements: Option<Vec<Node>>,
    pub expression: Option<Box<Node>>,

    pub test: Option<Box<Node>>,
    pub consequent: Option<Box<Node>>,
    pub alternate: Option<Box<Node>>,
    pub prefix: Option<bool>,
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
            left: None,
            right: None,
            operator: None,
            name: None,
            properties: None,
            argument: None,
            kind: None,
            value: None,
            key: None,
            elements: None,
            expression: None,
            test: None,
            consequent: None,
            alternate: None,
            prefix: None,
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
