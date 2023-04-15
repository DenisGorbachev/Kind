use std::fmt::Debug;
use tree_sitter::Point;

#[derive(Default, Clone, PartialEq)]
pub struct Token {
    pub span: Span,
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Token({:?})", self.span)
    }
}

#[derive(Default, Clone, PartialEq)]
pub struct Span(pub Point, pub Point);

impl Debug for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Span({:?}:{:?}..{:?}:{:?})", self.0.row, self.0.column, self.1.row, self.1.column)
    }
}

impl Span {
    pub fn new(start: Point, end: Point) -> Span {
        Span(start, end)
    }

    pub fn mix(&self, _other: &Span) -> Span {
        self.clone()
    }
}

pub type Hash = Token;
pub type Minus = Token;
pub type Plus = Token;
pub type Semi = Token;
pub type RightArrow = Token;
pub type Tilde = Token;
pub type FatArrow = Token;
pub type ColonColon = Token;
pub type Let = Token;
pub type Type = Token;
pub type Help = Token;
pub type With = Token;
pub type Ask = Token;
pub type Return = Token;
pub type Sign = Token;
pub type Specialize = Token;
pub type In = Token;
pub type Match = Token;
pub type Open = Token;
pub type Do = Token;
pub type Dot = Token;

#[derive(Clone, PartialEq)]
pub enum Either<A, B> {
    Left(A),
    Right(B),
}

impl<A: Debug, B: Debug> Debug for Either<A, B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Either::Left(a) => write!(f, "Left({:?})", a),
            Either::Right(b) => write!(f, "Right({:?})", b),
        }
    }
}

// Compounds
#[derive(Clone, PartialEq)]
pub struct Paren<T>(pub Token, pub T, pub Token);

impl<A: Debug> Debug for Paren<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Paren({:?}, {:?})", self.0, self.1)
    }
}

#[derive(Clone, PartialEq)]
pub struct Bracket<T>(pub Token, pub T, pub Token);

impl<A: Debug> Debug for Bracket<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Bracket({:?}, {:?})", self.0, self.1)
    }
}

#[derive(Clone, PartialEq)]
pub struct Brace<T>(pub Token, pub T, pub Token);

impl<A: Debug> Debug for Brace<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Brace({:?}, {:?})", self.0, self.1)
    }
}

#[derive(Clone, PartialEq)]
pub struct AngleBracket<T>(pub Token, pub T, pub Token);

impl<A: Debug> Debug for AngleBracket<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AngleBracket({:?}, {:?})", self.0, self.1)
    }
}

#[derive(Clone, PartialEq)]
pub struct Equal<T>(pub Token, pub T);

impl<A: Debug> Debug for Equal<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Equal({:?}, {:?})", self.0, self.1)
    }
}

#[derive(Clone, PartialEq)]
pub struct Colon<T>(pub Token, pub T);

impl<A: Debug> Debug for Colon<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Colon({:?}, {:?})", self.0, self.1)
    }
}

#[derive(Clone, PartialEq)]
pub struct Tokenized<T>(pub Token, pub T);

impl<A: Debug> Debug for Tokenized<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tokenized({:?}, {:?})", self.0, self.1)
    }
}

// Concrete syntax tree

#[derive(Clone, PartialEq)]
pub struct Ident(pub Item<Tokenized<String>>);

#[derive(Clone, PartialEq)]
pub enum Name {
    Ident(Ident),
    QualifiedIdent(QualifiedIdent),
}

impl Debug for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Name::Ident(ident) => write!(f, "{ident:?}"),
            Name::QualifiedIdent(ident) => write!(f, "{ident:?}"),
        }
    }
}

impl Debug for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ident({:?})", self.0.data.1)
    }
}

#[derive(Clone, PartialEq)]
pub struct QualifiedIdent(pub Item<Tokenized<String>>);

impl Debug for QualifiedIdent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "QualifiedIdent({:?})", self.0.data.1)
    }
}

/// A localized data structure, it's useful to keep track of source code
/// location.
#[derive(Clone, PartialEq)]
pub struct Item<T> {
    pub data: T,
    pub span: Span,
}

impl<A: Debug> Debug for Item<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Item({:?}, {:?})", self.span, self.data)
    }
}

impl<T> Paren<T> {
    pub fn span(&self) -> Span {
        self.0.span.mix(&self.2.span)
    }
}

impl<T> Bracket<T> {
    pub fn span(&self) -> Span {
        self.0.span.mix(&self.2.span)
    }
}

impl<T> Brace<T> {
    pub fn span(&self) -> Span {
        self.0.span.mix(&self.2.span)
    }
}

impl<T> Tokenized<T> {
    pub fn span(&self) -> Span {
        self.0.span.clone()
    }
}

impl<T> From<Tokenized<T>> for Item<T> {
    fn from(val: Tokenized<T>) -> Self {
        Item::new(val.span(), val.1)
    }
}

impl<T> Item<T> {
    pub fn new(span: Span, data: T) -> Item<T> {
        Item { data, span }
    }

    pub fn map<U>(self, fun: fn(T) -> U) -> Item<U> {
        Item {
            data: fun(self.data),
            span: self.span,
        }
    }
}
