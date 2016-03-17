
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

pub type IntType = i64;

pub enum Token {
  LITERAL(Literal),
  OP(Operator),
  LPAR,
  RPAR,
  WS(char),
  EOF,
}

pub enum Literal {
  INT(IntType),
}

pub enum Operator {
  Pow,
  Mult,
  Div,
  Add,
  Sub,
  Not,
}



