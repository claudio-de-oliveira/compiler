#![allow(dead_code)]

use std::char;

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
pub enum StringLiteralType {
    Standard,       // Texto comum, mensagens de erro, nomes.
    Raw(usize),     // Regex, caminhos de arquivo, JSON/HTML manual.
    ByteString,     // Buffers de rede, assinaturas de arquivos binários.
    RawByte(usize), // Buffers que contêm muitas barras invertidas.
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FloatLiteralType {
    F32,
    F64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IntegerLiteralType {
    I8,
    I16,
    I32,
    I64,
    I128,
    ISIZE,
    U8,
    U16,
    U32,
    U64,
    U128,
    USIZE,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CommentType {
    Line,
    Block,
    ExtDoc,
    IntDoc,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Keyword(Tag, usize, usize, String),
    Identifier(Tag, usize, usize, String),
    Character(Tag, usize, usize, char),
    StringLiteral(Tag, usize, usize, StringLiteralType, String),
    Integer(Tag, usize, usize, String, IntegerLiteralType),
    Float(Tag, usize, usize, String, FloatLiteralType),
    LPar(Tag, usize, usize),
    RPar(Tag, usize, usize),
    EndMark(Tag, usize, usize),
    Error(Tag, usize, usize, String),
    DefaultPattern(Tag, usize, usize),
    Division(Tag, usize, usize),               // /	expr / expr	Arithmetic division	Div
    Not(Tag, usize, usize),                    // !	!expr	Bitwise or logical complement	Not
    Equality(Tag, usize, usize, EqualityOp),   // !=	expr != expr	Nonequality comparison	PartialEq
                                           // ==	expr == expr	Equality comparison	PartialEq
    Remainder(Tag, usize, usize),              // %	expr % expr	Arithmetic remainder	Rem
    Assignment(Tag, usize, usize),             // =	var = expr, ident = type	Assignment/equivalence
    OpAssignment(Tag, usize, usize, AssignOp), // %=	var %= expr	Arithmetic remainder and assignment	RemAssign
                                           // &=	var &= expr	Bitwise AND and assignment	BitAndAssign
                                           // *=	var *= expr	Arithmetic multiplication and assignment	MulAssign
                                           // /=	var /= expr	Arithmetic divisionn and assignment	DivAssign
                                           // +=	var += expr	Arithmetic addition and assignment	AddAssign
                                           // -=	var -= expr	Arithmetic subtraction and assignment	SubAssign
                                           // <<=	var <<= expr	Left-shift and assignment	ShlAssign
                                           // >>=	var >>= expr	Right-shift and assignment	ShrAssign
    LogicalAnd(Tag, usize, usize),             // &&	expr && expr	Short-circuiting logical AND
    LogicalOr(Tag, usize, usize),              // ||	expr || expr	Short-circuiting logical OR
    BitwiseAnd(Tag, usize, usize),             // &	expr & expr	Bitwise AND	BitAnd
    BitwiseOr(Tag, usize, usize),              // |	expr | expr	Bitwise OR	BitOr
    BitwiseXor(Tag, usize, usize),             // ^	expr ^ expr	Bitwise exclusive OR	BitXor
    BitwiseAndAssign(Tag, usize, usize),       // &=	expr & expr	Bitwise AND	BitAnd
    BitwiseOrAssign(Tag, usize, usize),        // |=	var |= expr	Bitwise OR and assignment	BitOrAssign
    BitwiseXorAssign(Tag, usize, usize),       // ^=	var ^= expr	Bitwise exclusive OR and assignment	BitXorAssign
    InterrogationSymbol(Tag, usize, usize),    // ?   expr?	Error propagation
    MatchArm(Tag, usize, usize),               // =>	pat => expr	Part of match arm syntax
    ShiftOp(Tag, usize, usize, ShiftDir),      // <<	<	expr << expr	Left-shift	Shl
    StarSymbol(Tag, usize, usize),             // * 	expr * expr	Arithmetic multiplication	Mul
                                           // *	*expr	Dereference	Deref
                                           // *	*const type, *mut type	Raw pointer
    PlusSymbol(Tag, usize, usize),             // +	trait + trait, 'a + trait	Compound type constraint
                                           // +	expr + expr	Arithmetic addition	Add
    CommaSymbol(Tag, usize, usize),            // ,   expr, expr	Argument and element separator
    MinusSymbol(Tag, usize, usize),            // - 	- expr	Arithmetic negation	Neg
                                           // -	expr - expr	Arithmetic subtraction	Sub
    ReturnType(Tag, usize, usize),             // ->	fn(...) -> type, |…| -> type	Function and closure return type
    SglPtSymbol(Tag, usize, usize),            // .	expr.ident	Field access
                                           // .	expr.ident(expr, ...)	Method call
                                           // .	expr.0, expr.1, and so on	Tuple indexing
    DblPtSymbol(Tag, usize, usize),            // ..	.., expr.., ..expr, expr..expr	Right-exclusive range literal	PartialOrd
                                           // ..	..expr	Struct literal update syntax
                                           // ..	variant(x, ..), struct_type { x, .. }	“And the rest” pattern binding
    InclusiveRange(Tag, usize, usize),         // ..=	..=expr, expr..=expr	Right-inclusive range literal	PartialOrd
    AmpersandSymbol(Tag, usize, usize),        // &	&expr, &mut expr	Borrow
                                           // &	&type, &mut type, &'a type, &'a mut type	Borrowed pointer type
                                           // &	expr & expr	Bitwise AND	BitAnd
    SemicolonSymbol(Tag, usize, usize),        // ;	expr;	Statement and item terminator
                                           // ;	[...; len]	Part of fixed-size array syntax
    EqualSymbol(Tag, usize, usize),            // =	var = expr, ident = type	Assignment/equivalence
    Comparison(Tag, usize, usize, OrderOp),    // <	expr < expr	Less than comparison	PartialOrd
                                           // >	expr > expr	Greater than comparison	PartialOrd
    VerticalBarSymbol(Tag, usize, usize),      // |	pat | pat	Pattern alternatives
                                           // |	expr | expr	Bitwise OR	BitOr
    AtSymbol(Tag, usize, usize),               // @	ident @ pat	Pattern binding
    ColonSymbol(Tag, usize, usize),            // :	pat: type, ident: type	Constraints
                                           // :	ident: expr	Struct field initializer
                                           // :	'a: loop {...}	Loop label
    Comment(Tag, usize, usize, CommentType, String),
}

pub trait Scanner {
    fn next_token(&mut self) -> Token;
}

pub struct Rust<'a> {
    text: Vec<&'a str>,
    current_row: usize,
    current_col: usize,
}

impl<'a> Rust<'a> {
    pub fn new(text: &'a str) -> Self {
        Rust {
            text: text.lines().collect(),
            current_row: 0,
            current_col: 0,
        }
    }

    #[inline]
    fn row(&self) -> usize {
        self.current_row
    }
    #[inline]
    fn col(&self) -> usize {
        self.current_col
    }

    #[inline]
    fn current_char(&self) -> Option<char> {
        assert!(self.current_col <= self.text[self.current_row].chars().count());

        if self.current_col >= self.text[self.current_row].chars().count() {
            return Some('\n');
        } else {
            self.text[self.current_row].chars().nth(self.current_col)
        }
    }

    /// Avança para o próximo caractere e retorna o caractere atual
    fn advance(&mut self) {
        self.current_col += 1;
        if self.current_col >= self.text[self.current_row].chars().count() {
            self.current_row += 1;
            self.current_col = 0;
        }
    }

    /// Volta uma posição com segurança (não vai abaixo de 0)
    fn retract(&mut self) {
        assert!(self.current_row + self.current_col > 0);

        if self.current_col > 0 {
            self.current_col -= 1;
        } else {
            self.current_row -= 1;
            self.current_col = self.text[self.current_row].chars().count();
        }
    }

    fn ignore_until_eol(&mut self) -> String {
        let mut comment = String::new();
        loop {
            match self.current_char() {
                Some('\n') => {
                    return comment;
                }
                Some(c) => {
                    comment.push(c);
                    self.advance();
                }
                None => {
                    return comment;
                }
            }
        }
    }

    fn ignore_until_eob(&mut self) -> Result<String, String> {
        let mut state = 0;
        let mut text = String::new();
        let mut counter = 1;

        loop {
            match state {
                0 => {
                    match self.current_char() {
                        Some('*') => {
                            self.advance();
                            state = 1;
                            continue;
                        }
                        Some('/') => {
                            self.advance();
                            state = 3;
                            continue;
                        }
                        Some('#') => {
                            todo!();
                        }
                        Some(c) => {
                            text.push(c);
                            self.advance();
                            state = 0;
                            continue;
                        }
                        None => {
                            todo!();
                        }
                    }
                }
                1 => {
                    match self.current_char() {
                        Some('/') if counter == 0 => {
                            self.advance();
                            state = 2;
                            continue;
                        }
                        Some('/') => {
                            text.push('*');
                            text.push('/');
                            counter -= 1;
                            self.advance();
                            state = 0;
                            continue;
                        }
                        Some(c) => {
                            text.push(c);
                            self.advance();
                            state = 0;
                            continue;
                        }
                        None => {
                            todo!();
                        }
                    }
                }
                2 => {
                    return Ok(text);
                }
                3 => {
                    match self.current_char() {
                        Some('*') => {
                            counter += 1;
                            self.advance();
                            state = 0;
                            continue;
                        }
                        Some('/') => {
                            self.advance();
                            state = 3;
                            continue;
                        }
                        Some('#') => {
                            todo!();
                        }
                        Some(c) => {
                            text.push(c);
                            self.advance();
                            state = 0;
                            continue;
                        }
                        None => {
                            todo!();
                        }
                    }
                }
                _ => {
                    unreachable!();
                }
            }
        }
    }
}

impl<'a> Scanner for Rust<'a> {
    fn next_token(&mut self) -> Token {
        let mut state = 0;
        let mut lexema = String::new();
        let mut counter = 0;
        let mut auxiliary = String::new();
        let mut string_type = StringLiteralType::Standard;

        loop {
            match state {
                0 => {
                    match self.current_char() {
                        Some(c) if c.is_whitespace() => {
                            self.advance();
                            state = 0;
                            continue;
                        }
                        Some('b') => {
                            lexema.push('b');
                            auxiliary.push('b');
                            self.advance();
                            state = 160;
                            continue;
                        }
                        Some('r') => {
                            lexema.push('r');
                            auxiliary.push('r');
                            self.advance();
                            state = 161;
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
                        Some('\'') => {
                            self.advance();
                            state = 150;
                            continue;
                        }
                        Some('\"') => {
                            self.advance();
                            state = 1620;
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
                        Some('_') => {
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
                    return Token::Integer(rust_tags::Tag::INTEGER, self.row(), self.col(), lexema, IntegerLiteralType::ISIZE);
                }
                7 => {
                    return Token::LPar(Tag::LPAR, self.row(), self.col());
                }
                8 => {
                    return Token::RPar(Tag::RPAR, self.row(), self.col());
                }
                9 => {
                    return Token::EndMark(Tag::END, self.row(), self.col());
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
                        return Token::DefaultPattern(Tag::DEFAULT, self.row(), self.col());
                    }

                    return match Tag::from_keyword(&lexema, keywords::KeywordContext::Normal) {
                        Some(tag) => Token::Keyword(tag, self.row(), self.col(), lexema),
                        _ => match Tag::from_keyword(&lexema, keywords::KeywordContext::Future) {
                                Some(tag) => Token::Keyword(tag, self.row(), self.col(), lexema),
                                _ => match Tag::from_keyword(&lexema, keywords::KeywordContext::Lifetimes) {
                                        Some(tag) => Token::Keyword(tag, self.row(), self.col(), lexema),
                                        _ => match Tag::from_keyword(&lexema, keywords::KeywordContext::Union) {
                                                Some(tag) => Token::Keyword(tag, self.row(), self.col(), lexema),
                                                _ => Token::Identifier(Tag::IDENTIFIER, self.row(), self.col(), lexema),
                                            },
                                    },
                            }
                    }
                }
                12 => {
                    // Macro
                    return Token::Identifier(Tag::IDENTIFIER, self.row(), self.col(), lexema);
                }

                100 => {
                    match self.current_char() {
                        Some('=') => {
                            self.advance();
                            state = 1001;
                            continue;
                        }
                        _  => {
                            return Token::Not(Tag::NOT, self.row(), self.col());
                        }
                    }
                }
                1001 => {
                    return Token::Equality(Tag::EQUALITY, self.row(), self.col(), EqualityOp::NotEqual);
                }

                101 => {
                    match self.current_char() {
                        Some('=') => {
                            self.advance();
                            state = 1011;
                            continue;
                        }
                        _  => {
                            return Token::Remainder(Tag::REM, self.row(), self.col());
                        }
                    }
                }
                1011 => {
                    return Token::OpAssignment(Tag::OPASSIGN, self.row(), self.col(), AssignOp::RemAssign);
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
                            return Token::AmpersandSymbol(Tag::AMPERSAND, self.row(), self.col());
                        }
                    }
                }
                1021 => {
                    return Token::OpAssignment(Tag::OPASSIGN, self.row(), self.col(), AssignOp::BitAndAssign);
                }
                1022 => {
                    return Token::LogicalAnd(Tag::AND, self.row(), self.col());
                }

                103 => {
                    match self.current_char() {
                        Some('=') => {
                            self.advance();
                            state = 1031;
                            continue;
                        }
                        _  => {
                            return Token::StarSymbol(Tag::STAR, self.row(), self.col());
                        }
                    }
                }
                1031 => {
                    return Token::OpAssignment(Tag::OPASSIGN, self.row(), self.col(), AssignOp::MulAssign);
                }

                104 => {
                    match self.current_char() {
                        Some('=') => {
                            self.advance();
                            state = 1041;
                            continue;
                        }
                        _  => {
                            return Token::PlusSymbol(Tag::PLUS, self.row(), self.col());
                        }
                    }
                }
                1041 => {
                    return Token::OpAssignment(Tag::OPASSIGN, self.row(), self.col(), AssignOp::AddAssign);
                }

                105 => {
                    return Token::CommaSymbol(Tag::COMMA, self.row(), self.col());
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
                            return Token::MinusSymbol(Tag::MINUS, self.row(), self.col());
                        }
                    }
                }
                1061 => {
                    return Token::OpAssignment(Tag::OPASSIGN, self.row(), self.col(), AssignOp::SubAssign);
                }
                1062 => {
                    return Token::ReturnType(Tag::ARROW, self.row(), self.col());
                }

                107 => {
                    match self.current_char() {
                        Some('.') => {
                            self.advance();
                            state = 1071;
                            continue;
                        }
                        _  => {
                            return Token::SglPtSymbol(Tag::SGLPT, self.row(), self.col());
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
                            return Token::DblPtSymbol(Tag::DBLPT, self.row(), self.col());
                        }
                    }
                }
                10711 => {
                    return Token::InclusiveRange(Tag::INRANGE, self.row(), self.col());
                }

                108 => {
                    match self.current_char() {
                        Some('/') => {
                            self.advance();
                            state = 1082;
                            continue;
                        }
                        Some('*') => {
                            self.advance();
                            state = 1085;
                            continue;
                        }
                        Some('=') => {
                            self.advance();
                            state = 1081;
                            continue;
                        }
                        _  => {
                            return Token::Division(Tag::DIV, self.row(), self.col());
                        }
                    }
                }
                1081 => {
                    return Token::OpAssignment(Tag::OPASSIGN, self.row(), self.col(), AssignOp::DivAssign);
                }
                1082 => {
                    match self.current_char() {
                        Some('/') => {
                            self.advance();
                            state = 1083;
                            continue;
                        }
                        Some('!') => {
                            self.advance();
                            state = 1084;
                            continue;
                        }
                        _  => {
                            let text = self.ignore_until_eol();
                            return Token::Comment(Tag::COMMENT, self.row(), self.col(), CommentType::Line, text);
                        }
                    }
                }
                1083 => {
                    let text = self.ignore_until_eol();
                    return Token::Comment(Tag::COMMENT, self.row(), self.col(), CommentType::ExtDoc, text);
                }
                1084 => {
                    let text = self.ignore_until_eol();
                    return Token::Comment(Tag::COMMENT, self.row(), self.col(), CommentType::IntDoc, text);
                }
                1085 => {
                    let text = self.ignore_until_eol();
                    return Token::Comment(Tag::COMMENT, self.row(), self.col(), CommentType::Block, text);
                }

                109 => {
                    match self.current_char() {
                        _  => {
                            return Token::ColonSymbol(Tag::COLON, self.row(), self.col());
                        }
                    }
                }

                110 => {
                    return Token::SemicolonSymbol(Tag::SEMICOLON, self.row(), self.col());
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
                            return Token::Comparison(Tag::LT, self.row(), self.col(), OrderOp::LT);
                        }
                    }
                }
                1111 => {
                    return Token::Comparison(Tag::LTE, self.row(), self.col(), OrderOp::LTE);
                }
                1112 => {
                    match self.current_char() {
                        Some('=') => {
                            self.advance();
                            state = 11121;
                            continue;
                        }
                        _  => {
                            return Token::ShiftOp(Tag::SHIFTOP, self.row(), self.col(), ShiftDir::Left);
                        }
                    }
                }
                11121 => {
                    return Token::OpAssignment(Tag::OPASSIGN, self.row(), self.col(), AssignOp::ShlAssign);
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
                            return Token::EqualSymbol(Tag::EQUAL, self.row(), self.col());
                        }
                    }
                }
                11211 => {
                    return Token::Equality(Tag::EQUALITY, self.row(), self.col(), EqualityOp::Equal);
                }
                11212 => {
                    return Token::MatchArm(Tag::MATCHARM, self.row(), self.col());
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
                            return Token::Comparison(Tag::GT, self.row(), self.col(), OrderOp::GT);
                        }
                    }
                }
                1131 => {
                    return Token::Comparison(Tag::GTE, self.row(), self.col(), OrderOp::GTE);
                }
                1132 => {
                    match self.current_char() {
                        Some('=') => {
                            self.advance();
                            state = 11311;
                            continue;
                        }
                        _  => {
                            return Token::ShiftOp(Tag::SHIFTOP, self.row(), self.col(), ShiftDir::Right);
                        }
                    }
                }
                11311 => {
                    return Token::OpAssignment(Tag::OPASSIGN, self.row(), self.col(), AssignOp::ShrAssign);
                }

                114 => {
                    return Token::AtSymbol(Tag::AT, self.row(), self.col());
                }

                115 => {
                    match self.current_char() {
                        Some('=') => {
                            self.advance();
                            state = 1151;
                            continue;
                        }
                        _  => {
                            return Token::BitwiseXor(Tag::BITXOR, self.row(), self.col());
                        }
                    }
                }
                1151 => {
                    return Token::OpAssignment(Tag::OPASSIGN, self.row(), self.col(), AssignOp::BitXorAssign);
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
                            return Token::VerticalBarSymbol(Tag::VBAR, self.row(), self.col());
                        }
                    }
                }
                1161 => {
                    return Token::OpAssignment(Tag::OPASSIGN, self.row(), self.col(), AssignOp::BitOrAssign);
                }
                1162 => {
                    return Token::LogicalOr(Tag::OR, self.row(), self.col());
                }

                117 => {
                    return Token::InterrogationSymbol(Tag::INTERROGATION, self.row(), self.col());
                }
                150 => {
                    let char = self.peek_char();

                    if char == None {
                        return Token::Error(Tag::ERR, self.row(), self.col(), "Literal de caractere inválido".to_string());
                    }

                    lexema.push(char.unwrap());

                    match self.current_char() {
                        Some('\'') => {
                            self.advance();
                            state = 1599;
                            continue;
                        }
                        _ => {
                            return Token::Error(Tag::ERR, self.row(), self.col(), "Literal de caractere inválido".to_string());
                        }
                    }
                }

                1599 => {
                    return Token::Character(Tag::CHARACTER, self.row(), self.col(), lexema.chars().nth(0).unwrap_or('\0'));
                }

                160 => {
                    match self.current_char() {
                        Some('r') => {  // br
                            lexema.push('r');
                            auxiliary.push('r');
                            self.advance();
                            state = 161;
                            continue;
                        }
                        Some('\"') => {  // bu
                            self.advance();
                            state = 1620;
                            continue;
                        }
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
                161 => {
                    match self.current_char() {
                        Some('\"') => {
                            self.advance();
                            state = 1620;
                            continue;
                        }
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
                        Some('#') => {  // bu
                            counter += 1;
                            self.advance();
                            state = 1602;
                            continue;
                        }
                        _ => {
                            self.advance();
                            state = 11;
                            continue;
                        }
                    }
                }
                1601 => {
                    match self.current_char() {
                        Some('\"') => {
                            self.advance();
                            state = 1620;
                            continue;
                        }
                        Some('#') => {  // bu
                            counter += 1;
                            self.advance();
                            state = 1602;
                            continue;
                        }
                        _ => {
                            return Token::Error(Tag::ERR, self.row(), self.col(), "Identificador de byte inválido".to_string());
                        }
                    }
                }
                1602 => {
                    match self.current_char() {
                        Some('\"') => {  // bu
                            self.advance();
                            state = 1620;
                            continue;
                        }
                        Some('#') => {  // bu
                            counter += 1;
                            self.advance();
                            state = 1602;
                            continue;
                        }
                        _ => {
                            return Token::Error(Tag::ERR, self.row(), self.col(), "Identificador de byte inválido".to_string());
                        }
                    }
                }
                1620 => {
                    if auxiliary == "b" {
                        string_type = StringLiteralType::ByteString;
                    } else if auxiliary == "r" {
                        string_type = StringLiteralType::Raw(counter);
                    } else if auxiliary == "br" {
                        string_type = StringLiteralType::RawByte(counter);
                    } else {
                        string_type = StringLiteralType::Standard;
                    }
                    lexema.clear();
                    self.advance();
                    state = 1622;
                    continue;
                }
                1622 => {
                    self.retract();
                    match self.current_char() {
                        Some('\"') => {
                            self.advance();
                            state = 1680;
                            continue;
                        }
                        Some('\\') if string_type == StringLiteralType::Standard || string_type == StringLiteralType::ByteString => {
                            lexema.push('\\');
                            self.advance();
                            state = 1623;
                            continue;
                        }
                        Some('\\') => {
                            lexema.push('\\');
                            self.advance();
                            state = 1625;
                            continue;
                        }
                        Some(c) => {
                            lexema.push(c);
                            self.advance();
                            state = 1624;
                            continue;
                        }
                        _ => {
                            return Token::Error(Tag::ERR, self.row(), self.col(), "Literal de string inválido".to_string());
                        }
                    }
                }
                1623 => {
                    match self.current_char() {
                        Some(c) => {
                            lexema.push(c);
                            self.advance();
                            state = 1622;
                            continue;
                        }
                        _ => {
                            return Token::Error(Tag::ERR, self.row(), self.col(), "Literal de string inválido".to_string());
                        }
                    }
                }

                1624 => {
                    match self.current_char() {
                        Some('\"') => {
                            self.advance();
                            state = 1680;
                            continue;
                        }
                        Some('\\') if string_type == StringLiteralType::Standard || string_type == StringLiteralType::ByteString => {
                            lexema.push('\\');
                            self.advance();
                            state = 1623;
                            continue;
                        }
                        Some('\\') => {
                            lexema.push('\\');
                            self.advance();
                            state = 1625;
                            continue;
                        }
                        Some(c) => {
                            lexema.push(c);
                            self.advance();
                            state = 1624;
                            continue;
                        }
                        _ => {
                            return Token::Error(Tag::ERR, self.row(), self.col(), "Literal de string inválido".to_string());
                        }
                    }
                }
                1625 => {
                    match self.current_char() {
                        Some('u') => {  // \u{...}
                            self.advance();
                            state = 1630;
                            continue;
                        }
                        Some('n') => {  // \n
                            lexema.push('\n');
                            self.advance();
                            state = 1650;
                            continue;
                        }
                        Some('r') => {  // \r
                            lexema.push('\r');
                            self.advance();
                            state = 1650;
                            continue;
                        }
                        Some('t') => {  // \t
                            lexema.push('\t');
                            self.advance();
                            state = 1650;
                            continue;
                        }
                        Some('\\') => {  // \\
                            lexema.push('\\');
                            self.advance();
                            state = 1650;
                            continue;
                        }
                        Some('\'') => {  // \'
                            lexema.push('\'');
                            self.advance();
                            state = 1650;
                            continue;
                        }
                        Some('\"') => {  // \"
                            lexema.push('"');
                            self.advance();
                            state = 1650;
                            continue;
                        }
                        Some(c) => {
                            lexema.push(c);
                            self.advance();
                            state = 1622;
                            continue;
                        }
                        _ => {
                            return Token::Error(Tag::ERR, self.row(), self.col(), "Literal de string inválido".to_string());
                        }
                    }
                }

                1630 => {
                    match self.current_char() {
                        Some('{') => {
                            self.advance();
                            state = 1631;
                            continue;
                        }
                        _ => {
                            return Token::Error(Tag::ERR, self.row(), self.col(), "Literal de caractere inválido".to_string());
                        }
                    }
                }
                1631 => {
                    match self.current_char() {
                        Some(c) if c.is_digit(16) => {  // primeiro
                            lexema.push(c);
                            self.advance();
                            state = 1632;
                            continue;
                        }
                        _ => {
                            return Token::Error(Tag::ERR, self.row(), self.col(), "Literal de caractere inválido".to_string());
                        }
                    }
                }
                1632 => {
                    match self.current_char() {
                        Some(c) if c.is_digit(16) => {  // segundo
                            lexema.push(c);
                            self.advance();
                            state = 1633;
                            continue;
                        }
                        Some('}') => {
                            self.advance();
                            state = 1638;
                            continue;
                        }
                        _ => {
                            return Token::Error(Tag::ERR, self.row(), self.col(), "Literal de caractere inválido".to_string());
                        }
                    }
                }
                1633 => {
                    match self.current_char() {
                        Some(c) if c.is_digit(16) => {  // terceiro
                            lexema.push(c);
                            self.advance();
                            state = 1634;
                            continue;
                        }
                        Some('}') => {
                            self.advance();
                            state = 1638;
                            continue;
                        }
                        _ => {
                            return Token::Error(Tag::ERR, self.row(), self.col(), "Literal de caractere inválido".to_string());
                        }
                    }
                }
                1634 => {
                    match self.current_char() {
                        Some(c) if c.is_digit(16) => {  // quarto
                            lexema.push(c);
                            self.advance();
                            state = 1635;
                            continue;
                        }
                        Some('}') => {
                            self.advance();
                            state = 1638;
                            continue;
                        }
                        _ => {
                            return Token::Error(Tag::ERR, self.row(), self.col(), "Literal de caractere inválido".to_string());
                        }
                    }
                }
                1635 => {
                    match self.current_char() {
                        Some(c) if c.is_digit(16) => {  // quinto
                            lexema.push(c);
                            self.advance();
                            state = 1636;
                            continue;
                        }
                        Some('}') => {
                            self.advance();
                            state = 1638;
                            continue;
                        }
                        _ => {
                            return Token::Error(Tag::ERR, self.row(), self.col(), "Literal de caractere inválido".to_string());
                        }
                    }
                }
                1636 => {
                    match self.current_char() {
                        Some(c) if c.is_digit(16) => {  // sexto
                            lexema.push(c);
                            self.advance();
                            state = 1637;
                            continue;
                        }
                        Some('}') => {
                            self.advance();
                            state = 1638;
                            continue;
                        }
                        _ => {
                            return Token::Error(Tag::ERR, self.row(), self.col(), "Literal de caractere inválido".to_string());
                        }
                    }
                }
                1637 => {
                    match self.current_char() {
                        Some('}') => {
                            self.advance();
                            state = 1638;
                            continue;
                        }
                        _ => {
                            return Token::Error(Tag::ERR, self.row(), self.col(), "Literal de caractere inválido".to_string());
                        }
                    }
                }
                1638 => {
                    lexema.push(self.current_char().unwrap());
                    self.advance();
                    state = 1622;
                    continue;
                }

                1650 => {
                    lexema.push(self.current_char().unwrap());
                    self.advance();
                    state = 1622;
                    continue;
                }
                1660 => {
                    lexema.push(self.current_char().unwrap());
                    self.advance();
                    state = 1622;
                    continue;
                }

                1680 => {
                    match self.current_char() {
                        Some('#') => {
                            counter -= 1;
                            self.advance();
                            state = 1680;
                            continue;
                        }
                        _ => {
                            self.advance();
                            state = 1681;
                            continue;
                        }
                    }
                }
                1681 => {
                    self.retract();
                    if counter == 0 {
                        return Token::StringLiteral(Tag::STRING, self.row(), self.col(), string_type, lexema);
                    }
                    return Token::Error(Tag::ERR, self.row(), self.col(), "Literal de string inválido".to_string());
                }
                1690 => {
                    todo!();
                }

                999 => {
                    self.retract();
                    return Token::Error(Tag::ERR, self.row(), self.col(), format!("Caracter inválido: {}", lexema));
                }
                _ => {
                    return Token::Error(Tag::ERR, self.row(), self.col(), "Caracter inválido".to_string());
                }
            }
        }
    }

}

