bitflags! {
    /// Each scope gets a bitset that may contain these flags
    pub struct Flags: u8 {
        const Zero = 0b0000_0000;
        const Top = 0b0000_0001;
        const Function = 0b0000_0010;
        const Var = 0b0000_0011;
        const Async = 0b0000_0100;
        const Generator = 0b0000_1000;
        const Arrow = 0b0001_0000;
        const SimpleCatch = 0b0010_0000;
        const Super = 0b0100_0000;
        const DirectSuper = 0b1000_0000;
    }
}

pub fn function_flags(is_async: bool, is_generator: bool) -> Flags {
    let async_flag = if is_async { Flags::Async } else { Flags::Zero };
    let generator_flag = if is_generator {
        Flags::Generator
    } else {
        Flags::Zero
    };
    Flags::Function | async_flag | generator_flag
}

/// Used in checkLVal and declareName to determine the type of a binding
enum Binds {
    /// Not a binding
    None,
    /// Var-style binding
    Var,
    /// Let- or const-style binding
    Lexical,
    /// Function declaration
    Function,
    /// Simple (identifier pattern) catch binding
    SimpleCatch,
    /// Special case for function names as bound inside the function
    Outside,
}
