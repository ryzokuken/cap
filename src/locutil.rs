use crate::state;
use crate::whitespace;

#[derive(Clone)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl Position {
    pub fn new(line: usize, column: usize) -> Self {
        Position { line, column }
    }

    fn offset(&self, n: usize) -> Self {
        Position::new(self.line, self.column + n)
    }
}

#[derive(Clone)]
pub struct SourceLocation {
    start: Option<Position>,
    pub end: Option<Position>,
    source: String,
}

impl SourceLocation {
    pub fn new(p: &state::Parser, start: Option<Position>, end: Option<Position>) -> Self {
        let loc = SourceLocation {
            start,
            end,
            source: String::from(""),
        };
        if p.options.sourceFile.is_some() {
            loc.source = p.options.sourceFile.unwrap();
        }
        loc
    }

    pub fn from_parser(p: &state::Parser) -> Self {
        SourceLocation::new(p, p.startLoc, p.endLoc)
    }
}

/// The `getLineInfo` function is mostly useful when the
/// `locations` option is off (for performance reasons) and you
/// want to find the line/column position for a given character
/// offset. `input` should be the code string that the offset refers
/// into.
pub fn getLineInfo(input: String, offset: usize) -> Position {
    let line = 1;
    let cur = 0;
    loop {
        let mat = whitespace::lineBreak.find_at(input.as_str(), cur);
        if mat.is_some() && mat.unwrap().start() < offset {
            line += 1;
            cur = mat.unwrap().end();
        } else {
            return Position::new(line, offset - cur);
        }
    }
}
