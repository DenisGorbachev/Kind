//! Describes the concrete tree with all of the sugars. It's useful
//! to pretty printing and resugarization from the type checker.
//! It stores some tokens inside the tree in order to make it easier
//! to reconstruct the entire program.

use num_bigint::BigUint;
use thin_vec::ThinVec;

use crate::lexemes;
use crate::lexemes::{AngleBracket, Brace, Bracket, Colon, Either, Equal, Ident, Item, Name, Paren, QualifiedIdent, Token, Tokenized};

#[derive(Debug, Clone, PartialEq)]
pub enum AttributeStyleKind {
    String(Tokenized<String>),
    Number(Tokenized<u64>),
    Identifier(Ident),
    List(Bracket<ThinVec<AttributeStyle>>),
}

/// The "argument" part of the attribute. It can be used both in
/// the value after an equal or in the arguments e.g
///
/// ```kind
/// #name = "Vaundy"
/// #derive[match]
/// ```
///
pub type AttributeStyle = Item<AttributeStyleKind>;

#[derive(Debug, Clone, PartialEq)]
pub struct AttributeKind {
    pub r#hash: lexemes::Hash,
    pub name: Ident,
    pub value: Option<Equal<AttributeStyle>>,
    pub arguments: Option<Bracket<ThinVec<AttributeStyle>>>,
}

/// An attribute is a special compiler flag.
pub type Attribute = Item<AttributeKind>;

