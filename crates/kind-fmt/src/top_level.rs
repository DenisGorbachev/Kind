use thin_vec::thin_vec;
use kind_syntax::concrete::{Signature, TopLevel, TopLevelKind};
use kind_syntax::lexemes::{Colon, Item, Span, Token};
use crate::{FmtContext, Result};

impl<'a> FmtContext<'a> {
    pub fn top_level(&mut self) -> Result<TopLevel> {
        match self.kind() {
            "val_declaration" => {
                let name = self.find("name", |node| self.cursor(node).name())?.unwrap();
                let value = self.find("value", |node| self.cursor(node).statements())?;
                let parameters = self
                    .properties("parameters")
                    .map(|node| self.cursor(node).parameter())
                    .collect::<Result<_>>()?;
                let return_type = self.find("return_type", |node| {
                    let value = self.cursor(node).expr()?;
                    Ok(Colon(Token::default(), value))
                })?;

                Ok(TopLevel {
                    data: Item::new(
                        Span::default(),
                        TopLevelKind::Signature(Signature {
                            name,
                            parameters,
                            return_type,
                            value,
                        }),
                    ),
                    attributes: thin_vec![],
                })
            }
            kind => todo!("{}", kind),
        }
    }
}
