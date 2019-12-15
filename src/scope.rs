use crate::scopeflags;
use crate::state;

use scopeflags::Flags;

pub struct Scope {
  pub flags: Flags,
  /// A list of var-declared names in the current lexical scope
  var: Vec<String>,
  /// A list of lexically-declared names in the current lexical scope
  lexical: Vec<String>,
  /// A list of lexically-declared FunctionDeclaration names in the current lexical scope
  functions: Vec<String>,
}

impl Scope {
  fn new(flags: Flags) -> Self {
    Scope {
      flags,
      var: Vec::new(),
      lexical: Vec::new(),
      functions: Vec::new(),
    }
  }
}

pub trait ParserScope {
  fn currentVarScope(self) -> Option<Scope>;
}

impl ParserScope for state::Parser {
  fn currentVarScope(self) -> Option<Scope> {
    for &scope in self.scopeStack.iter().rev() {
      if (scope.flags & Flags::Var) != Flags::Zero {
        return Some(scope);
      }
    }
    None
  }
}
