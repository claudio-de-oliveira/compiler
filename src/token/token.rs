#![allow(dead_code)]

use crate::tags::{rust_tags, rust_tags::Tag, rust_tags::keywords};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AddOp {
    Plus,
    Minus,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ShiftDir {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MulOp {
    Times,
    Divide,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EqualityOp {
    Equal,
    NotEqual,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AssignOp {
    AddAssign,
    BitAndAssign,
    BitXorAssign,
    BitOrAssign,
    DivAssign,
    MulAssign,
    RemAssign,
    SubAssign,
    ShrAssign,
    ShlAssign,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OrderOp {
    LT,
    GT,
    LTE,
    GTE,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Keyword(Tag, String),
    Identifier(Tag, String),
    Number(Tag, String),
    LPar(Tag),
    RPar(Tag),
    EndMark(Tag),
    Error(Tag, String),
    DefaultPattern(Tag),
    Division(Tag),               // /	expr / expr	Arithmetic division	Div
    Not(Tag),                    // !	!expr	Bitwise or logical complement	Not
    Equality(Tag, EqualityOp),   // !=	expr != expr	Nonequality comparison	PartialEq
                                 // ==	expr == expr	Equality comparison	PartialEq
    Remainder(Tag),              // %	expr % expr	Arithmetic remainder	Rem
    Assignment(Tag),             // =	var = expr, ident = type	Assignment/equivalence
    OpAssignment(Tag, AssignOp), // %=	var %= expr	Arithmetic remainder and assignment	RemAssign
                                 // &=	var &= expr	Bitwise AND and assignment	BitAndAssign
                                 // *=	var *= expr	Arithmetic multiplication and assignment	MulAssign
                                 // /=	var /= expr	Arithmetic divisionn and assignment	DivAssign
                                 // +=	var += expr	Arithmetic addition and assignment	AddAssign
                                 // -=	var -= expr	Arithmetic subtraction and assignment	SubAssign
                                 // <<=	var <<= expr	Left-shift and assignment	ShlAssign
                                 // >>=	var >>= expr	Right-shift and assignment	ShrAssign
    LogicalAnd(Tag),             // &&	expr && expr	Short-circuiting logical AND
    LogicalOr(Tag),              // ||	expr || expr	Short-circuiting logical OR
    BitwiseAnd(Tag),             // &	expr & expr	Bitwise AND	BitAnd
    BitwiseOr(Tag),              // |	expr | expr	Bitwise OR	BitOr
    BitwiseXor(Tag),             // ^	expr ^ expr	Bitwise exclusive OR	BitXor
    BitwiseAndAssign(Tag),       // &=	expr & expr	Bitwise AND	BitAnd
    BitwiseOrAssign(Tag),        // |=	var |= expr	Bitwise OR and assignment	BitOrAssign
    BitwiseXorAssign(Tag),       // ^=	var ^= expr	Bitwise exclusive OR and assignment	BitXorAssign
    InterrogationSymbol(Tag),    // ?   expr?	Error propagation
    MatchArm(Tag),               // =>	pat => expr	Part of match arm syntax
    ShiftOp(Tag, ShiftDir),      // <<	<	expr << expr	Left-shift	Shl
    StarSymbol(Tag),             // * 	expr * expr	Arithmetic multiplication	Mul
                                 // *	*expr	Dereference	Deref
                                 // *	*const type, *mut type	Raw pointer
    PlusSymbol(Tag),             // +	trait + trait, 'a + trait	Compound type constraint
                                 // +	expr + expr	Arithmetic addition	Add
    CommaSymbol(Tag),            // ,   expr, expr	Argument and element separator
    MinusSymbol(Tag),            // - 	- expr	Arithmetic negation	Neg
                                 // -	expr - expr	Arithmetic subtraction	Sub
    ReturnType(Tag),             // ->	fn(...) -> type, |…| -> type	Function and closure return type
    SglPtSymbol(Tag),            // .	expr.ident	Field access
                                 // .	expr.ident(expr, ...)	Method call
                                 // .	expr.0, expr.1, and so on	Tuple indexing
    DblPtSymbol(Tag),            // ..	.., expr.., ..expr, expr..expr	Right-exclusive range literal	PartialOrd
                                 // ..	..expr	Struct literal update syntax
                                 // ..	variant(x, ..), struct_type { x, .. }	“And the rest” pattern binding
    InclusiveRange(Tag),         // ..=	..=expr, expr..=expr	Right-inclusive range literal	PartialOrd
    AmpersandSymbol(Tag),        // &	&expr, &mut expr	Borrow
                                 // &	&type, &mut type, &'a type, &'a mut type	Borrowed pointer type
                                 // &	expr & expr	Bitwise AND	BitAnd
    SemicolonSymbol(Tag),        // ;	expr;	Statement and item terminator
                                 // ;	[...; len]	Part of fixed-size array syntax
    EqualSymbol(Tag),            // =	var = expr, ident = type	Assignment/equivalence
    Comparison(Tag, OrderOp),    // <	expr < expr	Less than comparison	PartialOrd
                                 // >	expr > expr	Greater than comparison	PartialOrd
    VerticalBarSymbol(Tag),      // |	pat | pat	Pattern alternatives
                                 // |	expr | expr	Bitwise OR	BitOr
    AtSymbol(Tag),               // @	ident @ pat	Pattern binding
    ColonSymbol(Tag),            // :	pat: type, ident: type	Constraints
                                 // :	ident: expr	Struct field initializer
                                 // :	'a: loop {...}	Loop label
}

pub trait Scanner {
    fn next_token(&mut self) -> Token;
}

pub struct Rust {
    current_position: usize,
    chars: Vec<char>,
}

impl Rust {
    pub fn new(text: &str) -> Self {
        Rust {
            current_position: 0,
            chars: text.chars().collect(),
        }
    }

    #[inline]
    fn current_char(&self) -> Option<char> {
        self.chars.get(self.current_position).copied()
    }

    /// Espia o próximo caractere sem avançar a posição
    #[inline]
    fn peek_char(&self) -> Option<char> {
        self.chars.get(self.current_position + 1).copied()
    }

    /// Avança para o próximo caractere e retorna o caractere atual
    #[inline]
    fn advance(&mut self) {
        self.current_position += 1;
    }

    /// Volta uma posição com segurança (não vai abaixo de 0)
    #[inline]
    fn retract(&mut self) {
        if self.current_position > 0 {
            self.current_position -= 1;
        }
    }
}

impl Scanner for Rust {
    fn next_token(&mut self) -> Token {
        let mut state = 0;
        let mut lexema = String::new();

        loop {
            match state {
                0 => {
                    match self.current_char() {
                        Some(c) if c.is_whitespace() => {
                            self.advance();
                            state = 0;
                            continue;
                        }
                        Some(c) if c.is_alphabetic() => {
                            lexema.push(c);
                            self.advance();
                            state = 10;
                            continue;
                        }
                        Some('_') => {
                            lexema.push('_');
                            self.advance();
                            state = 10;
                            continue;
                        }
                        Some(c) if c.is_digit(10) => {
                            lexema.push(c);
                            self.advance();
                            state = 1;
                            continue;
                        }
                        Some('(') => {
                            self.advance();
                            state = 7;
                            continue;
                        }
                        Some(')') => {
                            self.advance();
                            state = 8;
                            continue;
                        }
                        Some('!') => {
                            self.advance();
                            state = 100;
                            continue;
                        }
                        Some('%') => {
                            self.advance();
                            state = 101;
                            continue;
                        }
                        Some('&') => {
                            self.advance();
                            state = 102;
                            continue;
                        }
                        Some('*') => {
                            self.advance();
                            state = 103;
                            continue;
                        }
                        Some('+') => {
                            self.advance();
                            state = 104;
                            continue;
                        }
                        Some(',') => {
                            self.advance();
                            state = 105;
                            continue;
                        }
                        Some('-') => {
                            self.advance();
                            state = 106;
                            continue;
                        }
                        Some('.') => {
                            self.advance();
                            state = 107;
                            continue;
                        }
                        Some('/') => {
                            self.advance();
                            state = 108;
                            continue;
                        }
                        Some(':') => {
                            self.advance();
                            state = 109;
                            continue;
                        }
                        Some(';') => {
                            self.advance();
                            state = 110;
                            continue;
                        }
                        Some('<') => {
                            self.advance();
                            state = 111;
                            continue;
                        }
                        Some('=') => {
                            self.advance();
                            state = 112;
                            continue;
                        }
                        Some('>') => {
                            self.advance();
                            state = 113;
                            continue;
                        }
                        Some('@') => {
                            self.advance();
                            state = 114;
                            continue;
                        }
                        Some('^') => {
                            self.advance();
                            state = 115;
                            continue;
                        }
                        Some('|') => {
                            self.advance();
                            state = 116;
                            continue;
                        }
                        Some('?') => {
                            self.advance();
                            state = 117;
                            continue;
                        }
                        Some('#') => {
                            self.advance();
                            state = 9;
                            continue;
                        }
                        Some(c) => {
                            lexema.push(c);
                            self.advance();
                            state = 999;
                            continue;
                        }
                        None => todo!(),
                    }
                }
                1 => {
                    match self.current_char() {
                        Some(c) if c.is_digit(10) => {
                            lexema.push(c);
                            self.advance();
                            state = 1;
                            continue;
                        }
                        _ => {
                            self.advance();
                            state = 2;
                            continue;
                        }
                    }
                }
                2 => {
                    self.retract(); // RETRACT
                    return Token::Number(rust_tags::Tag::NUM, lexema);
                }
                7 => {
                    return Token::LPar(Tag::LPAR);
                }
                8 => {
                    return Token::RPar(Tag::RPAR);
                }
                9 => {
                    return Token::EndMark(Tag::END);
                }
                10 => {
                    match self.current_char() {
                        Some(c) if c.is_alphanumeric() => {
                            lexema.push(c);
                            self.advance();
                            state = 10;
                            continue;
                        }
                        Some('_') => {
                            lexema.push('_');
                            self.advance();
                            state = 13;
                            continue;
                        }
                        Some('!') => {
                            lexema.push('!');
                            self.advance();
                            state = 12;
                            continue;
                        }
                        _ => {
                            self.advance();
                            state = 11;
                            continue;
                        }
                    }
                }
                11 => {
                    self.retract();
                    if lexema == "_" {
                        return Token::DefaultPattern(Tag::DEFAULT);
                    }
                    return match Tag::from_keyword(&lexema, keywords::KeywordContext::Normal) {
                        Some(tag) => Token::Keyword(tag, lexema),
                        _ => match Tag::from_keyword(&lexema, keywords::KeywordContext::Future) {
                                Some(tag) => Token::Keyword(tag, lexema),
                                _ => match Tag::from_keyword(&lexema, keywords::KeywordContext::Lifetimes) {
                                        Some(tag) => Token::Keyword(tag, lexema),
                                        _ => match Tag::from_keyword(&lexema, keywords::KeywordContext::Union) {
                                                Some(tag) => Token::Keyword(tag, lexema),
                                                _ => Token::Identifier(Tag::IDENTIFIER, lexema),
                                            },
                                    },
                            }
                    }
                }
                12 => {
                    // Macro
                    return Token::Identifier(Tag::IDENTIFIER, lexema);
                }

                100 => {
                    match self.current_char() {
                        Some('=') => {
                            self.advance();
                            state = 1001;
                            continue;
                        }
                        _  => {
                            return Token::Not(Tag::NOT);
                        }
                    }
                }
                1001 => {
                    return Token::Equality(Tag::EQUALITY, EqualityOp::NotEqual);
                }

                101 => {
                    match self.current_char() {
                        Some('=') => {
                            self.advance();
                            state = 1011;
                            continue;
                        }
                        _  => {
                            return Token::Remainder(Tag::REM);
                        }
                    }
                }
                1011 => {
                    return Token::OpAssignment(Tag::OPASSIGN, AssignOp::RemAssign);
                }

                102 => {
                    match self.current_char() {
                        Some('=') => {
                            self.advance();
                            state = 1021;
                            continue;
                        }
                        Some('&') => {
                            self.advance();
                            state = 1022;
                            continue;
                        }
                        _  => {
                            return Token::AmpersandSymbol(Tag::AMPERSAND);
                        }
                    }
                }
                1021 => {
                    return Token::OpAssignment(Tag::OPASSIGN, AssignOp::BitAndAssign);
                }
                1022 => {
                    return Token::LogicalAnd(Tag::AND);
                }

                103 => {
                    match self.current_char() {
                        Some('=') => {
                            self.advance();
                            state = 1031;
                            continue;
                        }
                        _  => {
                            return Token::StarSymbol(Tag::STAR);
                        }
                    }
                }
                1031 => {
                    return Token::OpAssignment(Tag::OPASSIGN, AssignOp::MulAssign);
                }

                104 => {
                    match self.current_char() {
                        Some('=') => {
                            self.advance();
                            state = 1041;
                            continue;
                        }
                        _  => {
                            return Token::PlusSymbol(Tag::PLUS);
                        }
                    }
                }
                1041 => {
                    return Token::OpAssignment(Tag::OPASSIGN, AssignOp::AddAssign);
                }

                105 => {
                    return Token::CommaSymbol(Tag::COMMA);
                }

                106 => {
                    match self.current_char() {
                        Some('=') => {
                            self.advance();
                            state = 1061;
                            continue;
                        }
                        Some('>') => {
                            self.advance();
                            state = 1062;
                            continue;
                        }
                        _  => {
                            return Token::MinusSymbol(Tag::MINUS);
                        }
                    }
                }
                1061 => {
                    return Token::OpAssignment(Tag::OPASSIGN, AssignOp::SubAssign);
                }
                1062 => {
                    return Token::ReturnType(Tag::ARROW);
                }

                107 => {
                    match self.current_char() {
                        Some('.') => {
                            self.advance();
                            state = 1071;
                            continue;
                        }
                        _  => {
                            return Token::SglPtSymbol(Tag::SGLPT);
                        }
                    }
                }
                1071 => {
                    match self.current_char() {
                        Some('=') => {
                            self.advance();
                            state = 10711;
                            continue;
                        }
                        _  => {
                            return Token::DblPtSymbol(Tag::DBLPT);
                        }
                    }
                }
                10711 => {
                    return Token::InclusiveRange(Tag::INRANGE);
                }

                108 => {
                    match self.current_char() {
                        Some('=') => {
                            self.advance();
                            state = 1081;
                            continue;
                        }
                        _  => {
                            return Token::Division(Tag::DIV);
                        }
                    }
                }
                1081 => {
                    return Token::OpAssignment(Tag::OPASSIGN, AssignOp::DivAssign);
                }

                109 => {
                    match self.current_char() {
                        _  => {
                            return Token::ColonSymbol(Tag::COLON);
                        }
                    }
                }

                110 => {
                    return Token::SemicolonSymbol(Tag::SEMICOLON);
                }

                111 => {
                    match self.current_char() {
                        Some('=') => {
                            self.advance();
                            state = 1111;
                            continue;
                        }
                        Some('<') => {
                            self.advance();
                            state = 1112;
                            continue;
                        }
                        _  => {
                            return Token::Comparison(Tag::LT, OrderOp::LT);
                        }
                    }
                }
                1111 => {
                    return Token::Comparison(Tag::LTE, OrderOp::LTE);
                }
                1112 => {
                    match self.current_char() {
                        Some('=') => {
                            self.advance();
                            state = 11121;
                            continue;
                        }
                        _  => {
                            return Token::ShiftOp(Tag::SHIFTOP, ShiftDir::Left);
                        }
                    }
                }
                11121 => {
                    return Token::OpAssignment(Tag::OPASSIGN, AssignOp::ShlAssign);
                }

                112 => {
                    match self.current_char() {
                        Some('=') => {
                            self.advance();
                            state = 11211;
                            continue;
                        }
                        Some('>') => {
                            self.advance();
                            state = 11212;
                            continue;
                        }
                        _  => {
                            return Token::EqualSymbol(Tag::EQUAL);
                        }
                    }
                }
                11211 => {
                    return Token::Equality(Tag::EQUALITY, EqualityOp::Equal);
                }
                11212 => {
                    return Token::MatchArm(Tag::MATCHARM);
                }

                113 => {
                    match self.current_char() {
                        Some('=') => {
                            self.advance();
                            state = 1131;
                            continue;
                        }
                        Some('>') => {
                            self.advance();
                            state = 1132;
                            continue;
                        }
                        _  => {
                            return Token::Comparison(Tag::GT, OrderOp::GT);
                        }
                    }
                }
                1131 => {
                    return Token::Comparison(Tag::GTE, OrderOp::GTE);
                }
                1132 => {
                    match self.current_char() {
                        Some('=') => {
                            self.advance();
                            state = 11311;
                            continue;
                        }
                        _  => {
                            return Token::ShiftOp(Tag::SHIFTOP, ShiftDir::Right); // >>	expr >> expr	Right-shift	Shr
                        }
                    }
                }
                11311 => {
                    return Token::OpAssignment(Tag::OPASSIGN, AssignOp::ShrAssign); // >>=	var >>= expr	Right-shift and assignment	ShrAssign
                }

                114 => {
                    // @	ident @ pat	Pattern binding
                    return Token::AtSymbol(Tag::AT);
                }

                115 => {
                    match self.current_char() {
                        Some('=') => {
                            self.advance();
                            state = 1151;
                            continue;
                        }
                        _  => {
                            return Token::BitwiseXor(Tag::BITXOR);
                        }
                    }
                }
                1151 => {
                    return Token::OpAssignment(Tag::OPASSIGN, AssignOp::BitXorAssign);
                }

                116 => {
                    match self.current_char() {
                        Some('=') => {
                            self.advance();
                            state = 1161;
                            continue;
                        }
                        Some('|') => {
                            self.advance();
                            state = 1162;
                            continue;
                        }
                        _  => {
                            return Token::VerticalBarSymbol(Tag::VBAR);
                        }
                    }
                }
                1161 => {
                    return Token::OpAssignment(Tag::OPASSIGN, AssignOp::BitOrAssign);
                }
                1162 => {
                    return Token::LogicalOr(Tag::OR);
                }

                117 => {
                    return Token::InterrogationSymbol(Tag::INTERROGATION);
                }

                999 => {
                    self.retract();
                    return Token::Error(Tag::ERR, format!("Caracter inválido: {}", lexema));
                }
                _ => {
                    return Token::Error(Tag::ERR, "Caracter inválido".to_string());
                }
            }
        }
    }
}

