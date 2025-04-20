use syn::{parse::{Parse, ParseStream}, Ident, Result as SynResult};

struct MacroArgs {
    method: Ident,
    path: String
}

impl Parse for MacroArgs {
    fn parse(input: ParseStream) -> SynResult<Self> {
        todo!()
    }
}
