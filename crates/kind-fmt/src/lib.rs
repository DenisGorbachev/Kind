use std::path::PathBuf;

use thin_vec::{thin_vec, ThinVec};
use tree_sitter::{Parser, Tree, TreeCursor};

use kind_syntax::concrete::{Block, ConstructorExpr, Expr, ExprKind, LocalExpr, Module, Pat, PatKind, Rule, Signature, Stmt, TopLevel, TopLevelKind};
use kind_syntax::lexemes::{Brace, Colon, Equal, Ident, Item, Name, Span, Token, Tokenized};

#[derive(Debug)]
pub enum FmtError {
    IoError(std::io::Error),
    TreeSitterLanguageError(tree_sitter::LanguageError),
    TreeSitterQueryError(tree_sitter::QueryError),
    UnknownParseError,
}

pub type Result<T> = std::result::Result<T, FmtError>;

pub fn run_fmt(string: String) -> Result<Module> {
    let mut parser = Parser::new();
    parser
        .set_language(tree_sitter_kind::language())
        .map_err(FmtError::TreeSitterLanguageError)?;
    let tree = parser
        .parse(string.as_bytes(), None)
        .ok_or(FmtError::UnknownParseError)?;

    println!("{:?}", tree.root_node().to_sexp());

    let mut cursor = tree.root_node().walk();

    let declarations = tree.root_node().children(&mut cursor)
        .map(|node| {
            specialize_top_level(&string, &tree, &mut node.walk())
        })
        .collect::<Result<ThinVec<_>>>()?;

    Ok(Module {
        shebang: None,
        items: declarations,
        eof: Token::default(),
    })
}

fn specialize_top_level(file: &String, tree: &Tree, cursor: &mut TreeCursor) -> Result<TopLevel> {
    let node = cursor.node();
    match node.kind() {
        "val_declaration" => {
            let name = node.child_by_field_name("name").unwrap();
            let return_type = node.child_by_field_name("return_type");
            let value = node.child_by_field_name("value");

            Ok(TopLevel {
                data: Item::new(
                    Span::default(),
                    TopLevelKind::Signature(Signature {
                        name: specialize_name(file, tree, &mut name.walk())?,
                        arguments: thin_vec![],
                        return_type: return_type.map_or(Ok(None), |node| {
                            let expr = specialize_expr(file, tree, &mut node.walk())?;
                            Ok(Some(Colon(Token::default(), expr)))
                        })?,
                        value: value.map_or(Ok(None), |node| {
                            let block = specialize_statements(file, tree, &mut node.walk())?;
                            Ok(Some(block))
                        })?,
                    }),
                ),
                attributes: thin_vec![],
            })
        }
        _ => todo!(),
    }
}

fn specialize_pattern(file: &String, tree: &Tree, cursor: &mut TreeCursor) -> Result<Pat> {
    let node = cursor.node();
    match node.kind() {
        "identifier" => {
            specialize_name(file, tree, cursor)
                .map(|name| Pat::new(
                    Span::default(),
                    PatKind::Name(name),
                ))
        }
        "constructor_identifier" => {
            specialize_name(file, tree, cursor)
                .map(|name| Pat::new(
                    Span::default(),
                    PatKind::Name(name),
                ))
        }
        kind => todo!("{}", kind),
    }
}

fn specialize_expr(file: &String, tree: &Tree, cursor: &mut TreeCursor) -> Result<Expr> {
    let node = cursor.node();
    match node.kind() {
        "call" => {
            let callee = node
                .child_by_field_name("callee")
                .unwrap();

            specialize_primary(file, tree, &mut callee.walk())
        },
        kind => todo!("{}", kind),
    }
}

fn specialize_name(file: &String, _tree: &Tree, cursor: &mut TreeCursor) -> Result<Name> {
    let node = cursor.node();
    match node.kind() {
        "identifier" => {
            let name = node.utf8_text(file.as_bytes()).unwrap().to_string();

            Ok(Name::Ident(Ident(Item::new(
                Span::default(),
                Tokenized(Token::default(), name),
            ))))
        }
        kind => todo!("{}", kind),
    }
}

fn specialize_statements(file: &String, tree: &Tree, cursor: &mut TreeCursor) -> Result<Block> {
    let node = cursor.node();
    match node.kind() {
        "statements" => {
            let statements = node.named_children(cursor)
                .map(|node| {
                    specialize_expr(file, tree, &mut node.walk())
                })
                .collect::<Result<ThinVec<_>>>()?;

            Ok(Brace(
                Token::default(),
                statements.iter().map(|expr| Stmt {
                    value: expr.clone(),
                    semi: None,
                }).collect(),
                Token::default(),
            ))
        }
        kind => todo!("{}", kind),
    }
}

fn specialize_primary(file: &String, tree: &Tree, cursor: &mut TreeCursor) -> Result<Expr> {
    let node = cursor.node();
    match node.kind() {
        "constructor_identifier" => {
            specialize_name(file, tree, cursor)
                .map(|name| Expr::new(
                    Span::default(),
                    ExprKind::Constructor(Box::new(ConstructorExpr {
                        name,
                    })),
                ))
        }
        "identifier" => {
            specialize_name(file, tree, cursor)
                .map(|name| Expr::new(
                    Span::default(),
                    ExprKind::Local(Box::new(LocalExpr {
                        name,
                    })),
                ))
        }
        _ => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let expr = run_fmt("bao:pao{a}".into()).unwrap();
        println!("{:#?}", expr);
    }
}
