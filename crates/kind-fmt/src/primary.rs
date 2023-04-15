use kind_syntax::concrete::Expr;
use kind_syntax::lexemes::Span;
use crate::{FmtContext, Result};

impl<'a> FmtContext<'a> {
    pub fn primary(&mut self) -> Result<Expr> {
        match self.kind() {
            "constructor_identifier" => {
                let name = self.name()?;
                Ok(Expr::constructor(Span::default(), name))
            }
            "identifier" => {
                let name = self.name()?;
                Ok(Expr::local(Span::default(), name))
            }
            kind => todo!("{}", kind),
        }
    }
}