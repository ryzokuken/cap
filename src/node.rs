use crate::locutil;
use crate::state;

pub struct Node {
    r#type: String,
    start: u32,
    end: u32,
    loc: Option<locutil::SourceLocation>,
    sourceFile: Option<String>,
    range: Option<(u32, u32)>,
}

impl Node {
    fn new(parser: state::Parser, pos: u32, loc: Option<locutil::SourceLocation>) -> Node {
        let node = Node {
            r#type: String::from(""),
            start: pos,
            end: 0,
            loc: None,
            sourceFile: None,
            range: None,
        };
        if parser.options.locations {
            // TODO(ryzokuken): Talk to acorn maintainers about this.
            node.loc = loc;
        }
        // TODO: finish this.
        node
    }
}
