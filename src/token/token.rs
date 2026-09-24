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

impl Token {
    pub fn get_tag(&self) -> Tag {
        match self {
            Token::Keyword(tag, _, _, _) => *tag,
            Token::Identifier(tag, _, _, _) => *tag,
            Token::Character(tag, _, _, _) => *tag,
            Token::StringLiteral(tag, _, _, _, _) => *tag,
            Token::Integer(tag, _, _, _, _) => *tag,
            Token::Float(tag, _, _, _, _) => *tag,
            Token::LPar(tag, _, _) => *tag,
            Token::RPar(tag, _, _) => *tag,
            Token::EndMark(tag, _, _) => *tag,
            Token::Error(tag, _, _, _) => *tag,
            Token::DefaultPattern(tag, _, _) => *tag,
            Token::Division(tag, _, _) => *tag,
            Token::Not(tag, _, _) => *tag,
            Token::Equality(tag, _, _, _) => *tag,
            Token::Remainder(tag, _, _) => *tag,
            Token::Assignment(tag, _, _) => *tag,
            Token::OpAssignment(tag, _, _, _) => *tag,
            Token::LogicalAnd(tag, _, _) => *tag,
            Token::LogicalOr(tag, _, _) => *tag,
            Token::BitwiseAnd(tag, _, _) => *tag,
            Token::BitwiseOr(tag, _, _) => *tag,
            Token::BitwiseXor(tag, _, _) => *tag,
            Token::BitwiseAndAssign(tag, _, _) => *tag,
            Token::BitwiseOrAssign(tag, _, _) => *tag,
            Token::BitwiseXorAssign(tag, _, _) => *tag,
            Token::InterrogationSymbol(tag, _, _) => *tag,
            Token::MatchArm(tag, _, _) => *tag,
            Token::ShiftOp(tag, _, _, _) => *tag,
            Token::StarSymbol(tag, _, _) => *tag,
            Token::PlusSymbol(tag, _, _) => *tag,
            Token::CommaSymbol(tag, _, _) => *tag,
            Token::MinusSymbol(tag, _, _) => *tag,
            Token::ReturnType(tag, _, _) => *tag,
            Token::SglPtSymbol(tag, _, _) => *tag,
            Token::DblPtSymbol(tag, _, _) => *tag,
            Token::InclusiveRange(tag, _, _) => *tag,
            Token::AmpersandSymbol(tag, _, _) => *tag,
            Token::SemicolonSymbol(tag, _, _) => *tag,
            Token::EqualSymbol(tag, _, _) => *tag,
            Token::Comparison(tag, _, _, _) => *tag,
            Token::VerticalBarSymbol(tag, _, _) => *tag,
            Token::AtSymbol(tag, _, _) => *tag,
            Token::ColonSymbol(tag, _, _) => *tag,
            Token::Comment(tag, _, _, _, _) => *tag,
        }
    }
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
    fn current_char(&self) -> char {
        if self.current_row >= self.text.len() {
            return '\0';
        } 

        if self.current_col >= self.text[self.current_row].chars().count() {
            return '\n';
        } else {
            self.text[self.current_row].chars().nth(self.current_col).unwrap()
        }
    }

