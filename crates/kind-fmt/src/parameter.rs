use kind_syntax::concrete::{ParameterBinding, SignatureParameter, TypeBinding};
use kind_syntax::lexemes::{AngleBracket, Colon, Paren, Token};

use crate::{FmtContext, Result};

impl<'a> FmtContext<'a> {
    pub fn parameter(&mut self) -> Result<SignatureParameter> {
        match self.kind() {
            "parameter" => {
                let modifier = self.find("modifier", |node| self.text_of(node))?;
                let signature = match modifier {
                    Some(x) if x == "+" => SignatureParameter::Include,
                    Some(x) if x == "-" => SignatureParameter::Exclude,
                    Some(x) if x == "-+" => SignatureParameter::Both,
                    Some(x) if x == "+-" => SignatureParameter::Both,
                    _ => SignatureParameter::Include,
                };

                let parameter = self.cursor(self.first().unwrap());

                let name = parameter
                    .find("name", |node| parameter.cursor(node).name())?
                    .unwrap();
                let binding_type = parameter.clone().find("type", |node| {
                    let expr = parameter.cursor(node).expr()?;
                    Ok(Colon(Token::default(), Box::new(expr)))
                })?;

                match parameter.kind() {
                    "explicit_parameter" => Ok(signature(ParameterBinding::Explicit(Paren(
                        Token::default(),
                        TypeBinding { name, binding_type },
                        Token::default(),
                    )))),
                    "implicit_parameter" => {
                        Ok(signature(ParameterBinding::Implicit(AngleBracket(
                            Token::default(),
                            TypeBinding { name, binding_type },
                            Token::default(),
                        ))))
                    }
                    kind => todo!("{}", kind),
                }
            }
            kind => todo!("{}", kind),
        }
    }
}
