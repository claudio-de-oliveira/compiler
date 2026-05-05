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

#[derive(Debug, Clone, PartialEq)]
pub enum CommentType {
    Line(String),
    Block(String),
    ExpDoc(String),
    IntDoc(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Keyword(Tag, row, col, String),
    Identifier(Tag, row, col, String),
    Character(Tag, row, col, char),
    StringLiteral(Tag, row, col, StringLiteralType, String),
    Number(Tag, row, col, String),
    LPar(Tag, row, col),
    RPar(Tag, row, col),
    EndMark(Tag, row, col),
    Error(Tag, row, col, String),
    DefaultPattern(, row, colTag),
    Division(Tag, row, col),               // /	expr / expr	Arithmetic division	Div
    Not(Tag, row, col),                    // !	!expr	Bitwise or logical complement	Not
    Equality(Tag, row, col, EqualityOp),   // !=	expr != expr	Nonequality comparison	PartialEq
                                           // ==	expr == expr	Equality comparison	PartialEq
    Remainder(Tag, row, col),              // %	expr % expr	Arithmetic remainder	Rem
    Assignment(Tag, row, col),             // =	var = expr, ident = type	Assignment/equivalence
    OpAssignment(Tag, row, col, AssignOp), // %=	var %= expr	Arithmetic remainder and assignment	RemAssign
                                           // &=	var &= expr	Bitwise AND and assignment	BitAndAssign
                                           // *=	var *= expr	Arithmetic multiplication and assignment	MulAssign
                                           // /=	var /= expr	Arithmetic divisionn and assignment	DivAssign
                                           // +=	var += expr	Arithmetic addition and assignment	AddAssign
                                           // -=	var -= expr	Arithmetic subtraction and assignment	SubAssign
                                           // <<=	var <<= expr	Left-shift and assignment	ShlAssign
                                           // >>=	var >>= expr	Right-shift and assignment	ShrAssign
    LogicalAnd(Tag, row, col),             // &&	expr && expr	Short-circuiting logical AND
    LogicalOr(Tag, row, col),              // ||	expr || expr	Short-circuiting logical OR
    BitwiseAnd(Tag, row, col),             // &	expr & expr	Bitwise AND	BitAnd
    BitwiseOr(Tag, row, col),              // |	expr | expr	Bitwise OR	BitOr
    BitwiseXor(Tag, row, col),             // ^	expr ^ expr	Bitwise exclusive OR	BitXor
    BitwiseAndAssign(Tag, row, col),       // &=	expr & expr	Bitwise AND	BitAnd
    BitwiseOrAssign(Tag, row, col),        // |=	var |= expr	Bitwise OR and assignment	BitOrAssign
    BitwiseXorAssign(Tag, row, col),       // ^=	var ^= expr	Bitwise exclusive OR and assignment	BitXorAssign
    InterrogationSymbol(Tag, row, col),    // ?   expr?	Error propagation
    MatchArm(Tag, row, col),               // =>	pat => expr	Part of match arm syntax
    ShiftOp(Tag, row, col, ShiftDir),      // <<	<	expr << expr	Left-shift	Shl
    StarSymbol(Tag, row, col),             // * 	expr * expr	Arithmetic multiplication	Mul
                                           // *	*expr	Dereference	Deref
                                           // *	*const type, *mut type	Raw pointer
    PlusSymbol(Tag, row, col),             // +	trait + trait, 'a + trait	Compound type constraint
                                           // +	expr + expr	Arithmetic addition	Add
    CommaSymbol(Tag, row, col),            // ,   expr, expr	Argument and element separator
    MinusSymbol(Tag, row, col),            // - 	- expr	Arithmetic negation	Neg
                                           // -	expr - expr	Arithmetic subtraction	Sub
    ReturnType(Tag, row, col),             // ->	fn(...) -> type, |…| -> type	Function and closure return type
    SglPtSymbol(Tag, row, col),            // .	expr.ident	Field access
                                           // .	expr.ident(expr, ...)	Method call
                                           // .	expr.0, expr.1, and so on	Tuple indexing
    DblPtSymbol(Tag, row, col),            // ..	.., expr.., ..expr, expr..expr	Right-exclusive range literal	PartialOrd
                                           // ..	..expr	Struct literal update syntax
                                           // ..	variant(x, ..), struct_type { x, .. }	“And the rest” pattern binding
    InclusiveRange(Tag, row, col),         // ..=	..=expr, expr..=expr	Right-inclusive range literal	PartialOrd
    AmpersandSymbol(Tag, row, col),        // &	&expr, &mut expr	Borrow
                                           // &	&type, &mut type, &'a type, &'a mut type	Borrowed pointer type
                                           // &	expr & expr	Bitwise AND	BitAnd
    SemicolonSymbol(Tag, row, col),        // ;	expr;	Statement and item terminator
                                           // ;	[...; len]	Part of fixed-size array syntax
    EqualSymbol(Tag, row, col),            // =	var = expr, ident = type	Assignment/equivalence
    Comparison(Tag, row, col, OrderOp),    // <	expr < expr	Less than comparison	PartialOrd
                                           // >	expr > expr	Greater than comparison	PartialOrd
    VerticalBarSymbol(Tag, row, col),      // |	pat | pat	Pattern alternatives
                                           // |	expr | expr	Bitwise OR	BitOr
    AtSymbol(Tag, row, col),               // @	ident @ pat	Pattern binding
    ColonSymbol(Tag, row, col),            // :	pat: type, ident: type	Constraints
                                           // :	ident: expr	Struct field initializer
                                           // :	'a: loop {...}	Loop label
}

pub trait Scanner {
    fn next_token(&mut self) -> Token;
}

pub struct Rust {
    current_position: usize,
    text: String,
    row: u32,
    col: u32,
}

impl Rust {
    pub fn new(text: &str) -> Self {
        Rust {
            current_position: 0,
            text: text.to_string(),
            row = 0,
            col = 0,
        }
    }

