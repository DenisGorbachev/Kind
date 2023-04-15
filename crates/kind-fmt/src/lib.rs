use thin_vec::ThinVec;
use tree_sitter::{Node, Parser, Tree, TreeCursor};

use kind_syntax::concrete::Module;
use kind_syntax::lexemes::Token;

mod expr;
mod name;
mod parameter;
mod pattern;
mod primary;
mod statements;
mod top_level;

#[derive(Debug)]
pub enum FmtError {
    IoError(std::io::Error),
    Utf8Error(std::str::Utf8Error),
    TreeSitterLanguageError(tree_sitter::LanguageError),
    TreeSitterQueryError(tree_sitter::QueryError),
    UnknownParseError,
}

#[derive(Clone, Copy)]
pub struct FmtContext<'a> {
    pub file: &'a str,
    pub tree: &'a Tree,
    pub cursor: *mut TreeCursor<'a>,
}

pub type Result<T> = std::result::Result<T, FmtError>;

pub fn run_fmt(string: String) -> Result<Module> {
    let mut parser = Parser::new();
    parser
        .set_language(tree_sitter_kind::language())
        .map_err(FmtError::TreeSitterLanguageError)?;
    let mut tree = parser
        .parse(string.as_bytes(), None)
        .ok_or(FmtError::UnknownParseError)?;

    println!("{:?}", tree.root_node().to_sexp());

    let mut cursor = tree.root_node().walk();

    let context = FmtContext {
        file: &string,
        tree: &tree,
        cursor: &mut cursor,
    };

    let declarations = tree
        .root_node()
        .children(&mut cursor)
        .map(|node| context.cursor(node).top_level())
        .collect::<Result<ThinVec<_>>>()?;

    Ok(Module {
        shebang: None,
        items: declarations,
        eof: Token::default(),
    })
}

impl<'a> FmtContext<'a> {
    pub fn node(&self) -> Node<'a> {
        unsafe { (*self.cursor).node() }
    }

    pub fn get_current_cursor(&self) -> TreeCursor<'a> {
        unsafe { (*self.cursor).clone() }
    }

    pub fn kind(&self) -> &'static str {
        self.node().kind()
    }

    pub fn first(&self) -> Option<Node<'a>> {
        self.node().child(0)
    }

    pub fn at(&self, at: usize) -> Option<Node<'a>> {
        self.node().child(at)
    }

    pub fn named_at(&self, at: usize) -> Option<Node<'a>> {
        self.node().named_child(at)
    }

    pub fn property(&self, name: &str) -> Option<Node<'a>> {
        self.node().child_by_field_name(name)
    }

    pub fn properties(&self, name: &'static str) -> impl Iterator<Item = Node<'a>> {
        unsafe {
            self.node().children_by_field_name(name, self.cursor.as_mut().unwrap())
        }
    }

    pub fn find<F, T>(&self, name: &str, mut f: F) -> Result<Option<T>>
    where
        F: FnMut(Node) -> Result<T>,
    {
        self
            .node()
            .child_by_field_name(name)
            .map_or(Ok(None), |node| {
                let value = f(node)?;
                Ok(Some(value))
            })
    }

    pub fn named_children(&self) -> impl Iterator<Item = Node<'a>> + '_ {
        unsafe {
            self.node().named_children(self.cursor.as_mut().unwrap())
        }
    }

    pub fn children(&self) -> impl Iterator<Item = Node<'a>> + '_ {
        let mut cursor = self.get_current_cursor();

        cursor.reset(self.node());
        cursor.goto_first_child();
        (0..self.node().child_count()).map(move |_| {
            let result = cursor.node();
            cursor.goto_next_sibling();
            result
        })
    }

    pub fn text(&self) -> Result<&'a str> {
        self.node()
            .utf8_text(self.file.as_bytes())
            .map_err(FmtError::Utf8Error)
    }

    pub fn text_of(&self, node: Node<'_>) -> Result<&'a str> {
        node.utf8_text(self.file.as_bytes())
            .map_err(FmtError::Utf8Error)
    }

    pub fn cursor<'b>(&'b self, node: Node<'b>) -> FmtContext<'b> {
        // fix this leak
        let cursor = Box::leak(Box::new(node.walk()));
        FmtContext {
            file: self.file,
            tree: self.tree,
            cursor,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let expr = run_fmt("Ata (name: U60) : Type { Something }".into()).unwrap();
        println!("{:#?}", expr);
    }
}
