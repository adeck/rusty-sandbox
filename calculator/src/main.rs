

//  syntax:
//    LITERAL   [0-9]+
//    BINOP     **|*|/|+|-
//    UNOP      -!
//    LPAR      (
//    RPAR      )
//    EOF       $ // not an actual dollar sign; simply end-of-stream
//

//  grammar:
//    Program         := ExpressionList EOF
//    ExpressionList  := Expression ExpressionList
//                      | None
//    Expression      := UNOP Expression
//                      | LPAR Expression BINOP Expression RPAR
//

// ---- lexer classes ----
enum Token<T> {
  LITERAL(T),
  BINOP(BinaryOp),
  UNOP(UnaryOp),
  LPAR,
  RPAR,
  EOF,
}

enum BinaryOp {
  Pow,
  Mult,
  Div,
  Add,
  Sub,
}

enum UnaryOp {
  Neg,
  Not,
}

// ---- parser classes ----

enum ExpressionList<'a, T : 'a> {
  Some(Expression<'a, T>, &'a ExpressionList<'a, T>),
  None,
}

enum Expression<'a, T : 'a> {
  Literal(T),
  UnExpr(&'a Expression<'a, T>, UnaryOp),
  BinExpr(&'a Expression<'a, T>, BinaryOp, &'a Expression<'a, T>)
}

fn main() {
  println!("Hello, world");
}