    #[inline]
    fn get_position(&self) -> usize {
        self.current_position
    }

    #[inline]
    fn current_row(&self) -> usize {
        self.row
    }
    #[inline]
    fn current_col(&self) -> usize {
        self.col
    }

    #[inline]
    fn set_position(&mut self, position: usize) {
        self.current_position = position;
    }

    #[inline]
    fn current_char(&self) -> Option<char> {
        self.text.chars().nth(self.current_position)
    }

    /// Avança para o próximo caractere e retorna o caractere atual
    fn advance(&mut self) {
        self.current_position += 1;
        match self.current_char() {
            Some('\n') => {
                row += 1; col = 0;
            }
            Some('\r') => {
                col = 0;
                todo!();
            }
            _ => {
                col += 1;
            }
        }
    }

    /// Volta uma posição com segurança (não vai abaixo de 0)
    fn retract(&mut self) {
        if self.current_position > 0 {
            self.current_position -= 1;
        }
        if col == 0 {
            row -= 1;
        } else {
            col -= 1;
        }
    }

    fn ignore_until_eol(&mut self) -> String {
        // let rest = self.text.get(self.current_position..);
        let rest = self.text.chars().skip(self.current_position).collect()
        let eol = rest.find('\n').unwrap_or(rest.len());
        rest[..eol].to_string()    
    }

    fn ignore_until_eob(&mut self) -> Return<String> {
        let mut state = 0;
        let mut text: String::new();
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
                    }
                }
            }
        }
    }
}

impl Scanner for Rust {
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
                            return Token::Division(Tag::DIV);
                        }
                    }
                }
                1081 => {
                    return Token::OpAssignment(Tag::OPASSIGN, AssignOp::DivAssign);
                }
                1082 => {
                    match self.current_char() {
                        Some('/') => {
                            self.advance();
                            state = 1083
                            continue;
                        }
                        Some('!') => {
                            self.advance();
                            state = 1084;
                            continue;
                        }
                        _  => {
                            let text = self.ignore_rest_of_line();
                            return Token::Comment(Tag::COMMENT, CommentType::Line, text);
                        }
                    }
                }
                1083 => {
                    let text = self.ignore_until_eol();
                    return Token::Comment(Tag::COMMENT, CommentType::ExternalDoc, text);
                }
                1084 => {
                    let text = self.ignore_until_eol();
                    return Token::Comment(Tag::COMMENT, CommentType::InternalDoc, text);
                }
                1085 => {
                    let text = self.ignore_until_eob();
                    return Token::Comment(Tag::COMMENT, CommentType::Block, text);
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
                            return Token::ShiftOp(Tag::SHIFTOP, ShiftDir::Right);
                        }
                    }
                }
                11311 => {
                    return Token::OpAssignment(Tag::OPASSIGN, AssignOp::ShrAssign);
                }

                114 => {
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
                150 => {
                    let char = self.peek_char();

                    if char == None {
                        return Token::Error(Tag::ERR, "Literal de caractere inválido".to_string());
                    }

                    lexema.push(char.unwrap());

                    match self.current_char() {
                        Some('\'') => {
                            self.advance();
                            state = 1599;
                            continue;
                        }
                        _ => {
                            return Token::Error(Tag::ERR, "Literal de caractere inválido".to_string());
                        }
                    }
                }

                1599 => {
                    return Token::Character(Tag::CHARACTER, lexema.chars().nth(0).unwrap_or('\0'));
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
                            return Token::Error(Tag::ERR, "Identificador de byte inválido".to_string());
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
                            return Token::Error(Tag::ERR, "Identificador de byte inválido".to_string());
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
                            return Token::Error(Tag::ERR, "Literal de string inválido".to_string());
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
                            return Token::Error(Tag::ERR, "Literal de string inválido".to_string());
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
                            return Token::Error(Tag::ERR, "Literal de string inválido".to_string());
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
                            return Token::Error(Tag::ERR, "Literal de string inválido".to_string());
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
                            return Token::Error(Tag::ERR, "Literal de caractere inválido".to_string());
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
                            return Token::Error(Tag::ERR, "Literal de caractere inválido".to_string());
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
                            return Token::Error(Tag::ERR, "Literal de caractere inválido".to_string());
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
                            return Token::Error(Tag::ERR, "Literal de caractere inválido".to_string());
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
                            return Token::Error(Tag::ERR, "Literal de caractere inválido".to_string());
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
                            return Token::Error(Tag::ERR, "Literal de caractere inválido".to_string());
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
                            return Token::Error(Tag::ERR, "Literal de caractere inválido".to_string());
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
                            return Token::Error(Tag::ERR, "Literal de caractere inválido".to_string());
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
                        return Token::StringLiteral(Tag::STRING, string_type, lexema);
                    }
                    return Token::Error(Tag::ERR, "Literal de string inválido".to_string());
                }
                1690 => {
                    todo!();
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

impl Rust {
    fn peek_char(&mut self) -> Option<char> {
        let mut state = 0;
        let mut lexema = String::new();
        let mut auxiliary = String::new();
        let position = self.get_position();

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
}
