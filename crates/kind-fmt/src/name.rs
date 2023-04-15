use kind_syntax::lexemes::{Ident, Item, Name, QualifiedIdent, Span, Token, Tokenized};
use crate::{Result, FmtContext};

impl<'a> FmtContext<'a> {
    pub fn name(&mut self) -> Result<Name> {
        match self.kind() {
            "identifier" => {
                let name = self.text()?.to_string();

                Ok(Name::Ident(Ident(Item::new(
                    Span::default(),
                    Tokenized(Token::default(), name),
                ))))
            }
            "constructor_identifier" => {
                let name = self.text()?.to_string();

                Ok(Name::QualifiedIdent(QualifiedIdent(Item::new(
                    Span::default(),
                    Tokenized(Token::default(), name),
                ))))
            }
            kind => todo!("{}", kind),
        }
    }
}