use crate::state;

#[derive(Default)]
pub struct TokContext {
    token: String,
    isExpr: bool,
    pub preserveSpace: bool,
    overrideFn: Option<fn(state::Parser)>,
    generator: bool,
}

impl TokContext {
    // fn new(
    //     token: String,
    //     isExpr: bool,
    //     preserveSpace: bool,
    //     overrideFn: Option<fn(state::Parser)>,
    //     generator: bool,
    // ) -> Self {
    // }

    pub fn b_stat() -> Self {
        TokContext {
            token: String::from("{"),
            isExpr: false,
            ..Default::default()
        }
    }
}

// trait ParserTokenContext {
//     // TODO(ryzokuken): why is this not static again?
//     fn initialContext(&self) -> Vec<TokContext> {
//         vec![TokContext::b_stat()]
//     }
// }
