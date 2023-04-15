use kind_syntax::concrete::Expr;
use crate::{Result, FmtContext};

impl<'a> FmtContext<'a> {
    pub fn expr(&mut self) -> Result<Expr> {
        match self.kind() {
            "call" => {
                let callee = self.property("callee").unwrap();

                self.cursor(callee).primary()
            },
            kind => todo!("{}", kind),
        }
    }
}