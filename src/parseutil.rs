pub struct DestructuringErrors {
  shorthandAssign: isize,
  trailingComma: isize,
  parenthesizedAssign: isize,
  parenthesizedBind: isize,
  doubleProto: isize,
}

impl DestructuringErrors {
  fn new() -> Self {
    DestructuringErrors {
      shorthandAssign: -1,
      trailingComma: -1,
      parenthesizedAssign: -1,
      parenthesizedBind: -1,
      doubleProto: -1,
    }
  }
}