/// A type binding is a type annotation for a variable.
#[derive(Debug, Clone, PartialEq)]
pub struct TypeBinding {
    pub name: Name,
    pub binding_type: Option<Colon<Box<Expr>>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParameterBinding {
    Implicit(AngleBracket<TypeBinding>),
    Explicit(Paren<TypeBinding>),
}

/// An argument of a type signature.
#[derive(Debug, Clone, PartialEq)]
pub enum SignatureParameter {
    Exclude(ParameterBinding),
    Include(ParameterBinding),
    Both(ParameterBinding),
}

/// A local expression is a reference atom to a local declaration.
/// * Always starts with a lower case letter.
#[derive(Debug, Clone, PartialEq)]
pub struct LocalExpr {
    pub name: Name,
}

/// A constructor expression is a reference atom to a top level declaration.
/// * Always starts with an upper case letter.
#[derive(Debug, Clone, PartialEq)]
pub struct ConstructorExpr {
    pub name: Name,
}

/// A param is a variable or a type binding.
/// i.e.
/// ```kind
/// (x : Int)
/// // or
/// x
#[derive(Debug, Clone, PartialEq)]
pub enum PiParameter {
    Named(Paren<TypeBinding>),
    Expr(Box<Expr>),
}

/// A all node is a dependent function type.
#[derive(Debug, Clone, PartialEq)]
pub struct PiExpr {
    pub r#tilde: lexemes::Tilde,
    pub param: PiParameter,
    pub r#arrow: lexemes::RightArrow,
    pub body: Box<Expr>,
}

/// A sigma node is a dependent pair type. It express
/// the dependency of the type of the second element
/// of the pair on the first one.
#[derive(Debug, Clone, PartialEq)]
pub struct SigmaExpr {
    pub param: Bracket<TypeBinding>,
    pub r#arrow: lexemes::RightArrow,
    pub body: Box<Expr>,
}

/// A lambda expression (an anonymous function).
#[derive(Debug, Clone, PartialEq)]
pub struct LambdaExpr {
    pub r#tilde: Option<lexemes::Tilde>,
    pub param: PiParameter,
    pub r#arrow: lexemes::FatArrow,
    pub body: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rename(pub Ident, pub Equal<Box<Expr>>);

#[derive(Debug, Clone, PartialEq)]
pub enum NamedBinding {
    Named(Paren<Rename>),
    Expr(Box<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Binding {
    pub r#tilde: Option<lexemes::Tilde>,
    pub value: NamedBinding,
}

/// Application of a function to a sequence of arguments.
#[derive(Debug, Clone, PartialEq)]
pub struct AppExpr {
    pub fun: Box<Expr>,
    pub arg: ThinVec<Binding>,
}

/// A type annotation.
#[derive(Debug, Clone, PartialEq)]
pub struct AnnExpr {
    pub value: Box<Expr>,
    pub r#colon: lexemes::ColonColon,
    pub typ: Box<Expr>,
}

/// A literal is a constant value that can be used in the program.
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    U60(Tokenized<u64>),
    F60(Tokenized<f64>),
    U120(Tokenized<u128>),
    Nat(Tokenized<BigUint>),
    String(Tokenized<String>),
    Char(Tokenized<char>),
}

// TODO: Rename
#[derive(Debug, Clone, PartialEq)]
pub enum TypeExpr {
    Help(Tokenized<String>),
    Type(lexemes::Type),
    TypeU60(Token),
    TypeU120(Token),
    TypeF60(Token),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Neq,
    Lt,
    Gt,
    Leq,
    Geq,
    And,
    Or,
    Xor,
    Not,
    Shl,
    Shr,
}

/// A binary operation.
#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub op: Tokenized<Operation>,
    pub right: Box<Expr>,
}

/// An ask statement is a monadic binding inside the `do` notation
/// with a name.
#[derive(Debug, Clone, PartialEq)]
pub struct AskExpr {
    pub r#ask: lexemes::Ask,
    pub name: Ident,
    pub value: Equal<Box<Expr>>,
}

/// A let binding inside the `do` notation.
#[derive(Debug, Clone, PartialEq)]
pub struct LetExpr {
    pub r#let: lexemes::Let,
    pub name: Ident,
    pub value: Equal<Box<Expr>>,
    pub r#semi: Option<lexemes::Semi>,
    pub next: Box<Expr>,
}

/// The "pure" function of the `A` monad.
#[derive(Debug, Clone, PartialEq)]
pub struct ReturnExpr {
    pub r#return: lexemes::Return,
    pub value: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Stmt {
    pub value: Expr,
    pub r#semi: Option<lexemes::Semi>,
}

pub type Block = Brace<ThinVec<Stmt>>;

/// A DoNode is similar to the haskell do notation.
/// i.e.
///
/// ```kind
/// do A {
///     ask a = 2;
///     ask b = 3;
///     return a + b;
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct DoNode {
    pub r#do: lexemes::Do,
    pub typ: Option<Ident>,
    pub value: Block,
}

/// Conditional expression.
#[derive(Debug, Clone, PartialEq)]
pub struct IfExpr {
    pub cond: Tokenized<Box<Expr>>,
    pub then: Brace<Box<Expr>>,
    pub otherwise: Tokenized<Brace<Box<Expr>>>,
}

/// A Pair node represents a dependent pair. i.e.
///
/// ```kind
/// $ a b
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct PairNode<T> {
    pub r#sign: lexemes::Sign,
    pub left: Box<T>,
    pub right: Box<T>,
}

/// A substitution expression is a substitution of a value inside the context.
/// i.e.
///
/// ```kind
/// specialize a into #0 in a
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct SubstExpr {
    pub r#specialize: lexemes::Specialize,
    pub name: Ident,
    pub r#into: Token,
    pub r#hash: lexemes::Hash,
    pub num: u64,
    pub r#in: Token,
    pub value: Box<Expr>,
}

/// A List expression represents a list of values.
#[derive(Debug, Clone, PartialEq)]
pub struct ListNode<T> {
    pub bracket: Bracket<ThinVec<T>>,
}

/// A Case is a single case in a match node.
#[derive(Debug, Clone, PartialEq)]
pub struct CaseNode {
    pub name: Ident,
    pub r#arrow: lexemes::FatArrow,
    pub value: Box<Expr>,
}

/// A match expression is a case analysis on a value (dependent eliminator)
/// i.e.
///
/// ```kind
/// match List a {
///     nil  => 0
///     cons => a.head
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct MatchExpr {
    pub r#match: lexemes::Match,
    pub typ: Option<Ident>,
    pub with: Option<(lexemes::With, ThinVec<PiParameter>)>,
    pub scrutinee: Box<Expr>,
    pub cases: Brace<ThinVec<CaseNode>>,
    pub motive: Option<Colon<Box<Expr>>>,
}

/// An OpenNode introduces each field of a constructor as variables
/// into the context so we can program like the dot notation of object
/// oriented programming languages. i.e.
///
/// ```kind
/// open List a
/// a.head
/// ```
///
#[derive(Debug, Clone, PartialEq)]
pub struct OpenExpr {
    pub r#open: lexemes::Open,
    pub typ: Ident,

