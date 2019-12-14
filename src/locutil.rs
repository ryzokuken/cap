use crate::state;

#[derive(Clone)]
pub struct Position {
    line: usize,
    column: usize,
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
