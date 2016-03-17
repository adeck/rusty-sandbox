
// TODO -- figure out which of these references actually need to be
//          mutable, and which don't.
//          I feel like the BufReader should only need a mutable reference
//            within the context of a call to get_token, otherwise it
//            should be possible to externally read from BufReader (if,
//            for example, you have a contextualized grammar s.t. this
//            thing's lexing is only applicable some of the time, and
//            the rest of the time the stream should be interpreted
//            by a different lexer).

use lexer::tokens::{Token, Literal, Operator};
use std::io::BufReader;
use std::result;
use std::option;

pub struct Scanner<'a, R : 'a> {
  src : &'a mut BufReader<R>,
}

impl <'a, R : 'a> Scanner<'a, R> {

  pub fn new(r : &'a mut BufReader<R>) -> Scanner<'a, R> {
    Scanner::<'a, R>{src: r}
  }
  
  pub fn get_token(& mut self) -> Result<Token, &'static str> {
    match self.peek() {
      Ok(x) => match x {
        None => Result::Ok(Token::EOF),
        Some(c) => match c {
          '0' ... '9' => self.get_literal(),
          '(' => Result::Ok(Token::LPAR),
          ')' => Result::Ok(Token::RPAR),
          '!' | '*' | '/' | '+' | '-' => self.get_operator(),
          ' ' | '\t' | '\n' => Result::Ok(Token::WS(c)),
          _ => Result::Err("unrecognized character"),
        },
      },
      Err(x) => Result::Err(x),
    }
  }

  fn peek(& mut self) -> Result<Option<char>, &'static str> {
    // TODO
    Result::Err("not implemented")
  }

  fn pop(& mut self) -> Result<Option<char>, &'static str> {
    // TODO
    Result::Err("not implemented")
  }

  fn get_literal(& mut self) -> Result<Token , &'static str> {
    // TODO
    Result::Err("not implemented")
  }

  fn get_operator(& mut self) -> Result<Token , &'static str> {
    match self.pop() {
      Ok(Some('*')) => 
          match self.peek() {
            Ok(Some('*')) => match self.pop() {
                Ok(_) => Result::Ok(Token::OP(Operator::Mult)),
                Err(x) => Err(x),
              }, 
            Ok(_) => Result::Ok(Token::OP(Operator::Mult)),
            Err(x) => Err(x),
          },
      Ok(Some(c)) => match c {
            '!' => Result::Ok(Token::OP(Operator::Not)),
            '/' => Result::Ok(Token::OP(Operator::Div)),
            '+' => Result::Ok(Token::OP(Operator::Add)),
            '-' => Result::Ok(Token::OP(Operator::Sub)),
            _ => Result::Err("unexpected stream read error"),
          },
      Ok(None) => Result::Err("unexpected stream read error"),
      Err(x) => Result::Err(x),
    }
  }
}


