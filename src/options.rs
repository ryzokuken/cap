use crate::locutil;
use crate::node;

#[derive(PartialEq, PartialOrd)]
pub enum EcmaVersion {
    Ecma3,
    Ecma5,
    Ecma6,
    Ecma7,
    Ecma8,
    Ecma9,
    Ecma10,
    Ecma2015,
    Ecma2016,
    Ecma2017,
    Ecma2018,
    Ecma2019,
}

impl Default for EcmaVersion {
    fn default() -> Self {
        EcmaVersion::Ecma9
    }
}

#[derive(PartialEq)]
pub enum SourceType {
    Script,
    Module,
}

impl Default for SourceType {
    fn default() -> Self {
        SourceType::Script
    }
}

// TODO(ryzokuken): onToken
// TODO(ryzokuken): onComment
#[derive(Default)]
pub struct Options {
    pub ecmaVersion: EcmaVersion,
    pub sourceType: SourceType,
    onInsertedSemicolon: Option<fn(u32, Option<locutil::Position>) -> ()>,
    onTrailingComma: Option<fn(u32, Option<locutil::Position>) -> ()>,
    allowReserved: Option<bool>,
    allowReturnOutsideFunction: bool,
    allowImportExportEverywhere: bool,
    allowAwaitOutsideFunction: bool,
    allowHashBang: bool,
    pub locations: bool,
    pub ranges: bool,
    pub program: Option<node::Node>,
    pub sourceFile: Option<String>,
    directSourceFile: Option<String>,
    preserveParens: bool,
}
