use crate::state;

pub struct Position {
    line: u32,
    column: u32,
}

impl Position {
    fn new(line: u32, column: u32) -> Self {
        Position { line, column }
    }

    fn offset(&self, n: u32) -> Self {
        Position::new(self.line, self.column + n)
    }
}

pub struct SourceLocation {
    start: Position,
    end: Position,
    source: String,
}

impl SourceLocation {
    pub fn new(p: state::Parser, start: Position, end: Position) -> Self {
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
}
