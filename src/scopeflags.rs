const SCOPE_TOP: u8 = 1;
const SCOPE_FUNCTION: u8 = 2;
const SCOPE_VAR: u8 = SCOPE_TOP | SCOPE_FUNCTION;
const SCOPE_ASYNC: u8 = 4;
const SCOPE_GENERATOR: u8 = 8;
const SCOPE_ARROW: u8 = 16;
const SCOPE_SIMPLE_CATCH: u8 = 32;
const SCOPE_SUPER: u8 = 64;
const SCOPE_DIRECT_SUPER: u8 = 128;

pub fn function_flags(async: bool, generator: bool) -> u8 {
    let is_async = if async { SCOPE_ASYNC } else { 0u8 };
    let is_generator = if generator { SCOPE_GENERATOR } else { 0u8 };
    SCOPE_FUNCTION | is_async | is_generator
}

const BIND_NONE: u8 = 0;
const BIND_VAR: u8 = 1;
const BIND_LEXICAL: u8 = 2;
const BIND_FUNCTION: u8 = 3;
const BIND_SIMPLE_CATCH: u8 = 4;
const BIND_OUTSIDE: u8 = 5;