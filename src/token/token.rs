#![allow(dead_code)]

use crate::tags::{Tag, IDENTIFIER};
use crate::tags::{NUM, ADD, MUL, LPR, RPR, END, ERR};
use crate::tags::keywords;

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

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Keyword(Tag, String),
    Identifier(Tag, String),
    Number(Tag, i32),
    AddOperator(Tag, AddOp),
    MulOperator(Tag, MulOp),
    LPar(Tag),
    RPar(Tag),
    EndMark(Tag),
    Error(Tag, String),
}

pub trait Scanner {
    fn next_token(&mut self) -> Token;
}

pub struct Expression {
    current_position: usize,
    chars: Vec<char>,
}

impl Expression {
    pub fn new(text: &str) -> Self {
        Expression {
            current_position: 0,
            chars: text.chars().collect(),
        }
    }

    fn current_char(&self) -> Option<char> {
        self.chars.get(self.current_position).copied()
    }

    /// Espia o próximo caractere sem avançar a posição
    fn peek_char(&self) -> Option<char> {
        self.chars.get(self.current_position + 1).copied()
    }

    /// Avança para o próximo caractere e retorna o caractere atual
    fn advance(&mut self) {
        self.current_position += 1;
    }

    /// Volta uma posição com segurança (não vai abaixo de 0)
    fn retract(&mut self) {
        if self.current_position > 0 {
            self.current_position -= 1;
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
                        Some('+') => {
                            self.advance();
                            state = 3;
                            continue;
                        }
                        Some('-') => {
                            self.advance();
                            state = 4;
                            continue;
                        }
                        Some('*') => {
                            self.advance();
                            state = 5;
                            continue;
                        }
                        Some('/') => {
                            self.advance();
                            state = 6;
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
                    let value = lexema.parse::<i32>().unwrap();
                    return Token::Number(NUM, value);
                }
                3 => {
                    return Token::AddOperator(ADD, AddOp::Plus);
                }
                4 => {
                    return Token::AddOperator(ADD, AddOp::Minus);
                }
                5 => {
                    return Token::MulOperator(MUL, MulOp::Times);
                }
                6 => {
                    return Token::MulOperator(MUL, MulOp::Divide);
                }
                7 => {
                    return Token::LPar(LPR);
                }
                8 => {
                    return Token::RPar(RPR);
                }
                9 => {
                    return Token::EndMark(END);
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
                            state = 10;
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
                    match Tag::from_keyword(&lexema, keywords::KeywordContext::Normal) {
                        Some(tag) => return Token::Keyword(tag, lexema),
                        _ => return Token::Identifier(IDENTIFIER, lexema),
                    }
                }
                999 => {
                    self.retract();
                    return Token::Error(ERR, format!("Caracter inválido: {}", lexema));
                }
                _ => {
                    return Token::Error(ERR, "Caracter inválido".to_string());
                }
            }
        }
    }
}

/*
!	ident!(...), ident!{...}, ident![...]	Macro expansion	
!	!expr	Bitwise or logical complement	Not
!=	expr != expr	Nonequality comparison	PartialEq
%	expr % expr	Arithmetic remainder	Rem
%=	var %= expr	Arithmetic remainder and assignment	RemAssign
&	&expr, &mut expr	Borrow	
&	&type, &mut type, &'a type, &'a mut type	Borrowed pointer type	
&	expr & expr	Bitwise AND	BitAnd
&=	var &= expr	Bitwise AND and assignment	BitAndAssign
&&	expr && expr	Short-circuiting logical AND	
*	expr * expr	Arithmetic multiplication	Mul
*=	var *= expr	Arithmetic multiplication and assignment	MulAssign
*	*expr	Dereference	Deref
*	*const type, *mut type	Raw pointer	
+	trait + trait, 'a + trait	Compound type constraint	
+	expr + expr	Arithmetic addition	Add
+=	var += expr	Arithmetic addition and assignment	AddAssign
,	expr, expr	Argument and element separator	
-	- expr	Arithmetic negation	Neg
-	expr - expr	Arithmetic subtraction	Sub
-=	var -= expr	Arithmetic subtraction and assignment	SubAssign
->	fn(...) -> type, |…| -> type	Function and closure return type	
.	expr.ident	Field access	
.	expr.ident(expr, ...)	Method call	
.	expr.0, expr.1, and so on	Tuple indexing	
..	.., expr.., ..expr, expr..expr	Right-exclusive range literal	PartialOrd
..=	..=expr, expr..=expr	Right-inclusive range literal	PartialOrd
..	..expr	Struct literal update syntax	
..	variant(x, ..), struct_type { x, .. }	“And the rest” pattern binding	
...	expr...expr	(Deprecated, use ..= instead) In a pattern: inclusive range pattern	
/	expr / expr	Arithmetic division	Div
/=	var /= expr	Arithmetic division and assignment	DivAssign
:	pat: type, ident: type	Constraints	
:	ident: expr	Struct field initializer	
:	'a: loop {...}	Loop label	
;	expr;	Statement and item terminator	
;	[...; len]	Part of fixed-size array syntax	
<<	expr << expr	Left-shift	Shl
<<=	var <<= expr	Left-shift and assignment	ShlAssign
<	expr < expr	Less than comparison	PartialOrd
<=	expr <= expr	Less than or equal to comparison	PartialOrd
=	var = expr, ident = type	Assignment/equivalence	
==	expr == expr	Equality comparison	PartialEq
=>	pat => expr	Part of match arm syntax	
>	expr > expr	Greater than comparison	PartialOrd
>=	expr >= expr	Greater than or equal to comparison	PartialOrd
>>	expr >> expr	Right-shift	Shr
>>=	var >>= expr	Right-shift and assignment	ShrAssign
@	ident @ pat	Pattern binding	
^	expr ^ expr	Bitwise exclusive OR	BitXor
^=	var ^= expr	Bitwise exclusive OR and assignment	BitXorAssign
|	pat | pat	Pattern alternatives	
|	expr | expr	Bitwise OR	BitOr
|=	var |= expr	Bitwise OR and assignment	BitOrAssign
||	expr || expr	Short-circuiting logical OR	
?	expr?	Error propagation
*/
