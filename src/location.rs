use crate::locutil;
use crate::state;

trait ParserLocation {
  fn curPosition(self) -> Option<locutil::Position>;
}

impl ParserLocation for state::Parser {
  fn curPosition(self) -> Option<locutil::Position> {
    if self.options.locations {
      return Some(locutil::Position::new(
        self.curLine,
        self.pos - self.lineStart,
      ));
    }
    None
  }
}
