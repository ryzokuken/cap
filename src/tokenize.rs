use crate::state;
use crate::tokencontext;
use crate::whitespace;
use std::char;

struct Token {}

pub trait ParserTokenize {
    // fn next() -> ();
    // fn getToken() -> Token;
    fn curContext(&self) -> Option<&tokencontext::TokContext>;
    fn nextToken(&self) -> ();
    fn skipSpace(&self) -> ();
    fn skipLineComment(&self, startSkip: usize) -> ();
}

impl ParserTokenize for state::Parser {
    fn curContext(&self) -> Option<&tokencontext::TokContext> {
        self.context.last()
    }

    fn nextToken(&self) {
        let curContext = self.curContext();
        if curContext.is_none() || !curContext.unwrap().preserveSpace {
            self.skipSpace();
        }
    }

    // TODO(ryzokuken): why do they break and not return?
    fn skipSpace(&self) {
        while self.pos < self.input.len() {
            let chars = self.input.encode_utf16();
            let ch = chars.nth(self.pos).unwrap();
            match ch {
                32 | 160 => self.pos += 1,
                13 => {
                    if chars.nth(self.pos + 1).unwrap() == 10 {
                        self.pos += 1;
                    }
                }
                10 | 8232 | 8233 => {
                    self.pos += 1;
                    if self.options.locations {
                        self.curLine += 1;
                        self.lineStart = self.pos;
                    }
                }
                47 => match chars.nth(self.pos + 1).unwrap() {
                    42 => self.skipBlockComment(),
                    47 => self.skipLineComment(2),
                    _ => return,
                },
                _ => {
                    let chr = char::from_u32(ch as u32).unwrap();
                    if ch > 8 && ch < 14
                        || ch >= 5760 && chr.is_whitespace() && !chr.is_ascii_whitespace()
                    {
                        self.pos += 1;
                    } else {
                        return;
                    }
                }
            }
        }
    }

    // TODO(ryzokuken): contemplate life and decide if onComment is required.
    fn skipLineComment(&self, startSkip: usize) {
        let start = self.pos;
        // let startLoc = self.options.onComment && self.curPosition();
        self.pos += startSkip;
        let chars = self.input.chars();
        let ch = chars.nth(self.pos);
        while self.pos < self.input.len() && !whitespace::is_newline(ch.unwrap(), false) {
            self.pos += 1;
            ch = chars.nth(self.pos);
        }
        // if self.options.onComment.is_some() {
        //     self.options.onComment(
        //         false,
        //         self.input.as_str()[start + startSkip..self.pos],
        //         start,
        //         self.pos,
        //         startLoc,
        //         self.curPosition(),
        //     );
        // }
    }
}
