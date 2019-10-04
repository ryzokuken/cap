#[macro_use]
extern crate bitflags;

bitflags! {
    pub struct Scopes: u8 {
        const SCOPE_ZERO = 0b00000000;
        const SCOPE_TOP = 0b00000001;
        const SCOPE_FUNCTION = 0b00000010;
        const SCOPE_VAR = 0b00000011;
        const SCOPE_ASYNC = 0b00000100;
        const SCOPE_GENERATOR = 0b000010000;
        const SCOPE_ARROW = 0b00010000;
        const SCOPE_SIMPLE_CATCH = 0b00100000;
        const SCOPE_SUPER = 0b01000000;
        const SCOPE_DIRECT_SUPER = 0b10000000;
    }
}

pub fn function_flags(is_async: bool, is_generator: bool) -> Scopes {
    let async_flag = if is_async { Scopes::SCOPE_ASYNC } else { Scopes::SCOPE_ZERO };
    let generator_flag = if is_generator { Scopes::SCOPE_GENERATOR } else { Scopes::SCOPE_ZERO };
    Scopes::SCOPE_FUNCTION | async_flag | generator_flag
}

bitflags! {
    pub struct Binds: u8 {
        const BIND_NONE = 0b00000000;
        const BIND_VAR = 0b00000001;
        const BIND_LEXICAL = 0b00000010;
        const BIND_FUNCTION = 0b00000011;
        const BIND_SIMPLE_CATCH = 0b00000100;
        const BIND_OUTSIDE = 0b00000101;
    }
}
