use crate::concrete::{ConstructorExpr, Expr, ExprKind, LocalExpr};
use crate::lexemes::{Name, Span};

impl Expr {
    pub fn constructor(span: Span, name: Name) -> Self {
        Expr::new(
            span,
            ExprKind::Constructor(Box::new(ConstructorExpr { name })),
        )
    }

    pub fn local(span: Span, name: Name) -> Self {
        Expr::new(span, ExprKind::Local(Box::new(LocalExpr { name })))
    }
}
//
// macro_rules! impl_expr_kind {
//     ( $( $name:ident( $( $arg:ident: $ty:ty ),* ) ),* ) => {
//         impl Expr {
//             $(pub fn $name($($arg: $ty),* span: Span) -> Self {
//                 Expr::new(
//                     span,
//                     ExprKind::$name(Box::new($name Expr { $($arg),* })))
//             })*
//         }
//     };
// }
//
// impl_expr_kind!(
//         Local( name: Name, ),
//         Constructor( name: Name, ),
// );