    /// Avança para o próximo caractere e retorna o caractere atual
    fn advance(&mut self) {
        self.current_col += 1;
        if self.current_col >= self.text[self.current_row].chars().count() {
            self.current_row += 1;
            self.current_col = 0;
        }
        print!("{}", self.current_char().unwrap());
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
                '\n' => {
                    return comment;
                }
                '\0' => {
                    return comment;
                }
                c => {
                    comment.push(c);
                    self.advance();
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
                        '*' => {
                            self.advance();
                            state = 1;
                            continue;
                        }
                        '/' => {
                            self.advance();
                            state = 3;
                            continue;
                        }
                        '#' => {
                            todo!();
                        }
                        c => {
                            text.push(c);
                            self.advance();
                            state = 0;
                            continue;
                        }
                    }
                }
                1 => {
                    match self.current_char() {
                        '/' if counter == 0 => {
                            self.advance();
                            state = 2;
                            continue;
                        }
                        '/' => {
                            text.push('*');
                            text.push('/');
                            counter -= 1;
                            self.advance();
                            state = 0;
                            continue;
                        }
                        '\0' => {
                            todo!();
                        }
                        c => {
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
                        '*' => {
                            counter += 1;
                            self.advance();
                            state = 0;
                            continue;
                        }
                        '/' => {
                            self.advance();
                            state = 3;
                            continue;
                        }
                        '#' => {
                            todo!();
                        }
                        '\0' => {
                            todo!();
                        }
                        c => {
                            text.push(c);
                            self.advance();
                            state = 0;
                            continue;
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
                        c if c.is_whitespace() => {
                            self.advance();
                            state = 0;
                            continue;
                        }
                        'b' => {
                            lexema.push('b');
                            auxiliary.push('b');
                            self.advance();
                            state = 160;
                            continue;
                        }
                        'r' => {
                            lexema.push('r');
                            auxiliary.push('r');
                            self.advance();
                            state = 161;
                            continue;
                        }
                        c if c.is_alphabetic() => {
                            lexema.push(c);
                            self.advance();
                            state = 10;
                            continue;
                        }
                        '_' => {
                            lexema.push('_');
                            self.advance();
                            state = 10;
                            continue;
                        }
                        c if c.is_digit(10) => {
                            lexema.push(c);
                            self.advance();
                            state = 1;
                            continue;
                        }
                        '(' => {
                            self.advance();
                            state = 7;
                            continue;
                        }
                        ')' => {
                            self.advance();
                            state = 8;
                            continue;
                        }
                        '!' => {
                            self.advance();
                            state = 100;
                            continue;
                        }
                        '%' => {
                            self.advance();
                            state = 101;
                            continue;
                        }
                        '&' => {
                            self.advance();
                            state = 102;
                            continue;
                        }
                        '*' => {
                            self.advance();
                            state = 103;
                            continue;
                        }
                        '+' => {
                            self.advance();
                            state = 104;
                            continue;
                        }
                        ',' => {
                            self.advance();
                            state = 105;
                            continue;
                        }
                        '-' => {
                            self.advance();
                            state = 106;
                            continue;
                        }
                        '.' => {
                            self.advance();
                            state = 107;
                            continue;
                        }
                        '/' => {
                            self.advance();
                            state = 108;
                            continue;
                        }
                        ':' => {
                            self.advance();
                            state = 109;
                            continue;
                        }
                        ';' => {
                            self.advance();
                            state = 110;
                            continue;
                        }
                        '<' => {
                            self.advance();
                            state = 111;
                            continue;
                        }
                        '=' => {
                            self.advance();
                            state = 112;
                            continue;
                        }
                        '>' => {
                            self.advance();
                            state = 113;
                            continue;
                        }
                        '@' => {
                            self.advance();
                            state = 114;
                            continue;
                        }
                        '^' => {
                            self.advance();
                            state = 115;
                            continue;
                        }
                        '|' => {
                            self.advance();
                            state = 116;
                            continue;
                        }
                        '?' => {
                            self.advance();
                            state = 117;
                            continue;
                        }
                        '\'' => {
                            println!("Veio o primeiro apóstrofe");
                            self.advance();
                            state = 150;
                            continue;
                        }
                        '\"' => {
                            println!("Veio a primeira aspas duplas");
                            self.advance();
                            state = 1620;
                            continue;
                        }
                        '#' => {
                            self.advance();
                            state = 9;
                            continue;
                        }
                        '\0' => {
                            todo!();
                        }
                        c => {
                            lexema.push(c);
                            self.advance();
                            state = 999;
                            continue;
                        }
                    }
                }
                1 => {
                    match self.current_char() {
                        c if c.is_digit(10) => {
                            lexema.push(c);
                            self.advance();
                            state = 1;
                            continue;
                        }
                        '_' => {
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
                    self.retract();
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
                        c if c.is_alphanumeric() => {
                            lexema.push(c);
                            self.advance();
                            state = 10;
                            continue;
                        }
                        '_' => {
                            lexema.push('_');
                            self.advance();
                            state = 13;
                            continue;
                        }
                        '!' => {
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
                        '=' => {
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
                        '=' => {
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
                        '=' => {
                            self.advance();
                            state = 1021;
                            continue;
                        }
                        '&' => {
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
                        '=' => {
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
                        '=' => {
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
                        '=' => {
                            self.advance();
                            state = 1061;
                            continue;
                        }
                        '>' => {
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
                        '.' => {
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
                        '=' => {
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
                        '/' => {
                            self.advance();
                            state = 1082;
                            continue;
                        }
                        '*' => {
                            self.advance();
                            state = 1085;
                            continue;
                        }
                        '=' => {
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
                        '/' => {
                            self.advance();
                            state = 1083;
                            continue;
                        }
                        '!' => {
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
                        '=' => {
                            self.advance();
                            state = 1111;
                            continue;
                        }
                        '<' => {
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
                        '=' => {
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
                        '=' => {
                            self.advance();
                            state = 11211;
                            continue;
                        }
                        '>' => {
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
                        '=' => {
                            self.advance();
                            state = 1131;
                            continue;
                        }
                        '>' => {
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
                        '=' => {
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
                        '=' => {
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
                        '=' => {
                            self.advance();
                            state = 1161;
                            continue;
                        }
                        '|' => {
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

                    println!("1: {}", lexema.clone ());

                    match self.current_char() {
                        '\'' => {
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
                        'r' => {  // br
                            lexema.push('r');
                            auxiliary.push('r');
                            self.advance();
                            state = 161;
                            continue;
                        }
                        '"' => {  // bu
                            self.advance();
                            state = 1620;
                            continue;
                        }
                        c if c.is_alphanumeric() => {
                            lexema.push(c);
                            self.advance();
                            state = 10;
                            continue;
                        }
                        '_' => {
                            lexema.push('_');
                            self.advance();
                            state = 13;
                            continue;
                        }
                        '!' => {
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
                        '"' => {
                            self.advance();
                            state = 1620;
                            continue;
                        }
                        c if c.is_alphanumeric() => {
                            lexema.push(c);
                            self.advance();
                            state = 10;
                            continue;
                        }
                        '_' => {
                            lexema.push('_');
                            self.advance();
                            state = 13;
                            continue;
                        }
                        '!' => {
                            lexema.push('!');
                            self.advance();
                            state = 12;
                            continue;
                        }
                        '#' => {  // bu
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
                        '"' => {
                            self.advance();
                            state = 1620;
                            continue;
                        }
                        '#' => {  // bu
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
                        '"' => {  // bu
                            self.advance();
                            state = 1620;
                            continue;
                        }
                        '#' => {  // bu
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
/*
STATE=1636 pos=(0,11) cur='}'          -> advance(); state=1638      (pos vai para 12, 'B')
STATE=1638 pos=(0,12) cur='B'          -> lexema.push('B'); advance(); state=1622   (empurra 'B' — devia estar decodificando o hex, não copiando o próximo char)
STATE=1622 pos=(0,13) cur='C'          -> retract() volta pra pos 12 ('B' de novo!) -> lexema.push('B') OUTRA VEZ -> lexema="A\1f980BB"
STATE=1624                              -> processa 'C', depois '\'
STATE=1625 cur='n'                      -> lexema.push('\n')  (aqui SEMPRE decodifica para quebra de linha real — o código não faz distinção por tipo neste ramo) ; advance(); state=1650   (pos agora é a aspa de fechamento)
STATE=1650 cur='"'                      -> lexema.push('"')  (empurra a ASPA DE FECHAMENTO como se fosse conteúdo!) ; advance()  -> passa do fim da única linha: current_row vira 1
STATE=1622                              -> retract() -> current_col vira o tamanho da linha (posição "fantasma" de fim-de-linha, current_char() = '\n' virtual) -> empurra esse '\n' virtual também; advance() -> current_row vira 1 de novo
STATE=1624 cur='\0'                     -> current_char() já está retornando o sentinela de "fim do texto" ('\0'), mas nenhum estado trata '\0' como parada — cai no ramo genérico, empurra '\0' e chama advance() mais uma vez
 */                
                1622 => {
                    self.retract();
                    match self.current_char() {
                        '\"' => {
                            self.advance();
                            state = 1680;
                            continue;
                        }
                        //Some('\\') if string_type == StringLiteralType::Standard || string_type == StringLiteralType::ByteString => {
                        '\\' if matches!(string_type, StringLiteralType::Standard | StringLiteralType::ByteString) => {
                            lexema.push('\\');
                            self.advance();
                            state = 1623;
                            continue;
                        }
                        '\\' => {
                            lexema.push('\\');
                            self.advance();
                            state = 1625;
                            continue;
                        }
                        c => {
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
                        c => {
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

                1624 => {
                    match self.current_char() {
                        '"' => {
                            self.advance();
                            state = 1680;
                            continue;
                        }
                        '\\' if matches!(string_type, StringLiteralType::Standard | StringLiteralType::ByteString) => {
                            lexema.push('\\');
                            self.advance();
                            state = 1623;
                            continue;
                        }
                        /*
                        Some('\\') if string_type == StringLiteralType::Standard || string_type == StringLiteralType::ByteString => {
                            // se for ByteString não pode ter \u
                            lexema.push('\\');
                            self.advance();
                            state = 1623;
                            continue;
                        }
                        */
                        '\\' => {
                            lexema.push('\\');
                            self.advance();
                            state = 1625;
                            continue;
                        }
                        c => {
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
                        'u' => {  // \u{...}
                            lexema.pop(); // remove o '\' empilhado pelo chamador; \u{...} vira 1 char, não texto
                            self.advance();
                            state = 1630;
                            continue;
                        }
                        /*
                        'u' => {  // \u{...}
                            self.advance();
                            state = 1630;
                            continue;
                        }
                        */
                        'n' => { 
                            lexema.push(if string_type == StringLiteralType::ByteString { 'n' } else { '\n' }); 
                            self.advance(); 
                            state = 1650; 
                            continue; 
                        }
                        'r' => { 
                            lexema.push(if string_type == StringLiteralType::ByteString { 'r' } else { '\r' }); 
                            self.advance(); 
                            state = 1650; 
                            continue; 
                        }
                        't' => { 
                            lexema.push(if string_type == StringLiteralType::ByteString { 't' } else { '\t' }); 
                            self.advance(); 
                            state = 1650; 
                            continue; 
                        }
                        /*
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
                        */
                        '\\' => {  // \\
                            lexema.push('\\');
                            self.advance();
                            state = 1650;
                            continue;
                        }
                        '\'' => {  // \'
                            lexema.push('\'');
                            self.advance();
                            state = 1650;
                            continue;
                        }
                        '"' => {  // \"
                            lexema.push('"');
                            self.advance();
                            state = 1650;
                            continue;
                        }
                        c => {
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

                1630 => {
                    match self.current_char() {
                        /*
                        '{' => {
                            self.advance();
                            state = 1631;
                            continue;
                        }
                        */
                        '{' => { 
                            auxiliary.clear(); 
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
                        c if c.is_digit(16) => {  // primeiro
                            /*lexema.push(c);*/
                            auxiliary.push(c);
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
                        c if c.is_digit(16) => {  // segundo
                            /*lexema.push(c);*/
                            auxiliary.push(c);
                            self.advance();
                            state = 1633;
                            continue;
                        }
                        '}' => {
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
                        c if c.is_digit(16) => {  // terceiro
                            /*lexema.push(c);*/
                            auxiliary.push(c);
                            self.advance();
                            state = 1634;
                            continue;
                        }
                        '}' => {
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
                        c if c.is_digit(16) => {  // quarto
                            /*lexema.push(c);*/
                            auxiliary.push(c);
                            self.advance();
                            state = 1635;
                            continue;
                        }
                        '}' => {
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
                        c if c.is_digit(16) => {  // quinto
                            /*lexema.push(c);*/
                            auxiliary.push(c);
                            self.advance();
                            state = 1636;
                            continue;
                        }
                        '}' => {
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
                        c if c.is_digit(16) => {  // sexto
                            /*lexema.push(c);*/
                            auxiliary.push(c);
                            self.advance();
                            state = 1637;
                            continue;
                        }
                        '}' => {
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
                        '}' => {
                            self.advance();
                            state = 1638;
                            continue;
                        }
                        _ => {
                            return Token::Error(Tag::ERR, self.row(), self.col(), "Literal de caractere inválido".to_string());
                        }
                    }
                }
                /*
                1638 => {
                    lexema.push(self.current_char().unwrap());
                    self.advance();
                    state = 1622;
                    continue;
                }
                */
                1638 => {
                    if let Some(ch) = std::char::from_u32(u32::from_str_radix(&auxiliary, 16).unwrap_or(0)) {
                        lexema.push(ch);
                    }
                    auxiliary.clear();
                    state = 1624;
                    continue;
                }

                1650 => {
                    //lexema.push(self.current_char().unwrap());
                    //self.advance();
                    state = 1624;
                    continue;
                }
                1660 => {
                    //lexema.push(self.current_char().unwrap());
                    //self.advance();
                    state = 1624;
                    continue;
                }

                1680 => {
                    match self.current_char() {
                        '#' => {
                            counter -= 1;
                            self.advance();
                            state = 1680;
                            continue;
                        }
                        _ => {
                            // self.advance();
                            state = 1681;
                            continue;
                        }
                    }
                }
                1681 => {
                    self.retract();
                    if counter == 0 {
                        return match string_type {
                            StringLiteralType::Standard => {
                                Token::StringLiteral(Tag::STRING, self.row(), self.col(), string_type, lexema)
                            },
                            StringLiteralType::ByteString => {
                                println!("{}", lexema);
                                Token::StringLiteral(Tag::STRING, self.row(), self.col(), string_type, String::from_utf8_lossy(lexema.as_bytes()).into_owned())
                            },
                            StringLiteralType::Raw(_) => {
                                Token::StringLiteral(Tag::STRING, self.row(), self.col(), string_type, lexema)
                            },
                            StringLiteralType::RawByte(_) => {
                                Token::StringLiteral(Tag::STRING, self.row(), self.col(), string_type, lexema)
                            }
                        }
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
        // let mut position = self.get_position();

        println!("Entrei no peek_char");

        loop {
            match state {
                0 => {
                    match self.current_char() {
                        '\\' => {
                            println!("Veio o caractere: '{}'", '\\');
                            auxiliary.push('\\');
                            self.advance();
                            state = 1;
                            continue;
                        }
                        c => {
                            println!("Veio o caractere: '{}'", c);
                            auxiliary.push(c);
                            lexema.push(c);
                            self.advance();
                            state = 12;
                            continue;
                        }
                        _ => {
                            // self.set_position(position);
                            return None;
                        }
                    }
                }
                1 => {
                    match self.current_char() {
                        'u' => {  // \u{...}
                            println!("Veio o caractere: '{}'", 'u');
                            auxiliary.push('u');
                            self.advance();
                            state = 2;
                            continue;
                        }
                        'n' => {  // \n
                            auxiliary.push('n');
                            lexema.push('\n');
                            self.advance();
                            state = 12;
                            continue;
                        }
                        'r' => {  // \r
                            auxiliary.push('r');
                            lexema.push('\r');
                            self.advance();
                            state = 12;
                            continue;
                        }
                        't' => {  // \t
                            auxiliary.push('t');
                            lexema.push('\t');
                            self.advance();
                            state = 12;
                            continue;
                        }
                        '\\' => {  // \\
                            auxiliary.push('\\');
                            lexema.push('\\');
                            self.advance();
                            state = 12;
                            continue;
                        }
                        '\'' => {  // \'
                            auxiliary.push('\'');
                            lexema.push('\'');
                            self.advance();
                            state = 12;
                            continue;
                        }
                        '\"' => {  // \"
                            auxiliary.push('\"');
                            lexema.push('\"');
                            self.advance();
                            state = 12;
                            continue;
                        }
                        _ => {
                            // self.set_position(position);
                            return None;
                        }
                    }
                }
                2 => {
                    match self.current_char() {
                        '{' => {
                            println!("Veio o caractere: '{}'", '{');
                            auxiliary.push('{');
                            self.advance();
                            state = 3;
                            continue;
                        }
                        _ => {
                            // self.set_position(position);
                                                        println!("Ops");
                            return None;
                        }
                    }
                }
                3 => {
                    match self.current_char() {
                        c if c.is_digit(16) => {  // primeiro
                            println!("Veio o caractere: '{}'", c);
                            auxiliary.push(c);
                            lexema.push(c);
                            self.advance();
                            state = 4;
                            continue;
                        }
                        _ => {
                            // self.set_position(position);
                            return None;
                        }
                    }
                }
                4 => {
                    match self.current_char() {
                        c if c.is_digit(16) => {  // segundo
                            println!("Veio o caractere: '{}'", c);
                            auxiliary.push(c);
                            lexema.push(c);
                            self.advance();
                            state = 5;
                            continue;
                        }
                        '}' => {
                            println!("Veio o caractere: '{}'", '}');
                            auxiliary.push('}');
                            self.advance();
                            state = 10;
                            continue;
                        }
                        _ => {
                            // self.set_position(position);
                            return None;
                        }
                    }
                }
                5 => {
                    match self.current_char() {
                        c if c.is_digit(16) => {  // terceiro
                            println!("Veio o caractere: '{}'", c);
                            auxiliary.push(c);
                            lexema.push(c);
                            self.advance();
                            state = 6;
                            continue;
                        }
                        '}' => {
                            println!("Veio o caractere: '{}'", '}');
                            auxiliary.push('}');
                            self.advance();
                            state = 10;
                            continue;
                        }
                        _ => {
                            // self.set_position(position);
                            return None;
                        }
                    }
                }
                6 => {
                    match self.current_char() {
                        c if c.is_digit(16) => {  // quarto
                            println!("Veio o caractere: '{}'", c);
                            auxiliary.push(c);
                            lexema.push(c);
                            self.advance();
                            state = 7;
                            continue;
                        }
                        '}' => {
                            println!("Veio o caractere: '{}'", '}');
                            auxiliary.push('}');
                            self.advance();
                            state = 10;
                            continue;
                        }
                        _ => {
                            // self.set_position(position);
                            return None;
                        }
                    }
                }
                7 => {
                    match self.current_char() {
                        c if c.is_digit(16) => {  // quinto
                            println!("Veio o caractere: '{}'", c);
                            auxiliary.push(c);
                            lexema.push(c);
                            self.advance();
                            state = 8;
                            continue;
                        }
                        '}' => {
                            println!("Veio o caractere: '{}'", '}');
                            auxiliary.push('}');
                            self.advance();
                            state = 10;
                            continue;
                        }
                        _ => {
                            // self.set_position(position);
                            return None;
                        }
                    }
                }
                8 => {
                    match self.current_char() {
                        c if c.is_digit(16) => {  // sexto
                            println!("Veio o caractere: '{}'", c);
                            auxiliary.push(c);
                            lexema.push(c);
                            self.advance();
                            state = 9;
                            continue;
                        }
                        '}' => {
                            println!("Veio o caractere: '{}'", '}');
                            auxiliary.push('}');
                            self.advance();
                            state = 10;
                            continue;
                        }
                        _ => {
                            // self.set_position(position);
                            return None;
                        }
                    }
                }
                9 => {
                    match self.current_char() {
                        '}' => {
                            println!("Veio o caractere: '{}'", '}');
                            auxiliary.push('}');
                            self.advance();
                            state = 10;
                            continue;
                        }
                        _ => {
                            // self.set_position(position);
                            return None;
                        }
                    }
                }
                10 => {
                    println!("Çexema: '{}'", lexema);
                    return std::char::from_u32(u32::from_str_radix(&lexema, 16).unwrap_or(0));
                }
                12 => {
                    return lexema.chars().nth(0);
                }
                _ => {
                    // self.set_position(position);
                    return None;
                }
            }
        }
    }

    pub fn peek_number(&mut self) -> Token {
        let mut state = 0;
        let mut lexema = String::new();
        let mut signed = true;
        let current_row = self.row();
        let current_col = self.col();

        loop {
            match state {
                0 => {  
                    match self.current_char() {
                        '0' => {
                            lexema.push('0');
                            self.advance();
                            state = 1;
                            continue;
                        }
                        c if c.is_digit(10) => {
                            lexema.push(c);
                            self.advance();
                            state = 25;
                            continue;
                        }
                        // Conferir se pode começar com um ponto
                        // Some('.') => {
                        //     lexema.push('0');
                        //     self.advance();
                        //     state = 7;
                        //     continue;
                        // }
                        _ => {
                            unreachable!();
                        }
                    }
                }
                1 => {
                    match self.current_char() {
                        '.' => {
                            lexema.push('.');
                            self.advance();
                            state = 7;
                            continue;
                        }
                        'e' | 'E' => {
                            lexema.push('e');
                            self.advance();
                            state = 32;
                            continue;
                        }
                        'i' => {
                            signed = true;
                            self.advance();
                            state = 9;
                            continue;
                        }
                        'u' => {
                            signed = false;
                            self.advance();
                            state = 9;
                            continue;
                        }
                        'f' => {
                            self.advance();
                            state = 36;
                            continue;
                        }
                        'b' | 'B' => {
                            lexema.clear();
                            self.advance();
                            state = 3;
                            continue;
                        }
                        'o' | 'O' => {
                            lexema.clear();
                            self.advance();
                            state = 4;
                            continue;
                        }
                        'x' | 'X' => {
                            lexema.clear();
                            self.advance();
                            state = 5;
                            continue;
                        }
                        c if c.is_digit(10) => {
                            lexema.push(c);
                            self.advance();
                            state = 25;
                            continue;
                        }
                        '_' => {
                            self.advance();
                            state = 25;
                            continue;
                        }
                        _ => {
                            // self.advance();
                            state = 26;
                            continue;
                        }
                    }
                }
                2 => {
                    unreachable!();
                }
                3 => {
                    match self.current_char() {
                        c if c.is_digit(2) => {
                            lexema.push(c);
                            self.advance();
                            state = 8;
                            continue;
                        }
                        '_' => {
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
                        c if c.is_digit(8) => {
                            lexema.push(c);
                            self.advance();
                            state = 23;
                            continue;
                        }
                        '_' => {
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
                        c if c.is_digit(16) => {
                            lexema.push(c);
                            self.advance();
                            state = 24;
                            continue;
                        }
                        '_' => {
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
                    unreachable!();
                }
                7 => {
                    match self.current_char() {
                        c if c.is_digit(10) => {
                            lexema.push(c);
                            self.advance();
                            state = 35;
                            continue;
                        }
                        'f' => {
                            lexema.push('f');
                            self.advance();
                            state = 36;
                            continue;
                        }
                        '_' => {
                            self.advance();
                            state = 35;
                            continue;
                        }
                        _ => {
                            lexema.push('f');
                            lexema.push('6');
                            lexema.push('4');
                            state = 40;
                            continue;
                        }
                    }
                }
                8 => {
                    match self.current_char() {
                        '_' => {
                            self.advance();
                            state = 8;
                            continue;
                        }
                        c if c.is_digit(2) => {
                            lexema.push(c);
                            self.advance();
                            state = 8;
                            continue;
                        }
                        'i' => {
                            signed = true;
                            self.advance();
                            state = 9;
                            continue;
                        }
                        'u' => {
                            signed = false;
                            self.advance();
                            state = 9;
                            continue;
                        }
                        // Some('\0') => {
                        //     state = 27;
                        //     continue;
                        // }
                        _ => {
                            state = 27;
                            // self.advance();
                            continue;
                        }
                    }
                }
                9 => {
                    match self.current_char() {
                        '1' => {
                            self.advance();
                            state = 11;
                            continue;
                        }
                        '3' => {
                            self.advance();
                            state = 15;
                            continue;
                        }
                        '6' => {
                            self.advance();
                            state = 17;
                            continue;
                        }
                        '8' => {
                            self.advance();
                            state = 10;
                            continue;
                        }
                        's' => {
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
                        return Token::Integer(Tag::INTEGER, current_row, current_col, lexema, IntegerLiteralType::I8);
                    } else {
                        return Token::Integer(Tag::INTEGER, current_row, current_col, lexema, IntegerLiteralType::U8);
                    }
                }
                11 => {
                    match self.current_char() {
                        '2' => {
                            self.advance();
                            state = 13;
                            continue;
                        }
                        '6' => {
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
                        return Token::Integer(Tag::INTEGER, current_row, current_col, lexema, IntegerLiteralType::I16);
                    } else {
                        return Token::Integer(Tag::INTEGER, current_row, current_col, lexema, IntegerLiteralType::U16);
                    }
                }
                13 => {
                    match self.current_char() {
                        '8' => {
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
                        return Token::Integer(Tag::INTEGER, current_row, current_col, lexema, IntegerLiteralType::I128);
                    } else {
                        return Token::Integer(Tag::INTEGER, current_row, current_col, lexema, IntegerLiteralType::U128);
                    }
                }
                15 => {
                    match self.current_char() {
                        '2' => {
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
                        return Token::Integer(Tag::INTEGER, current_row, current_col, lexema, IntegerLiteralType::I32);
                    } else {
                        return Token::Integer(Tag::INTEGER, current_row, current_col, lexema, IntegerLiteralType::U32);
                    }
                }
                17 => {
                    match self.current_char() {
                        '4' => {
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
                        return Token::Integer(Tag::INTEGER, current_row, current_col, lexema, IntegerLiteralType::I64);
                    } else {
                        return Token::Integer(Tag::INTEGER, current_row, current_col, lexema, IntegerLiteralType::U64);
                    }
                }
                19 => {
                    match self.current_char() {
                        'i' => {
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
                        'z' => {
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
                        'e' => {
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
                        return Token::Integer(Tag::INTEGER, current_row, current_col, lexema, IntegerLiteralType::ISIZE);
                    } else {
                        return Token::Integer(Tag::INTEGER, current_row, current_col, lexema, IntegerLiteralType::USIZE);
                    }
                }
                23 => {
                    match self.current_char() {
                        c if c.is_digit(8) => {
                            lexema.push(c);
                            self.advance();
                            state = 23;
                            continue;
                        }
                        '_' => {
                            self.advance();
                            state = 23;
                            continue;
                        }
                        'i' => {
                            signed = true;
                            self.advance();
                            state = 9;
                            continue;
                        }
                        'u' => {
                            signed = false;
                            self.advance();
                            state = 9;
                            continue;
                        }
                        _ => {
                            // self.advance();
                            state = 28;
                            continue;
                        }
                    }
                }
                24 => {
                    match self.current_char() {
                        c if c.is_digit(16) => {
                            lexema.push(c);
                            self.advance();
                            state = 24;
                            continue;
                        }
                        '_' => {
                            self.advance();
                            state = 24;
                            continue;
                        }
                        'i' => {
                            signed = true;
                            self.advance();
                            state = 9;
                            continue;
                        }
                        'u' => {
                            signed = false;
                            self.advance();
                            state = 9;
                            continue;
                        }
                        _ => {
                            // self.advance();
                            state = 29;
                            continue;
                        }
                    }
                }
                25 => {
                    match self.current_char() {
                        '.' => {
                            lexema.push('.');
                            self.advance();
                            state = 35;
                            continue;
                        }
                        c if c.is_digit(10) => {
                            lexema.push(c);
                            self.advance();
                            state = 25;
                            continue;
                        }
                        '_' => {
                            self.advance();
                            state = 25;
                            continue;
                        }
                        'e' | 'E' => {
                            lexema.push('e');
                            self.advance();
                            state = 32;
                            continue;
                        }
                        'i' => {
                            signed = true;
                            self.advance();
                            state = 9;
                            continue;
                        }
                        'u' => {
                            signed = false;
                            self.advance();
                            state = 9;
                            continue;
                        }
                        _ => {
                            // self.advance();
                            state = 30;
                            continue;
                        }
                    }
                }
                26 => {
                    self.retract();
                    return Token::Integer(Tag::INTEGER, current_row, current_col, lexema, IntegerLiteralType::ISIZE);
                }
                27 => {
                    self.retract();
                    //todo!("Converter binário para inteiro");
                    return Token::Integer(Tag::INTEGER, current_row, current_col, lexema, IntegerLiteralType::ISIZE);
                }
                28 => {
                    self.retract();
                    //todo!("Converter octal para inteiro");
                    return Token::Integer(Tag::INTEGER, current_row, current_col, lexema, IntegerLiteralType::ISIZE);
                }
                29 => {
                    self.retract();
                    //todo!("Converter hexa para inteiro");
                    return Token::Integer(Tag::INTEGER, current_row, current_col, lexema, IntegerLiteralType::ISIZE);
                }
                30 => {
                    self.retract();
                    //todo!("Converter decimal para inteiro");
                    return Token::Integer(Tag::INTEGER, current_row, current_col, lexema, IntegerLiteralType::ISIZE);
                }
                31 => {
                    self.retract();
                    return Token::Integer(Tag::INTEGER, current_row, current_col, lexema, IntegerLiteralType::ISIZE);
                }
                32 => {
                    match self.current_char() {
                        c if c.is_digit(10) => {
                            lexema.push(c);
                            self.advance();
                            state = 34;
                            continue;
                        }
                        '+' => {
                            lexema.push('+');
                            self.advance();
                            state = 33;
                            continue;
                        }
                        '-' => {
                            lexema.push('-');
                            self.advance();
                            state = 33;
                            continue;
                        }
                        _ => {
                            todo!("Terra");
                        }
                    }
                }
                33 => {
                    match self.current_char() {
                        c if c.is_digit(10) => {
                            lexema.push(c);
                            self.advance();
                            state = 34;
                            continue;
                        }
                        '_' => {
                            self.advance();
                            state = 34;
                            continue;
                        }
                        _ => {
                            todo!("Terra");
                        }
                    }
                }
                34 => {
                    match self.current_char() {
                        c if c.is_digit(10)  => {
                            lexema.push(c);
                            self.advance();
                            state = 34;
                            continue;
                        }
                        '_' => {
                            self.advance();
                            state = 34;
                            continue;
                        }
                        'f' => {
                            lexema.push('f');
                            self.advance();
                            state = 36;
                            continue;
                        }
                        _ => {
                            // self.advance();
                            state = 41;
                            continue;
                        }
                    }
                }
                35 => {
                    match self.current_char() {
                        c if c.is_digit(10) => {
                            lexema.push(c);
                            self.advance();
                            state = 35;
                            continue;
                        }
                        '_' => {
                            self.advance();
                            state = 35;
                            continue;
                        }
                        'f' => {
                            lexema.push('f');
                            self.advance();
                            state = 36;
                            continue;
                        }
                        'e' | 'E' => {
                            lexema.push('e');
                            self.advance();
                            state = 32;
                            continue;
                        }
                        _ => {
                            // self.advance();
                            state = 41;
                            continue;
                        }
                    }
                }
                36 => {
                    match self.current_char() {
                        '3' => {
                            lexema.push('3');
                            self.advance();
                            state = 37;
                            continue;
                        }
                        '6' => {
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
                        '2' => {
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
                    return Token::Float(Tag::FLOAT, current_row, current_col, lexema, FloatLiteralType::F32);
                }
                39 => {
                    match self.current_char() {
                        '4' => {
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
                    return Token::Float(Tag::FLOAT, current_row, current_col, lexema, FloatLiteralType::F64);
                }
                41 => {
                    self.retract();
                    return Token::Float(Tag::FLOAT, current_row, current_col, lexema, FloatLiteralType::F64);
                }
                42 => {
                    match self.current_char() {
                        'f' => {
                            self.advance();
                            state = 36;
                            continue;
                        }
                        _ => {
                            // self.advance();
                            state = 40;
                            continue;
                        }
                    }
                }

                _ => {
                    return Token::Error(Tag::ERR, current_row, current_col, format!("Número mal formado: {}", lexema));
                }
            }
        }
    }


    fn row_string(&mut self, n_sharp: usize) -> Token {
        let mut state = 0;
        let mut lexema = String::new();

        loop {
            match state {
                0 => {
                    match self.current_char() {
                        c  if c != '\"' => {
                            lexema.push(c);
                            self.advance();
                            state = 0;
                            continue;
                        }
                        '\\' => {
                            self.advance();
                            state = 1;
                            continue;
                        }
                        '\"' => {
                            self.advance();
                            state = 4;
                            continue;
                        }
                        _ => {
                            todo!();
                        }
                    }
                }
                1 => {
                    match self.current_char() {
                        'n' => {
                            lexema.push('\n');
                            self.advance();
                            state = 0;
                            continue;
                        }
                        't' => {
                            lexema.push('\t');
                            self.advance();
                            state = 0;
                            continue;
                        }
                        'r' => {
                            lexema.push('\r');
                            self.advance();
                            state = 0;
                            continue;
                        }
                        '\\' => {
                            lexema.push('\\');
                            self.advance();
                            state = 0;
                            continue;
                        }
                        '\"' => {
                            lexema.push('\"');
                            self.advance();
                            state = 0;
                            continue;
                        }
                        '\'' => {
                            lexema.push('\'');
                            self.advance();
                            state = 0;
                            continue;
                        }
                        'u' => {
                            self.advance();
                            state = 2;
                            continue;
                        }
                        'x' => {
                            self.advance();
                            state = 10;
                            continue;
                        }
                        _ => {
                            todo!();
                        }
                    }
                }
                2 => {
                    match self.current_char() {
                        '{' => {
                            self.advance();
                            state = 3;
                            continue;
                        }
                        _ => {
                            todo!();
                        }
                    }
                }
                3 => {
                    // q8
                }
                10 => {
                    // q7 - lendo primeiro hexadecimai
                    match self.current_char() {
                        c  if c.is_ascii_digit() => {
                            self.advance();
                            state = 11;
                            continue;
                        }
                        _ => {
                            todo!();
                        }
                    }
                }
                11 => {
                    // q7 - lendo segundo hexadecimai
                    match self.current_char() {
                        c  if c.is_ascii_digit() => {
                            self.advance();
                            state = 0;
                            continue;
                        }
                        _ => {
                            todo!();
                        }
                    }
                }
                _ => {
                    todo!();
                }
            }
        }
    }

    fn row_string_counter(&mut self, n_sharp: usize) -> Token {
        let mut state = 0;
        let mut lexema = String::new();
        let mut k_sharp: usize = 0;
        let current_row = self.row();
        let current_col = self.col();

        loop {
            match state {
                0 => {
                    match self.current_char() {
                        c  if c != '\"' => {
                            lexema.push(c);
                            self.advance();
                            state = 0;
                            continue;
                        }
                        '\"' => {
                            self.advance();
                            state = 1;
                            k_sharp = n_sharp;
                            continue;
                        }
                        _ => {
                            todo!();
                        }
                    }
                }
                1 => {
                    match self.current_char() {
                        '#' => {
                            self.advance();
                            k_sharp -= 1;
                            if k_sharp == 0 {
                                state = 2;
                            }
                            else{
                                state = 1;
                            }
                            continue;
                        }
                        _ => {
                            lexema.extend(std::iter::repeat('#').take(n_sharp - k_sharp));
                            self.advance();
                            state = 0;
                            continue;
                        }
                    }
                }
                2 => {
                    return Token::StringLiteral(Tag::STRING, current_row, current_col, StringLiteralType::Raw(n_sharp), lexema);
                }
            }
        }
        
    }

    fn string_prefix(&mut self) -> Token {
        let mut state = 0;
        let mut n_char = 0;
        let mut n_sharp= 0;
        let mut lexema = String::new();

        loop {
            match state {
                0 => {
                    match self.current_char() {
                        'r' => {
                            lexema.push('r');
                            self.advance();
                            n_char += 1;
                            state = 2;
                            continue;
                        }
                        'b' => {
                            lexema.push('b');
                            self.advance();
                            n_char += 1;
                            state = 1;
                            continue;
                        }
                        '\"' => {
                            lexema.push('b');
                            self.advance();
                            n_char += 1;
                            state = 4;
                            continue;
                        }
                        _ => {
                            todo!();
                        }
                    }
                }
                1 => {
                    match self.current_char() {
                        'r' => {
                            lexema.push('r');
                            self.advance();
                            n_char += 1;
                            state = 2;
                            continue;
                        }
                        '\"' => {
                            lexema.push('b');
                            self.advance();
                            n_char += 1;
                            state = 4;
                            continue;
                        }
                        _ => {
                            todo!();
                        }
                    }
                }
                2 => {
                    match self.current_char() {
                        '\"' => {
                            lexema.push('b');
                            self.advance();
                            n_char += 1;
                            state = 5;
                            continue;
                        }
                        '#' => {
                            lexema.push('b');
                            self.advance();
                            n_char += 1;
                            n_sharp += 1;
                            state = 3;
                            continue;
                        }
                        _ => {
                            todo!();
                        }
                    }
                }
                3 => {
                    match self.current_char() {
                        '\"' => {
                            lexema.push('b');
                            self.advance();
                            n_char += 1;
                            state = 5;
                            continue;
                        }
                        '#' => {
                            lexema.push('b');
                            self.advance();
                            n_char += 1;
                            n_sharp += 1;
                            state = 3;
                            continue;
                        }
                        _ => {
                            todo!();
                        }
                    }
                }
                5 => {

                }
                _ => {
                    todo!();
                }
            }
        }

    }

    pub fn peek_string(&mut self) -> Token {
        let mut state = 0;
        let mut lexema = String::new();
        let mut signed = true;
        let current_row = self.row();
        let current_col = self.col();

        loop {
            match state {
                0 => {  
                    match self.current_char() {
                        '0' => {
                            lexema.push('0');
                            self.advance();
                            state = 1;
                            continue;
                        }
                        c if c.is_digit(10) => {
                            lexema.push(c);
                            self.advance();
                            state = 25;
                            continue;
                        }
                        // Conferir se pode começar com um ponto
                        // Some('.') => {
                        //     lexema.push('0');
                        //     self.advance();
                        //     state = 7;
                        //     continue;
                        // }
                        _ => {
                            unreachable!();
                        }
                    }
                }
                1 => {
                    match self.current_char() {
                        '.' => {
                            lexema.push('.');
                            self.advance();
                            state = 7;
                            continue;
                        }
                        'e' | 'E' => {
                            lexema.push('e');
                            self.advance();
                            state = 32;
                            continue;
                        }
                        'i' => {
                            signed = true;
                            self.advance();
                            state = 9;
                            continue;
                        }
                        'u' => {
                            signed = false;
                            self.advance();
                            state = 9;
                            continue;
                        }
                        'f' => {
                            self.advance();
                            state = 36;
                            continue;
                        }
                        'b' | 'B' => {
                            lexema.clear();
                            self.advance();
                            state = 3;
                            continue;
                        }
                        'o' | 'O' => {
                            lexema.clear();
                            self.advance();
                            state = 4;
                            continue;
                        }
                        'x' | 'X' => {
                            lexema.clear();
                            self.advance();
                            state = 5;
                            continue;
                        }
                        c if c.is_digit(10) => {
                            lexema.push(c);
                            self.advance();
                            state = 25;
                            continue;
                        }
                        '_' => {
                            self.advance();
                            state = 25;
                            continue;
                        }
                        _ => {
                            // self.advance();
                            state = 26;
                            continue;
                        }
                    }
                }
                2 => {
                    unreachable!();
                }
                3 => {
                    match self.current_char() {
                        c if c.is_digit(2) => {
                            lexema.push(c);
                            self.advance();
                            state = 8;
                            continue;
                        }
                        '_' => {
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
                        c if c.is_digit(8) => {
                            lexema.push(c);
                            self.advance();
                            state = 23;
                            continue;
                        }
                        '_' => {
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
                        c if c.is_digit(16) => {
                            lexema.push(c);
                            self.advance();
                            state = 24;
                            continue;
                        }
                        '_' => {
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
                    unreachable!();
                }
                7 => {
                    match self.current_char() {
                        c if c.is_digit(10) => {
                            lexema.push(c);
                            self.advance();
                            state = 35;
                            continue;
                        }
                        'f' => {
                            lexema.push('f');
                            self.advance();
                            state = 36;
                            continue;
                        }
                        '_' => {
                            self.advance();
                            state = 35;
                            continue;
                        }
                        _ => {
                            lexema.push('f');
                            lexema.push('6');
                            lexema.push('4');
                            state = 40;
                            continue;
                        }
                    }
                }
                8 => {
                    match self.current_char() {
                        '_' => {
                            self.advance();
                            state = 8;
                            continue;
                        }
                        c if c.is_digit(2) => {
                            lexema.push(c);
                            self.advance();
                            state = 8;
                            continue;
                        }
                        'i' => {
                            signed = true;
                            self.advance();
                            state = 9;
                            continue;
                        }
                        'u' => {
                            signed = false;
                            self.advance();
                            state = 9;
                            continue;
                        }
                        // Some('\0') => {
                        //     state = 27;
                        //     continue;
                        // }
                        _ => {
                            state = 27;
                            // self.advance();
                            continue;
                        }
                    }
                }
                9 => {
                    match self.current_char() {
                        '1' => {
                            self.advance();
                            state = 11;
                            continue;
                        }
                        '3' => {
                            self.advance();
                            state = 15;
                            continue;
                        }
                        '6' => {
                            self.advance();
                            state = 17;
                            continue;
                        }
                        '8' => {
                            self.advance();
                            state = 10;
                            continue;
                        }
                        's' => {
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
                        return Token::Integer(Tag::INTEGER, current_row, current_col, lexema, IntegerLiteralType::I8);
                    } else {
                        return Token::Integer(Tag::INTEGER, current_row, current_col, lexema, IntegerLiteralType::U8);
                    }
                }
                11 => {
                    match self.current_char() {
                        '2' => {
                            self.advance();
                            state = 13;
                            continue;
                        }
                        '6' => {
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
                        return Token::Integer(Tag::INTEGER, current_row, current_col, lexema, IntegerLiteralType::I16);
                    } else {
                        return Token::Integer(Tag::INTEGER, current_row, current_col, lexema, IntegerLiteralType::U16);
                    }
                }
                13 => {
                    match self.current_char() {
                        '8' => {
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
                        return Token::Integer(Tag::INTEGER, current_row, current_col, lexema, IntegerLiteralType::I128);
                    } else {
                        return Token::Integer(Tag::INTEGER, current_row, current_col, lexema, IntegerLiteralType::U128);
                    }
                }
                15 => {
                    match self.current_char() {
                        '2' => {
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
                        return Token::Integer(Tag::INTEGER, current_row, current_col, lexema, IntegerLiteralType::I32);
                    } else {
                        return Token::Integer(Tag::INTEGER, current_row, current_col, lexema, IntegerLiteralType::U32);
                    }
                }
                17 => {
                    match self.current_char() {
                        '4' => {
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
                        return Token::Integer(Tag::INTEGER, current_row, current_col, lexema, IntegerLiteralType::I64);
                    } else {
                        return Token::Integer(Tag::INTEGER, current_row, current_col, lexema, IntegerLiteralType::U64);
                    }
                }
                19 => {
                    match self.current_char() {
                        'i' => {
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
                        'z' => {
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
                        'e' => {
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
                        return Token::Integer(Tag::INTEGER, current_row, current_col, lexema, IntegerLiteralType::ISIZE);
                    } else {
                        return Token::Integer(Tag::INTEGER, current_row, current_col, lexema, IntegerLiteralType::USIZE);
                    }
                }
                23 => {
                    match self.current_char() {
                        c if c.is_digit(8) => {
                            lexema.push(c);
                            self.advance();
                            state = 23;
                            continue;
                        }
                        '_' => {
                            self.advance();
                            state = 23;
                            continue;
                        }
                        'i' => {
                            signed = true;
                            self.advance();
                            state = 9;
                            continue;
                        }
                        'u' => {
                            signed = false;
                            self.advance();
                            state = 9;
                            continue;
                        }
                        _ => {
                            // self.advance();
                            state = 28;
                            continue;
                        }
                    }
                }
                24 => {
                    match self.current_char() {
                        c if c.is_digit(16) => {
                            lexema.push(c);
                            self.advance();
                            state = 24;
                            continue;
                        }
                        '_' => {
                            self.advance();
                            state = 24;
                            continue;
                        }
                        'i' => {
                            signed = true;
                            self.advance();
                            state = 9;
                            continue;
                        }
                        'u' => {
                            signed = false;
                            self.advance();
                            state = 9;
                            continue;
                        }
                        _ => {
                            // self.advance();
                            state = 29;
                            continue;
                        }
                    }
                }
                25 => {
                    match self.current_char() {
                        '.' => {
                            lexema.push('.');
                            self.advance();
                            state = 35;
                            continue;
                        }
                        c if c.is_digit(10) => {
                            lexema.push(c);
                            self.advance();
                            state = 25;
                            continue;
                        }
                        '_' => {
                            self.advance();
                            state = 25;
                            continue;
                        }
                        'e' | 'E' => {
                            lexema.push('e');
                            self.advance();
                            state = 32;
                            continue;
                        }
                        'i' => {
                            signed = true;
                            self.advance();
                            state = 9;
                            continue;
                        }
                        'u' => {
                            signed = false;
                            self.advance();
                            state = 9;
                            continue;
                        }
                        _ => {
                            // self.advance();
                            state = 30;
                            continue;
                        }
                    }
                }
                26 => {
                    self.retract();
                    return Token::Integer(Tag::INTEGER, current_row, current_col, lexema, IntegerLiteralType::ISIZE);
                }
                27 => {
                    self.retract();
                    //todo!("Converter binário para inteiro");
                    return Token::Integer(Tag::INTEGER, current_row, current_col, lexema, IntegerLiteralType::ISIZE);
                }
                28 => {
                    self.retract();
                    //todo!("Converter octal para inteiro");
                    return Token::Integer(Tag::INTEGER, current_row, current_col, lexema, IntegerLiteralType::ISIZE);
                }
                29 => {
                    self.retract();
                    //todo!("Converter hexa para inteiro");
                    return Token::Integer(Tag::INTEGER, current_row, current_col, lexema, IntegerLiteralType::ISIZE);
                }
                30 => {
                    self.retract();
                    //todo!("Converter decimal para inteiro");
                    return Token::Integer(Tag::INTEGER, current_row, current_col, lexema, IntegerLiteralType::ISIZE);
                }
                31 => {
                    self.retract();
                    return Token::Integer(Tag::INTEGER, current_row, current_col, lexema, IntegerLiteralType::ISIZE);
                }
                32 => {
                    match self.current_char() {
                        c if c.is_digit(10) => {
                            lexema.push(c);
                            self.advance();
                            state = 34;
                            continue;
                        }
                        '+' => {
                            lexema.push('+');
                            self.advance();
                            state = 33;
                            continue;
                        }
                        '-' => {
                            lexema.push('-');
                            self.advance();
                            state = 33;
                            continue;
                        }
                        _ => {
                            todo!("Terra");
                        }
                    }
                }
                33 => {
                    match self.current_char() {
                        c if c.is_digit(10) => {
                            lexema.push(c);
                            self.advance();
                            state = 34;
                            continue;
                        }
                        '_' => {
                            self.advance();
                            state = 34;
                            continue;
                        }
                        _ => {
                            todo!("Terra");
                        }
                    }
                }
                34 => {
                    match self.current_char() {
                        c if c.is_digit(10)  => {
                            lexema.push(c);
                            self.advance();
                            state = 34;
                            continue;
                        }
                        '_' => {
                            self.advance();
                            state = 34;
                            continue;
                        }
                        'f' => {
                            lexema.push('f');
                            self.advance();
                            state = 36;
                            continue;
                        }
                        _ => {
                            // self.advance();
                            state = 41;
                            continue;
                        }
                    }
                }
                35 => {
                    match self.current_char() {
                        c if c.is_digit(10) => {
                            lexema.push(c);
                            self.advance();
                            state = 35;
                            continue;
                        }
                        '_' => {
                            self.advance();
                            state = 35;
                            continue;
                        }
                        'f' => {
                            lexema.push('f');
                            self.advance();
                            state = 36;
                            continue;
                        }
                        'e' | 'E' => {
                            lexema.push('e');
                            self.advance();
                            state = 32;
                            continue;
                        }
                        _ => {
                            // self.advance();
                            state = 41;
                            continue;
                        }
                    }
                }
                36 => {
                    match self.current_char() {
                        '3' => {
                            lexema.push('3');
                            self.advance();
                            state = 37;
                            continue;
                        }
                        '6' => {
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
                        '2' => {
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
                    return Token::Float(Tag::FLOAT, current_row, current_col, lexema, FloatLiteralType::F32);
                }
                39 => {
                    match self.current_char() {
                        '4' => {
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
                    return Token::Float(Tag::FLOAT, current_row, current_col, lexema, FloatLiteralType::F64);
                }
                41 => {
                    self.retract();
                    return Token::Float(Tag::FLOAT, current_row, current_col, lexema, FloatLiteralType::F64);
                }
                42 => {
                    match self.current_char() {
                        'f' => {
                            self.advance();
                            state = 36;
                            continue;
                        }
                        _ => {
                            // self.advance();
                            state = 40;
                            continue;
                        }
                    }
                }

                _ => {
                    return Token::Error(Tag::ERR, current_row, current_col, format!("Número mal formado: {}", lexema));
                }
            }
        }


        /*
// Numeracao dos estados == numeracao das figuras/tabela:
//
//   0  q0   inicio
//   1  q1   apos b ou c
//   2  q2   apos r
//   3  q3   contando # de abertura
//   4  q4   corpo com escapes
//   5  q5   corpo raw
//   6  q6   apos a barra invertida
//   7  q7   primeiro digito de \xHH
//   8  q7b  segundo digito de \xHH
//   9  q8   apos \u, esperando {
//  10  q8a  primeiro digito dentro de {...}
//  11  q8b  demais digitos / '_' / fechamento }
//  12  q9   contando # de fechamento (raw)
//  13  q10  continuacao de linha
//
// qf (aceita) e implementado como `break` no ponto exato da transicao de
// aceitacao, nao como mais um valor de `state` -- ele encerra o loop.
// qe (erro) continua representado por `todo!()`, exatamente como no codigo
// original; numa implementacao final cada `todo!()` deveria virar um erro
// lexico apropriado (ver observacoes ao final).

enum StringKind {
    Str,
    ByteStr,
    CStr,
}

// Estrutura ilustrativa do literal reconhecido. Ajuste para o `Token` real
// do seu lexer -- o importante aqui e a maquina de estados, nao este tipo.
struct StringLiteralInfo {
    kind: StringKind,
    raw: bool,
    hashes: u32,
    lexema: String, // texto-fonte exato, incluindo prefixo, aspas e #
    valor: Vec<u8>, // conteudo ja decodificado (escapes resolvidos)
}

fn push_char(buf: &mut Vec<u8>, c: char) {
    let mut tmp = [0u8; 4];
    buf.extend_from_slice(c.encode_utf8(&mut tmp).as_bytes());
}

impl Lexer {
    fn scan_string(&mut self) -> Token {
        let mut state = 0;
        let mut n_char = 0;
        let mut n_sharp = 0u32; // n: cerquilhas da abertura
        let mut k = 0u32;       // k: cerquilhas ja casadas no fechamento
        let mut hex = 0u32;
        let mut ndig = 0u32;
        let mut is_byte = false;
        let mut is_cstr = false;
        let mut is_raw = false;
        let mut lexema = String::new();
        let mut valor: Vec<u8> = Vec::new();

        loop {
            match state {
                // ---------- prefixo e abertura (corrigido) ----------
                0 => match self.current_char() {
                    'r' => {
                        lexema.push('r');
                        self.advance();
                        n_char += 1;
                        is_raw = true;
                        state = 2;
                        continue;
                    }
                    'b' => {
                        lexema.push('b');
                        self.advance();
                        n_char += 1;
                        is_byte = true;
                        state = 1;
                        continue;
                    }
                    'c' => {
                        lexema.push('c');
                        self.advance();
                        n_char += 1;
                        is_cstr = true;
                        state = 1;
                        continue;
                    }
                    '\"' => {
                        lexema.push('\"');
                        self.advance();
                        n_char += 1;
                        state = 4;
                        continue;
                    }
                    _ => {
                        todo!(); // qe: nao e inicio de literal de cadeia
                    }
                },

                1 => match self.current_char() {
                    'r' => {
                        lexema.push('r');
                        self.advance();
                        n_char += 1;
                        is_raw = true;
                        state = 2;
                        continue;
                    }
                    '\"' => {
                        lexema.push('\"');
                        self.advance();
                        n_char += 1;
                        state = 4;
                        continue;
                    }
                    _ => {
                        todo!();
                    }
                },

                2 => match self.current_char() {
                    '\"' => {
                        lexema.push('\"');
                        self.advance();
                        n_char += 1;
                        state = 5;
                        continue;
                    }
                    '#' => {
                        lexema.push('#');
                        self.advance();
                        n_char += 1;
                        n_sharp += 1;
                        state = 3;
                        continue;
                    }
                    _ => {
                        todo!();
                    }
                },

                3 => match self.current_char() {
                    '\"' => {
                        lexema.push('\"');
                        self.advance();
                        n_char += 1;
                        state = 5;
                        continue;
                    }
                    '#' => {
                        lexema.push('#');
                        self.advance();
                        n_char += 1;
                        n_sharp += 1;
                        state = 3;
                        continue;
                    }
                    _ => {
                        todo!();
                    }
                },

                // ---------- q4: corpo nao-raw, com escapes ----------
                4 => match self.current_char() {
                    '\"' => {
                        lexema.push('\"');
                        self.advance();
                        n_char += 1;
                        break; // qf: literal completo
                    }
                    '\\' => {
                        lexema.push('\\');
                        self.advance();
                        n_char += 1;
                        state = 6;
                        continue;
                    }
                    c => {
                        if is_byte && !c.is_ascii() {
                            todo!(); // qe: b"..." so aceita ASCII
                        }
                        lexema.push(c);
                        push_char(&mut valor, c);
                        self.advance();
                        n_char += 1;
                        // state permanece 4
                        continue;
                    }
                },

                // ---------- q6: logo apos a barra invertida ----------
                6 => match self.current_char() {
                    'n' => {
                        lexema.push('n');
                        valor.push(b'\n');
                        self.advance();
                        n_char += 1;
                        state = 4;
                        continue;
                    }
                    'r' => {
                        lexema.push('r');
                        valor.push(b'\r');
                        self.advance();
                        n_char += 1;
                        state = 4;
                        continue;
                    }
                    't' => {
                        lexema.push('t');
                        valor.push(b'\t');
                        self.advance();
                        n_char += 1;
                        state = 4;
                        continue;
                    }
                    '\\' => {
                        lexema.push('\\');
                        valor.push(b'\\');
                        self.advance();
                        n_char += 1;
                        state = 4;
                        continue;
                    }
                    '\'' => {
                        lexema.push('\'');
                        valor.push(b'\'');
                        self.advance();
                        n_char += 1;
                        state = 4;
                        continue;
                    }
                    '\"' => {
                        lexema.push('\"');
                        valor.push(b'\"');
                        self.advance();
                        n_char += 1;
                        state = 4;
                        continue;
                    }
                    '0' => {
                        if is_cstr {
                            todo!(); // qe: c"..." nao pode ter byte nulo
                        }
                        lexema.push('0');
                        valor.push(0);
                        self.advance();
                        n_char += 1;
                        state = 4;
                        continue;
                    }
                    'x' => {
                        lexema.push('x');
                        self.advance();
                        n_char += 1;
                        hex = 0;
                        state = 7;
                        continue;
                    }
                    'u' => {
                        if is_byte {
                            todo!(); // qe: \u{...} proibido em b"..."
                        }
                        lexema.push('u');
                        self.advance();
                        n_char += 1;
                        state = 9;
                        continue;
                    }
                    c @ ('\n' | '\r') => {
                        lexema.push(c);
                        self.advance();
                        n_char += 1;
                        state = 13;
                        continue;
                    }
                    _ => {
                        todo!(); // qe: escape desconhecido
                    }
                },

                // ---------- q7 / q7b: \xHH ----------
                7 => {
                    let c = self.current_char();
                    match c.to_digit(16) {
                        Some(d) => {
                            lexema.push(c);
                            hex = d;
                            self.advance();
                            n_char += 1;
                            state = 8;
                            continue;
                        }
                        None => {
                            todo!();
                        }
                    }
                }

                8 => {
                    let c = self.current_char();
                    match c.to_digit(16) {
                        Some(d) => {
                            lexema.push(c);
                            hex = hex * 16 + d;
                            if !is_byte && hex > 0x7F {
                                todo!(); // qe: \x acima de 7F fora de byte string
                            }
                            if is_cstr && hex == 0 {
                                todo!(); // qe: byte nulo em c"..."
                            }
                            if is_byte {
                                valor.push(hex as u8);
                            } else {
                                push_char(&mut valor, char::from_u32(hex).unwrap());
                            }
                            self.advance();
                            n_char += 1;
                            state = 4;
                            continue;
                        }
                        None => {
                            todo!();
                        }
                    }
                }

                // ---------- q8 / q8a / q8b: \u{...} ----------
                9 => match self.current_char() {
                    '{' => {
                        lexema.push('{');
                        self.advance();
                        n_char += 1;
                        hex = 0;
                        ndig = 0;
                        state = 10;
                        continue;
                    }
                    _ => {
                        todo!(); // qe: esperado { apos \u
                    }
                },

                10 => {
                    let c = self.current_char();
                    match c.to_digit(16) {
                        Some(d) => {
                            lexema.push(c);
                            hex = d;
                            ndig = 1;
                            self.advance();
                            n_char += 1;
                            state = 11;
                            continue;
                        }
                        None => {
                            todo!(); // qe: \u{...} exige ao menos um digito
                        }
                    }
                }

                11 => match self.current_char() {
                    '_' => {
                        lexema.push('_');
                        self.advance();
                        n_char += 1;
                        // state permanece 11
                        continue;
                    }
                    '}' => {
                        lexema.push('}');
                        if is_cstr && hex == 0 {
                            todo!(); // qe: byte nulo em c"..."
                        }
                        match char::from_u32(hex) {
                            Some(c) => push_char(&mut valor, c),
                            None => todo!(), // qe: code point invalido
                        }
                        self.advance();
                        n_char += 1;
                        state = 4;
                        continue;
                    }
                    c => match c.to_digit(16) {
                        Some(d) => {
                            ndig += 1;
                            if ndig > 6 {
                                todo!(); // qe: mais de 6 digitos em \u{...}
                            }
                            lexema.push(c);
                            hex = hex * 16 + d;
                            self.advance();
                            n_char += 1;
                            // state permanece 11
                            continue;
                        }
                        None => {
                            todo!();
                        }
                    },
                },

                // ---------- q10: continuacao de linha ----------
                13 => match self.current_char() {
                    c @ (' ' | '\t' | '\n' | '\r') => {
                        lexema.push(c);
                        self.advance();
                        n_char += 1;
                        // state permanece 13
                        continue;
                    }
                    _ => {
                        // retro: NAO consome o caractere, apenas volta a q4
                        state = 4;
                        continue;
                    }
                },

                // ---------- q5: corpo raw ----------
                5 => match self.current_char() {
                    '\"' => {
                        lexema.push('\"');
                        self.advance();
                        n_char += 1;
                        if n_sharp == 0 {
                            break; // qf: r"..." fecha na primeira aspa
                        }
                        k = 0;
                        state = 12;
                        continue;
                    }
                    c => {
                        if is_byte && !c.is_ascii() {
                            todo!();
                        }
                        lexema.push(c);
                        push_char(&mut valor, c);
                        self.advance();
                        n_char += 1;
                        // state permanece 5
                        continue;
                    }
                },

                // ---------- q9: contando # de fechamento ----------
                12 => match self.current_char() {
                    '#' => {
                        lexema.push('#');
                        self.advance();
                        n_char += 1;
                        k += 1;
                        if k == n_sharp {
                            break; // qf: fechamento completo
                        }
                        // state permanece 12
                        continue;
                    }
                    '\"' => {
                        // a aspa e as k cerquilhas ja lidas eram conteudo;
                        // devolve-as ao buffer e reinicia a contagem
                        valor.push(b'\"');
                        for _ in 0..k {
                            valor.push(b'#');
                        }
                        lexema.push('\"');
                        self.advance();
                        n_char += 1;
                        k = 0;
                        // state permanece 12
                        continue;
                    }
                    c => {
                        valor.push(b'\"');
                        for _ in 0..k {
                            valor.push(b'#');
                        }
                        push_char(&mut valor, c);
                        lexema.push(c);
                        self.advance();
                        n_char += 1;
                        k = 0;
                        state = 5;
                        continue;
                    }
                },

                _ => {
                    todo!();
                }
            }
        }

        let kind = if is_byte {
            StringKind::ByteStr
        } else if is_cstr {
            StringKind::CStr
        } else {
            StringKind::Str
        };

        Token::StringLiteral(StringLiteralInfo {
            kind,
            raw: is_raw,
            hashes: n_sharp,
            lexema,
            valor,
        })
    }
}         */
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
