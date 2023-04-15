use thin_vec::ThinVec;

use kind_syntax::concrete::{Block, Stmt};
use kind_syntax::lexemes::{Brace, Token};

use crate::{FmtContext, Result};

impl<'a> FmtContext<'a> {
    pub fn statements(&mut self) -> Result<Block> {
        match self.kind() {
            "statements" => {
                let statements = self
                    .named_children()
                    .map(|node| self.cursor(node).expr())
                    .collect::<Result<ThinVec<_>>>()?;

                Ok(Brace(
                    Token::default(),
                    statements
                        .iter()
                        .map(|expr| Stmt {
                            value: expr.clone(),
                            semi: None,
                        })
                        .collect(),
                    Token::default(),
                ))
            }
            kind => todo!("{}", kind),
        }
    }
}
