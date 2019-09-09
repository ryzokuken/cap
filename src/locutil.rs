// TODO import lineBreakG fn from whitespace
// TODO define types for all fn parameters and structs
// TODO add getLineInfo fn

// These are used when `options.locations` is on, for the
// `startLoc` and `endLoc` properties.

mod locutil {
    pub struct Position {
        line: u64,
        column: u64,
    }

    impl Position {
        pub fn new(line: u64, column: u64) -> Position {
            Position{ line: line, column: column }
        }

        pub fn offset(&self, n) -> Position {
            Position{ line: self.line, column: self.column + n}
        }
    }

    pub struct SourceLocation {
        start,
        end,
        source,
    }

    impl SourceLocation {
        pub fn new(p, start, end) -> SourceLocation {
            if p.sourceFile != () { // TODO what should the empty value be ?
                SourceLocation{ start: start, end: end, sourceFile: p.sourceFile }
            } else {
                SourceLocation{ start: start, end: end, sourceFile: () }
            }
        }
    }
}
