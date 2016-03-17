
//  grammar:
//    Program         := ExpressionList EOF
//    ExpressionList  := Expression ExpressionList
//                      | None
//    Expression      := OP Expression
//                      | LPAR Expression OP Expression RPAR
//                      | LITERAL
//

// ---- production classes ----

enum ExpressionList<'a> {
  Some(Expression<'a>, &'a ExpressionList<'a>),
  None,
}

enum Expression<'a> {
  Literal(Literal),
  UnExpr(&'a Expression<'a>, Operator),
  BinExpr(&'a Expression<'a>, Operator, &'a Expression<'a>)
}


