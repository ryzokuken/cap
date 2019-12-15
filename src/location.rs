use crate::locutil;
use crate::state;

pub struct SyntaxError {
  pos: usize,
  loc: locutil::Position,
  raisedAt: usize,
  message: String,
}

pub trait ParserLocation {
  /// This function is used to raise exceptions on parse errors. It
  /// takes an offset integer (into the current `input`) to indicate
  /// the location of the error, attaches the position to the end
  /// of the error message, and then raises a `SyntaxError` with that
  /// message.
  fn raise(self, pos: usize, message: String) -> Result<(), SyntaxError>;
  fn raiseRecoverable(self, pos: usize, message: String) -> Result<(), SyntaxError>;
  fn curPosition(self) -> Option<locutil::Position>;
}

impl ParserLocation for state::Parser {
  fn raise(self, pos: usize, message: String) -> Result<(), SyntaxError> {
    let loc = locutil::getLineInfo(self.input, pos);
    message += &format!(" (#{}:#{})", loc.line, loc.column);
    Err(SyntaxError {
      pos,
      loc,
      raisedAt: pos,
      message,
    })
  }

  fn raiseRecoverable(self, pos: usize, message: String) -> Result<(), SyntaxError> {
    self.raise(pos, message)
  }

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
