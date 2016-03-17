
//  syntax:
//    INT       [0-9]+
//    BINOP     **|*|/|+|-|!
//    UNOP      -!
//    LPAR      (
//    RPAR      )
//    WS        [ \t\n]
//    EOF       $ // not an actual dollar sign; simply end-of-stream
//

// ---- token classes ----

type IntType = i64;

enum Token {
  LITERAL(Literal),
  OP(Operator),
  LPAR,
  RPAR,
  EOF,
}

enum Literal {
  INT(IntType),
}

enum Operator {
  Pow,
  Mult,
  Div,
  Add,
  Sub,
  Not,
}