    /// The concrete syntax tree allows some more flexibility in order
    /// to improve error messages. So, the name should be an identifier
    /// but the parser will allow any expression.
    pub name: Box<Expr>,
    pub motive: Option<Colon<Box<Expr>>>,
    pub body: Brace<ThinVec<Stmt>>,
}

/// A node that express the operation after accessing fields
/// of a record.
#[derive(Debug, Clone, PartialEq)]
pub enum AccessOperation {
    Set(Token, Box<Expr>),
    Mut(Token, Box<Expr>),
    Get,
}

/// A node for accessing and modifying fields of a record.
#[derive(Debug, Clone, PartialEq)]
pub struct AccessExpr {
    pub typ: Box<Expr>,
    pub expr: Box<Expr>,
    pub fields: ThinVec<(lexemes::Dot, Ident)>,
    pub operation: AccessOperation,
}

/// An expression is a piece of code that can be evaluated.
#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind {
    Local(Box<LocalExpr>),
    Pi(Box<PiExpr>),
    Sigma(Box<SigmaExpr>),
    Lambda(Box<LambdaExpr>),
    App(Box<AppExpr>),
    Let(Box<LetExpr>),
    Ann(Box<AnnExpr>),
    Binary(Box<BinaryExpr>),
    Do(Box<DoNode>),
    If(Box<IfExpr>),
    Literal(Box<Literal>),
    Constructor(Box<ConstructorExpr>),
    Pair(Box<PairNode<Expr>>),
    List(Box<ListNode<Expr>>),
    Subst(Box<SubstExpr>),
    Match(Box<MatchExpr>),
    Open(Box<OpenExpr>),
    Access(Box<AccessExpr>),
    Type(Box<TypeExpr>),
    Paren(Box<Paren<Expr>>),
    Error,
}

pub type Expr = Item<ExprKind>;

/// A constructor node is the name of a global function.
#[derive(Debug, Clone, PartialEq)]
pub struct ConstructorPat {
    pub name: QualifiedIdent,
    pub args: ThinVec<SignatureParameter>,
}

/// A pattern is part of a rule. It is a structure that matches an expression.
#[derive(Debug, Clone, PartialEq)]
pub enum PatKind {
    Name(Name),
    Pair(PairNode<Pat>),
    Constructor(ConstructorPat),
    List(ListNode<Pat>),
    Literal(Literal),
}

pub type Pat = Item<PatKind>;

/// A type signature is a top-level structure that says what is the type
/// of a function i.e.
///
/// ```kind
/// Add (n: Nat) (m: Nat) : Nat
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Signature {
    pub name: Name,
    pub parameters: ThinVec<SignatureParameter>,
    pub return_type: Option<Colon<Expr>>,
    pub value: Option<Block>,
}

/// A rule is a top-level structure that have pattern match rules. It does
/// not include the neither type signature nor other rules i.e.
///
/// ```kind
/// Add Nat.zero m = m
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Rule {
    pub name: Ident,
    pub patterns: ThinVec<Pat>,
    pub value: Equal<Expr>,
}

/// A function is a top-level structure that dont have pattern match rules
/// i.e.
///
/// ```kind
/// Add (n: Nat) (m: Nat) : Nat {
///    n + m
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: Ident,
    pub arguments: ThinVec<SignatureParameter>,
    pub return_typ: Option<Colon<Expr>>,
    pub value: Brace<Expr>,
}

/// Commands are top level structures that will run at compile time.
/// It's useful for evaluating expressions and making widgets without
/// compromising the structure of the program too much. i.e.
///
/// ```kind
/// @eval (+ 1 1)
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Command {
    pub at: Token,
    pub name: Ident,
    pub arguments: ThinVec<Expr>,
}

/// A constructor is a structure that defines a data constructor of a type
/// family. i.e.
///
/// ```kind
///    some (value: a) : Maybe a
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Constructor {
    pub name: Ident,
    pub arguments: ThinVec<SignatureParameter>,
    pub typ: Option<Colon<ThinVec<Expr>>>,
}

/// A type definition is a top-level structure that defines a type family
/// with multiple constructors that named fields, indices and parameters.
#[derive(Debug, Clone, PartialEq)]
pub struct TypeDef {
    pub name: QualifiedIdent,
    pub constructors: ThinVec<Constructor>,
    pub params: ThinVec<SignatureParameter>,
    pub indices: ThinVec<SignatureParameter>,
}

/// A record definition is a top-level structure that defines a type with
/// a single constructor that has named fields with named fields.
#[derive(Debug, Clone, PartialEq)]
pub struct RecordDef {
    pub name: QualifiedIdent,
    pub fields: ThinVec<TypeBinding>,
    pub params: ThinVec<SignatureParameter>,
    pub indices: ThinVec<SignatureParameter>,
}

/// A top-level item is a item that is on the outermost level of a
/// program. It includes functions, commands, signatures and rules.
#[derive(Debug, Clone, PartialEq)]
pub enum TopLevelKind {
    Function(Function),
    Commmand(Command),
    Signature(Signature),
    Record(RecordDef),
    Type(TypeDef),
    Rule(Rule),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Attributed<T> {
    pub attributes: ThinVec<Attribute>,
    pub data: T,
}

/// A top level structure with attributes.
pub type TopLevel = Attributed<Item<TopLevelKind>>;

/// A collection of top-level items. This is the root of the CST and
/// is the result of parsing a module.
#[derive(Debug, Clone, PartialEq)]
pub struct Module {
    pub shebang: Option<String>,
    pub items: ThinVec<TopLevel>,
    pub eof: Token,
}