impl Rust<'_> {
    fn peek_char(&mut self) -> Option<char> {
        let mut state = 0;
        let mut lexema = String::new();
        let mut auxiliary = String::new();
        let mut position = self.get_position();

        loop {
            match state {
                0 => {
                    match self.current_char() {
                        Some('\\') => {
                            auxiliary.push('\\');
                            self.advance();
                            state = 1;
                            continue;
                        }
                        Some(c) => {
                            auxiliary.push(c);
                            lexema.push(c);
                            self.advance();
                            state = 12;
                            continue;
                        }
                        _ => {
                            self.set_position(position);
                            return None;
                        }
                    }
                }
                1 => {
                    match self.current_char() {
                        Some('u') => {  // \u{...}
                            auxiliary.push('u');
                            self.advance();
                            state = 2;
                            continue;
                        }
                        Some('n') => {  // \n
                            auxiliary.push('n');
                            lexema.push('\n');
                            self.advance();
                            state = 12;
                            continue;
                        }
                        Some('r') => {  // \r
                            auxiliary.push('r');
                            lexema.push('\r');
                            self.advance();
                            state = 12;
                            continue;
                        }
                        Some('t') => {  // \t
                            auxiliary.push('t');
                            lexema.push('\t');
                            self.advance();
                            state = 12;
                            continue;
                        }
                        Some('\\') => {  // \\
                            auxiliary.push('\\');
                            lexema.push('\\');
                            self.advance();
                            state = 12;
                            continue;
                        }
                        Some('\'') => {  // \'
                            auxiliary.push('\'');
                            lexema.push('\'');
                            self.advance();
                            state = 12;
                            continue;
                        }
                        Some('\"') => {  // \"
                            auxiliary.push('\"');
                            lexema.push('\"');
                            self.advance();
                            state = 12;
                            continue;
                        }
                        _ => {
                            self.set_position(position);
                            return None;
                        }
                    }
                }
                2 => {
                    match self.current_char() {
                        Some('{') => {
                            auxiliary.push('{');
                            self.advance();
                            state = 3;
                            continue;
                        }
                        _ => {
                            self.set_position(position);
                            return None;
                        }
                    }
                }
                3 => {
                    match self.current_char() {
                        Some(c) if c.is_digit(16) => {  // primeiro
                            auxiliary.push(c);
                            lexema.push(c);
                            self.advance();
                            state = 4;
                            continue;
                        }
                        _ => {
                            self.set_position(position);
                            return None;
                        }
                    }
                }
                4 => {
                    match self.current_char() {
                        Some(c) if c.is_digit(16) => {  // segundo
                            auxiliary.push(c);
                            lexema.push(c);
                            self.advance();
                            state = 5;
                            continue;
                        }
                        Some('}') => {
                            auxiliary.push('}');
                            self.advance();
                            state = 10;
                            continue;
                        }
                        _ => {
                            self.set_position(position);
                            return None;
                        }
                    }
                }
                5 => {
                    match self.current_char() {
                        Some(c) if c.is_digit(16) => {  // terceiro
                            auxiliary.push(c);
                            lexema.push(c);
                            self.advance();
                            state = 6;
                            continue;
                        }
                        Some('}') => {
                            auxiliary.push('}');
                            self.advance();
                            state = 10;
                            continue;
                        }
                        _ => {
                            self.set_position(position);
                            return None;
                        }
                    }
                }
                6 => {
                    match self.current_char() {
                        Some(c) if c.is_digit(16) => {  // quarto
                            auxiliary.push(c);
                            lexema.push(c);
                            self.advance();
                            state = 7;
                            continue;
                        }
                        Some('}') => {
                            auxiliary.push('}');
                            self.advance();
                            state = 10;
                            continue;
                        }
                        _ => {
                            self.set_position(position);
                            return None;
                        }
                    }
                }
                7 => {
                    match self.current_char() {
                        Some(c) if c.is_digit(16) => {  // quinto
                            auxiliary.push(c);
                            lexema.push(c);
                            self.advance();
                            state = 8;
                            continue;
                        }
                        Some('}') => {
                            auxiliary.push('}');
                            self.advance();
                            state = 10;
                            continue;
                        }
                        _ => {
                            self.set_position(position);
                            return None;
                        }
                    }
                }
                8 => {
                    match self.current_char() {
                        Some(c) if c.is_digit(16) => {  // sexto
                            auxiliary.push(c);
                            lexema.push(c);
                            self.advance();
                            state = 9;
                            continue;
                        }
                        Some('}') => {
                            auxiliary.push('}');
                            self.advance();
                            state = 10;
                            continue;
                        }
                        _ => {
                            self.set_position(position);
                            return None;
                        }
                    }
                }
                9 => {
                    match self.current_char() {
                        Some('}') => {
                            auxiliary.push('}');
                            self.advance();
                            state = 10;
                            continue;
                        }
                        _ => {
                            self.set_position(position);
                            return None;
                        }
                    }
                }
                10 => {
                    return std::char::from_u32(u32::from_str_radix(&lexema, 16).unwrap_or(0));
                }
                12 => {
                    return lexema.chars().nth(0);
                }
                _ => {
                    self.set_position(position);
                    return None;
                }
            }
        }
    }

    pub fn peek_number(&mut self) -> Token {
        let mut state = 0;
        let mut lexema = String::new();
        let mut signed = true;

        loop {
            match state {
                0 => {
                    match self.current_char() {
                        Some('0') => {
                            lexema.push('0');
                            self.advance();
                            state = 1;
                            continue;
                        }
                        Some('.') => {
                            lexema.push('0');
                            self.advance();
                            state = 7;
                            continue;
                        }
                        _ => {
                            self.advance();
                            state = 25;
                            continue;
                        }
                    }
                }
                1 => {
                    match self.current_char() {
                        Some('.') => {
                            lexema.push('0');
                            self.advance();
                            state = 7;
                            continue;
                        }
                        Some('i') => {
                            signed = true;
                            self.advance();
                            state = 9;
                            continue;
                        }
                        Some('u') => {
                            signed = false;
                            self.advance();
                            state = 9;
                            continue;
                        }
                        Some('f') => {
                            self.advance();
                            state = 36;
                            continue;
                        }
                        Some('b') | Some('B') => {
                            lexema.clear();
                            self.advance();
                            state = 3;
                            continue;
                        }
                        Some('o') | Some('O') => {
                            lexema.clear();
                            self.advance();
                            state = 4;
                            continue;
                        }
                        Some('x') | Some('X') => {
                            lexema.clear();
                            self.advance();
                            state = 5;
                            continue;
                        }
                        Some(c) if c.is_digit(10) => {
                            lexema.push(c);
                            self.advance();
                            state = 25;
                            continue;
                        }
                        Some('_') => {
                            self.advance();
                            state = 25;
                            continue;
                        }
                        _ => {
                            self.advance();
                            state = 26;
                            continue;
                        }
                    }
                }
                3 => {
                    match self.current_char() {
                        Some(c) if c.is_digit(2) => {
                            lexema.push(c);
                            self.advance();
                            state = 8;
                            continue;
                        }
                        Some('_') => {
                            self.advance();
                            state = 8;
                            continue;
                        }
                        _ => {
                            todo!("Terra");
                        }
                    }
                }
                4 => {
                    match self.current_char() {
                        Some(c) if c.is_digit(8) => {
                            lexema.push(c);
                            self.advance();
                            state = 23;
                            continue;
                        }
                        Some('_') => {
                            self.advance();
                            state = 23;
                            continue;
                        }
                        _ => {
                            todo!("Terra");
                        }
                    }
                }
                5 => {
                    match self.current_char() {
                        Some(c) if c.is_digit(16) => {
                            lexema.push(c);
                            self.advance();
                            state = 24;
                            continue;
                        }
                        Some('_') => {
                            self.advance();
                            state = 24;
                            continue;
                        }
                        _ => {
                            todo!("Terra");
                        }
                    }
                }
                6 => {

                }
                7 => {
                    match self.current_char() {
                        Some(c) if c.is_digit(10) => {
                            lexema.push(c);
                            self.advance();
                            state = 35;
                            continue;
                        }
                        Some('_') => {
                            self.advance();
                            state = 35;
                            continue;
                        }
                        _ => {
                            todo!("Terra");
                        }
                    }
                }
                8 => {
                    match self.current_char() {
                        Some('_') => {
                            self.advance();
                            state = 8;
                            continue;
                        }
                        Some(c) if c.is_digit(2) => {
                            lexema.push(c);
                            self.advance();
                            state = 8;
                            continue;
                        }
                        Some('i') => {
                            signed = true;
                            self.advance();
                            state = 9;
                            continue;
                        }
                        Some('u') => {
                            signed = false;
                            self.advance();
                            state = 9;
                            continue;
                        }
                        _ => {
                            state = 27;
                            self.advance();
                            continue;
                        }
                    }
                }
                9 => {
                    match self.current_char() {
                        Some('1') => {
                            self.advance();
                            state = 11;
                            continue;
                        }
                        Some('3') => {
                            self.advance();
                            state = 15;
                            continue;
                        }
                        Some('6') => {
                            self.advance();
                            state = 17;
                            continue;
                        }
                        Some('8') => {
                            self.advance();
                            state = 10;
                            continue;
                        }
                        Some('s') => {
                            self.advance();
                            state = 19;
                            continue;
                        }
                        _ => {
                            todo!("Terra");
                        }
                    }
                }
                10 => {
                    // u8, i8
                    if signed {
                        return Token::Integer(Tag::INTEGER, self.row(), self.col(), lexema, IntegerLiteralType::I8);
                    } else {
                        return Token::Integer(Tag::INTEGER, self.row(), self.col(), lexema, IntegerLiteralType::U8);
                    }
                }
                11 => {
                    match self.current_char() {
                        Some('2') => {
                            self.advance();
                            state = 13;
                            continue;
                        }
                        Some('6') => {
                            self.advance();
                            state = 12;
                            continue;
                        }
                        _ => {
                            todo!("Terra");
                        }
                    }
                }
                12 => {
                    // u16, i16
                    if signed {
                        return Token::Integer(Tag::INTEGER, self.row(), self.col(), lexema, IntegerLiteralType::I16);
                    } else {
                        return Token::Integer(Tag::INTEGER, self.row(), self.col(), lexema, IntegerLiteralType::U16);
                    }
                }
                13 => {
                    match self.current_char() {
                        Some('8') => {
                            self.advance();
                            state = 14;
                            continue;
                        }
                        _ => {
                            todo!("Terra");
                        }
                    }
                }
                14 => {
                    // u128, i128
                    if signed {
                        return Token::Integer(Tag::INTEGER, self.row(), self.col(), lexema, IntegerLiteralType::I128);
                    } else {
                        return Token::Integer(Tag::INTEGER, self.row(), self.col(), lexema, IntegerLiteralType::U128);
                    }
                }
                15 => {
                    match self.current_char() {
                        Some('2') => {
                            self.advance();
                            state = 16;
                            continue;
                        }
                        _ => {
                            todo!("Terra");
                        }
                    }
                }
                16 => {
                    // u32, i32
                    if signed {
                        return Token::Integer(Tag::INTEGER, self.row(), self.col(), lexema, IntegerLiteralType::I32);
                    } else {
                        return Token::Integer(Tag::INTEGER, self.row(), self.col(), lexema, IntegerLiteralType::U32);
                    }
                }
                17 => {
                    match self.current_char() {
                        Some('4') => {
                            self.advance();
                            state = 18;
                            continue;
                        }
                        _ => {
                            todo!("Terra");
                        }
                    }
                }
                18 => {
                    // u64, i64
                    if signed {
                        return Token::Integer(Tag::INTEGER, self.row(), self.col(), lexema, IntegerLiteralType::I64);
                    } else {
                        return Token::Integer(Tag::INTEGER, self.row(), self.col(), lexema, IntegerLiteralType::U64);
                    }
                }
                19 => {
                    match self.current_char() {
                        Some('i') => {
                            self.advance();
                            state = 20;
                            continue;
                        }
                        _ => {
                            todo!("Terra");
                        }
                    }
                }
                20 => {
                    match self.current_char() {
                        Some('z') => {
                            self.advance();
                            state = 21;
                            continue;
                        }
                        _ => {
                            todo!("Terra");
                        }
                    }
                }
                21 => {
                    match self.current_char() {
                        Some('e') => {
                            self.advance();
                            state = 22;
                            continue;
                        }
                        _ => {
                            todo!("Terra");
                        }
                    }
                }
                22 => {
                    // usize, isize
                    if signed {
                        return Token::Integer(Tag::INTEGER, self.row(), self.col(), lexema, IntegerLiteralType::ISIZE);
                    } else {
                        return Token::Integer(Tag::INTEGER, self.row(), self.col(), lexema, IntegerLiteralType::USIZE);
                    }
                }
                23 => {
                    match self.current_char() {
                        Some(c) if c.is_digit(8) => {
                            lexema.push(c);
                            self.advance();
                            state = 23;
                            continue;
                        }
                        Some('_') => {
                            self.advance();
                            state = 23;
                            continue;
                        }
                        Some('i') => {
                            signed = true;
                            self.advance();
                            state = 9;
                            continue;
                        }
                        Some('u') => {
                            signed = false;
                            self.advance();
                            state = 9;
                            continue;
                        }
                        _ => {
                            self.advance();
                            state = 28;
                            continue;
                        }
                    }
                }
                24 => {
                    match self.current_char() {
                        Some(c) if c.is_digit(16) => {
                            lexema.push(c);
                            self.advance();
                            state = 24;
                            continue;
                        }
                        Some('_') => {
                            self.advance();
                            state = 24;
                            continue;
                        }
                        Some('i') => {
                            signed = true;
                            self.advance();
                            state = 9;
                            continue;
                        }
                        Some('u') => {
                            signed = false;
                            self.advance();
                            state = 9;
                            continue;
                        }
                        _ => {
                            self.advance();
                            state = 29;
                            continue;
                        }
                    }
                }
                25 => {
                    match self.current_char() {
                        Some('.') => {
                            lexema.push('.');
                            self.advance();
                            state = 35;
                            continue;
                        }
                        Some(c) if c.is_digit(10) => {
                            lexema.push(c);
                            self.advance();
                            state = 25;
                            continue;
                        }
                        Some('_') => {
                            self.advance();
                            state = 25;
                            continue;
                        }
                        Some('e') | Some('E') => {
                            signed = true;
                            self.advance();
                            state = 32;
                            continue;
                        }
                        Some('i') => {
                            signed = true;
                            self.advance();
                            state = 9;
                            continue;
                        }
                        Some('u') => {
                            signed = false;
                            self.advance();
                            state = 9;
                            continue;
                        }
                        _ => {
                            self.advance();
                            state = 30;
                            continue;
                        }
                    }
                }
                27 => {
                    self.retract();
                    //todo!("Converter binário para inteiro");
                    return Token::Integer(Tag::INTEGER, self.row(), self.col(), lexema, IntegerLiteralType::ISIZE);
                }
                28 => {
                    self.retract();
                    //todo!("Converter octal para inteiro");
                    return Token::Integer(Tag::INTEGER, self.row(), self.col(), lexema, IntegerLiteralType::ISIZE);
                }
                29 => {
                    self.retract();
                    //todo!("Converter hexa para inteiro");
                    return Token::Integer(Tag::INTEGER, self.row(), self.col(), lexema, IntegerLiteralType::ISIZE);
                }
                30 => {
                    self.retract();
                }
                31 => {
                    self.retract();
                    return Token::Integer(Tag::INTEGER, self.row(), self.col(), lexema, IntegerLiteralType::ISIZE);
                }
                32 => {
                    match self.current_char() {
                        Some(c) if c.is_digit(10) => {
                            lexema.push(c);
                            self.advance();
                            state = 34;
                            continue;
                        }
                        Some('+') => {
                            lexema.push('+');
                            self.advance();
                            state = 33;
                            continue;
                        }
                        Some('-') => {
                            lexema.push('-');
                            self.advance();
                            state = 33;
                            continue;
                        }
                        _ => {
                            self.advance();
                            state = 30;
                            continue;
                        }
                    }
                }
                33 => {
                    match self.current_char() {
                        Some(c) if c.is_digit(10) => {
                            lexema.push(c);
                            self.advance();
                            state = 25;
                            continue;
                        }
                        Some('_') => {
                            self.advance();
                            state = 25;
                            continue;
                        }
                        _ => {
                            todo!("Terra");
                        }
                    }
                }
                34 => {
                    match self.current_char() {
                        Some(c) if c.is_digit(10) => {
                            lexema.push(c);
                            self.advance();
                            state = 34;
                            continue;
                        }
                        _ => {
                            self.advance();
                            state = 40;
                            continue;
                        }
                    }
                }
                35 => {
                    match self.current_char() {
                        Some(c) if c.is_digit(10) => {
                            lexema.push(c);
                            self.advance();
                            state = 35;
                            continue;
                        }
                        Some('_') => {
                            self.advance();
                            state = 35;
                            continue;
                        }
                        Some('f') => {
                            lexema.push('f');
                            self.advance();
                            state = 36;
                            continue;
                        }
                        Some('e') | Some('E') => {
                            lexema.push('f');
                            self.advance();
                            state = 32;
                            continue;
                        }
                        _ => {
                            self.advance();
                            state = 31;
                            continue;
                        }
                    }
                }
                36 => {
                    match self.current_char() {
                        Some('3') => {
                            lexema.push('3');
                            self.advance();
                            state = 37;
                            continue;
                        }
                        Some('6') => {
                            lexema.push('6');
                            self.advance();
                            state = 39;
                            continue;
                        }
                        _ => {
                            todo!("Terra");
                        }
                    }
                }
                37 => {
                    match self.current_char() {
                        Some('2') => {
                            lexema.push('2');
                            self.advance();
                            state = 38;
                            continue;
                        }
                        _ => {
                            todo!("Terra");
                        }
                    }
                }
                38 => {
                    // f32
                    return Token::Float(Tag::FLOAT, self.row(), self.col(), lexema, FloatLiteralType::F32);
                }
                39 => {
                    match self.current_char() {
                        Some('4') => {
                            lexema.push('4');
                            self.advance();
                            state = 40;
                            continue;
                        }
                        _ => {
                            todo!("Terra");
                        }
                    }
                }
                40 => {
                    // f64
                    return Token::Float(Tag::FLOAT, self.row(), self.col(), lexema, FloatLiteralType::F64);
                }
                41 => {
                    self.retract();
                    return Token::Float(Tag::FLOAT, self.row(), self.col(), lexema, FloatLiteralType::F64);
                }
                _ => {
                    return Token::Error(Tag::ERR, self.row(), self.col(), format!("Número mal formado: {}", lexema));
                }
            }
        }
    }
}

