use crate::tags::Tag;

use crate::constants::exp_tags::{NUMBER, ADD_OP, MUL_OP, LPAR, RPAR, END_MARK, ERROR};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AddOp {
    Plus,
    Minus,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MulOp {
    Times,
    Divide,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
    Number(Tag, i32),
    AddOperator(Tag, AddOp),
    MulOperator(Tag, MulOp),
    LPar(Tag),
    RPar(Tag),
    EndMark(Tag),
    Error(Tag, &'static str),
}

pub trait Scanner {
    fn next_token(&mut self) -> Token;
}

pub struct Expression {
    current_position: usize,
    text: String,
}

impl Expression {
    pub fn new(text: &str) -> Self {
        Expression {
            current_position: 0,
            text: text.to_string(),
        }
    }
}

impl Scanner for Expression {
    fn next_token(&mut self) -> Token {
        let mut state = 0;
        let mut lexema = String::new();

        loop {
            match state {
                0 => {
                    match self.text.chars().nth(self.current_position) {
                        Some(c) if c.is_whitespace() => {
                            self.current_position += 1;
                            state = 0;
                            continue;
                        }
                        Some(c) if c.is_digit(10) => {
                            state = 1;
                            lexema.push(c);
                            self.current_position += 1;
                            continue;
                        }
                        Some('+') => {
                            let tag = Tag::new(crate::tags::Type::TERMINAL, ADD_OP);
                            self.current_position += 1;
                            return Token::AddOperator(tag, AddOp::Plus);
                        }
                        Some('-') => {
                            let tag = Tag::new(crate::tags::Type::TERMINAL, ADD_OP);
                            self.current_position += 1;
                            return Token::AddOperator(tag, AddOp::Minus);
                        }
                        Some('*') => {
                            let tag = Tag::new(crate::tags::Type::TERMINAL, MUL_OP);
                            self.current_position += 1;
                            return Token::MulOperator(tag, MulOp::Times);
                        }
                        Some('/') => {
                            let tag = Tag::new(crate::tags::Type::TERMINAL, MUL_OP);
                            self.current_position += 1;
                            return Token::MulOperator(tag, MulOp::Divide);
                        }
                        Some('(') => {
                            let tag = Tag::new(crate::tags::Type::TERMINAL, LPAR);
                            self.current_position += 1;
                            return Token::LPar(tag);
                        }
                        Some(')') => {
                            let tag = Tag::new(crate::tags::Type::TERMINAL, RPAR);
                            self.current_position += 1;
                            return Token::RPar(tag);
                        }
                        Some('#') => {
                            let tag = Tag::new(crate::tags::Type::TERMINAL, END_MARK);
                            self.current_position += 1;
                            return Token::EndMark(tag);
                        }
                        _ => {
                            let tag = Tag::new(crate::tags::Type::TERMINAL, ERROR);
                            return Token::Error(tag, "Caracter inválido");
                        }
                    }
                }
                1 => {
                    match self.text.chars().nth(self.current_position) {
                        Some(c) if c.is_digit(10) => {
                            lexema.push(c);
                            self.current_position += 1;
                            state = 1;
                            continue;
                        }
                        _ => {
                            self.current_position += 1;
                            state = 2;
                            continue;
                        }
                    }
                }
                2 => {
                    self.current_position -= 1; // RETRACT
                    let value = lexema.parse::<i32>().unwrap();
                    let tag = Tag::new(crate::tags::Type::TERMINAL, NUMBER);
                    return Token::Number(tag, value);
                }
                _ => {
                    let tag = Tag::new(crate::tags::Type::TERMINAL, ERROR);
                    return Token::Error(tag, "Caracter inválido");
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tags::Type;

    #[test]
    fn test_create_number_token() {
        let tag = Tag::new(Type::TERMINAL, NUMBER);
        let token = Token::Number(tag, 42);
        match token {
            Token::Number(_, value) => assert_eq!(value, 42),
            _ => panic!("Expected Number token"),
        }
    }

    #[test]
    fn test_create_add_operator_plus() {
        let tag = Tag::new(Type::TERMINAL, ADD_OP);
        let token = Token::AddOperator(tag, AddOp::Plus);
        match token {
            Token::AddOperator(_, op) => assert_eq!(op, AddOp::Plus),
            _ => panic!("Expected AddOperator token"),
        }
    }

    #[test]
    fn test_create_add_operator_minus() {
        let tag = Tag::new(Type::TERMINAL, ADD_OP);
        let token = Token::AddOperator(tag, AddOp::Minus);
        match token {
            Token::AddOperator(_, op) => assert_eq!(op, AddOp::Minus),
            _ => panic!("Expected AddOperator token"),
        }
    }

    #[test]
    fn test_create_mul_operator_times() {
        let tag = Tag::new(Type::TERMINAL, MUL_OP);
        let token = Token::MulOperator(tag, MulOp::Times);
        match token {
            Token::MulOperator(_, op) => assert_eq!(op, MulOp::Times),
            _ => panic!("Expected MulOperator token"),
        }
    }

    #[test]
    fn test_create_mul_operator_divide() {
        let tag = Tag::new(Type::TERMINAL, MUL_OP);
        let token = Token::MulOperator(tag, MulOp::Divide);
        match token {
            Token::MulOperator(_, op) => assert_eq!(op, MulOp::Divide),
            _ => panic!("Expected MulOperator token"),
        }
    }

    #[test]
    fn test_lpar_token() {
        let tag = Tag::new(Type::TERMINAL, LPAR);
        let token = Token::LPar(tag);
        assert_eq!(token, Token::LPar(tag));
    }

    #[test]
    fn test_rpar_token() {
        let tag = Tag::new(Type::TERMINAL, RPAR);
        let token = Token::RPar(tag);
        assert_eq!(token, Token::RPar(tag));
    }

    #[test]
    fn test_token_equality() {
        let tag1 = Tag::new(Type::TERMINAL, NUMBER);
        let tag2 = Tag::new(Type::TERMINAL, NUMBER);

        let token1 = Token::Number(tag1, 10);
        let token2 = Token::Number(tag2, 10);

        assert_eq!(token1, token2);
    }

    #[test]
    fn test_token_inequality() {
        let tag = Tag::new(Type::TERMINAL, NUMBER);
        let token1 = Token::Number(tag, 10);
        let token2 = Token::Number(tag, 20);

        assert_ne!(token1, token2);
    }

    #[test]
    fn test_add_op_equality() {
        assert_eq!(AddOp::Plus, AddOp::Plus);
        assert_ne!(AddOp::Plus, AddOp::Minus);
    }

    #[test]
    fn test_mul_op_equality() {
        assert_eq!(MulOp::Times, MulOp::Times);
        assert_ne!(MulOp::Times, MulOp::Divide);
    }

    #[test]
    fn test_token_debug_format() {
        let tag = Tag::new(Type::TERMINAL, NUMBER);
        let token = Token::Number(tag, 100);
        let debug_str = format!("{:?}", token);
        assert!(debug_str.contains("Number") || debug_str.contains("100"));
    }

    #[test]
    fn test_scanner_expression_tokens() {
        let expr_text = "43*(2 + 34) - 123 / (2 - 34) #";
        let mut expr = Expression::new(expr_text);

        // Token 1: Number(43)
        let token = expr.next_token();
        match token {
            Token::Number(_, 43) => (),
            _ => panic!("Expected Number(43), got {:?}", token),
        }

        // Token 2: MulOperator(*)
        let token = expr.next_token();
        match token {
            Token::MulOperator(_, MulOp::Times) => (),
            _ => panic!("Expected MulOperator(*), got {:?}", token),
        }

        // Token 3: LPar(
        let token = expr.next_token();
        match token {
            Token::LPar(_) => (),
            _ => panic!("Expected LPar, got {:?}", token),
        }

        // Token 4: Number(2)
        let token = expr.next_token();
        match token {
            Token::Number(_, 2) => (),
            _ => panic!("Expected Number(2), got {:?}", token),
        }

        // Token 5: AddOperator(+)
        let token = expr.next_token();
        match token {
            Token::AddOperator(_, AddOp::Plus) => (),
            _ => panic!("Expected AddOperator(+), got {:?}", token),
        }

        // Token 6: Number(34)
        let token = expr.next_token();
        match token {
            Token::Number(_, 34) => (),
            _ => panic!("Expected Number(34), got {:?}", token),
        }

        // Token 7: RPar)
        let token = expr.next_token();
        match token {
            Token::RPar(_) => (),
            _ => panic!("Expected RPar, got {:?}", token),
        }

        // Token 8: AddOperator(-)
        let token = expr.next_token();
        match token {
            Token::AddOperator(_, AddOp::Minus) => (),
            _ => panic!("Expected AddOperator(-), got {:?}", token),
        }

        // Token 9: Number(123)
        let token = expr.next_token();
        match token {
            Token::Number(_, 123) => (),
            _ => panic!("Expected Number(123), got {:?}", token),
        }

        // Token 10: MulOperator(/)
        let token = expr.next_token();
        match token {
            Token::MulOperator(_, MulOp::Divide) => (),
            _ => panic!("Expected MulOperator(/), got {:?}", token),
        }

        // Token 11: LPar(
        let token = expr.next_token();
        match token {
            Token::LPar(_) => (),
            _ => panic!("Expected LPar, got {:?}", token),
        }

        // Token 12: Number(2)
        let token = expr.next_token();
        match token {
            Token::Number(_, 2) => (),
            _ => panic!("Expected Number(2), got {:?}", token),
        }

        // Token 13: AddOperator(-)
        let token = expr.next_token();
        match token {
            Token::AddOperator(_, AddOp::Minus) => (),
            _ => panic!("Expected AddOperator(-), got {:?}", token),
        }

        // Token 14: Number(34)
        let token = expr.next_token();
        match token {
            Token::Number(_, 34) => (),
            _ => panic!("Expected Number(34), got {:?}", token),
        }

        // Token 15: RPar)
        let token = expr.next_token();
        match token {
            Token::RPar(_) => (),
            _ => panic!("Expected RPar, got {:?}", token),
        }

        // Token 16: EndMark(#)
        let token = expr.next_token();
        match token {
            Token::EndMark(_) => (),
            _ => panic!("Expected EndMark(#), got {:?}", token),
        }
    }
}
