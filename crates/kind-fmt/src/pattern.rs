use kind_syntax::concrete::{Pat, PatKind};
use kind_syntax::lexemes::Span;
use crate::{Result, FmtContext};

impl<'a> FmtContext<'a> {
    pub fn pattern(&mut self) -> Result<Pat> {
        match self.kind() {
            "identifier" | "constructor_identifier" => {
                let name = self.name()?;
                Ok(Pat::new(Span::default(), PatKind::Name(name)))
            }
            kind => todo!("{}", kind),
        }
    }
}