/*
(* ===================================================== *)
(* LITERAIS NUMÉRICOS DA LINGUAGEM RUST — EBNF COMPLETA  *)
(* ===================================================== *)

numeric_literal
    = integer_literal
    | float_literal
    ;

(* ===================================================== *)
(* INTEIROS                                               *)
(* ===================================================== *)

integer_literal
    =
      decimal_literal [ integer_suffix ]
    | binary_literal  [ integer_suffix ]
    | octal_literal   [ integer_suffix ]
    | hex_literal     [ integer_suffix ]
    ;

decimal_literal
    =
      "0"
    | nonzero_digit { decimal_digit_or_underscore }
    ;

binary_literal
    =
      "0" ("b" | "B") binary_digit
      { binary_digit_or_underscore }
    ;

octal_literal
    =
      "0" ("o" | "O") octal_digit
      { octal_digit_or_underscore }
    ;

hex_literal
    =
      "0" ("x" | "X") hex_digit
      { hex_digit_or_underscore }
    ;

(* ===================================================== *)
(* FLOATS                                                 *)
(* ===================================================== *)

float_literal
    =
      decimal_float [ float_suffix ]
    ;

decimal_float
    =
        decimal_digits "." [ decimal_digits ] [ exponent_part ]
      | "." decimal_digits [ exponent_part ]
      | decimal_digits exponent_part
    ;

exponent_part
    =
      ("e" | "E")
      [ "+" | "-" ]
      decimal_digits
    ;

(* ===================================================== *)
(* SUFIXOS                                                *)
(* ===================================================== *)

integer_suffix
    =
        "u8"
      | "u16"
      | "u32"
      | "u64"
      | "u128"
      | "usize"
      | "i8"
      | "i16"
      | "i32"
      | "i64"
      | "i128"
      | "isize"
    ;

float_suffix
    =
        "f32"
      | "f64"
    ;

(* ===================================================== *)
(* SEQUÊNCIAS DE DÍGITOS                                  *)
(* ===================================================== *)

decimal_digits
    =
      decimal_digit
      { decimal_digit_or_underscore }
    ;

decimal_digit_or_underscore
    =
        decimal_digit
      | "_"
    ;

binary_digit_or_underscore
    =
        binary_digit
      | "_"
    ;

octal_digit_or_underscore
    =
        octal_digit
      | "_"
    ;

hex_digit_or_underscore
    =
        hex_digit
      | "_"
    ;

(* ===================================================== *)
(* TERMINAIS                                              *)
(* ===================================================== *)

decimal_digit
    =
        "0" | "1" | "2" | "3" | "4"
      | "5" | "6" | "7" | "8" | "9"
    ;

nonzero_digit
    =
        "1" | "2" | "3" | "4" | "5"
      | "6" | "7" | "8" | "9"
    ;

binary_digit
    =
        "0" | "1"
    ;

octal_digit
    =
        "0" | "1" | "2" | "3"
      | "4" | "5" | "6" | "7"
    ;

hex_digit
    =
        decimal_digit
      | "a" | "b" | "c" | "d" | "e" | "f"
      | "A" | "B" | "C" | "D" | "E" | "F"
    ;
*/
