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
    Number(Tag, String),
    AddOperator(Tag, AddOp),
    MulOperator(Tag, MulOp),
    LPar(Tag),
    RPar(Tag),
    EndMark(Tag),
    Error(Tag, String),
    DefaultPattern(Tag),
    Not(Tag),                    // !	!expr	Bitwise or logical complement	Not
    Equality(Tag, EqualityOp),   // !=	expr != expr	Nonequality comparison	PartialEq                        
    RemainderOp(Tag),            // %	expr % expr	Arithmetic remainder	Rem
    OpAndAssignment(Tag, Op),    // %=	var %= expr	Arithmetic remainder and assignment	RemAssign
                                 // &=	var &= expr	Bitwise AND and assignment	BitAndAssign
                                 // *=	var *= expr	Arithmetic multiplication and assignment	MulAssign
                                 // +=	var += expr	Arithmetic addition and assignment	AddAssign
                                 // -=	var -= expr	Arithmetic subtraction and assignment	SubAssign
    
                            // &	&expr, &mut expr	Borrow	
                            // &	&type, &mut type, &'a type, &'a mut type	Borrowed pointer type	
                    return AMPERSAND; // &=	var &= expr	Bitwise AND and assignment	BitAndAssign
                    return AND; // &&	expr && expr	Short-circuiting logical AND
                            // * 	expr * expr	Arithmetic multiplication	Mul
                            // *	*expr	Dereference	Deref
                            // *	*const type, *mut type	Raw pointer	
                            return STAR;
                            // +	trait + trait, 'a + trait	Compound type constraint	
                            // +	expr + expr	Arithmetic addition	Add
                            return PLUS;
                    return COMMA; // ,   expr, expr	Argument and element separator	
                            return MINUS; // - 	- expr	Arithmetic negation	Neg
                                          // -	expr - expr	Arithmetic subtraction	Sub
                    return ReturnType; // ->	fn(...) -> type, |…| -> type	Function and closure return type
                            // .	expr.ident	Field access	
                            // .	expr.ident(expr, ...)	Method call	
                            // .	expr.0, expr.1, and so on	Tuple indexing	
                            return PT;
                            // ..	.., expr.., ..expr, expr..expr	Right-exclusive range literal	PartialOrd
                            // ..	..expr	Struct literal update syntax	
                            // ..	variant(x, ..), struct_type { x, .. }	“And the rest” pattern binding	
                            return PTPT;
                    return RINRANGE; // ..=	..=expr, expr..=expr	Right-inclusive range literal	PartialOrd
    
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
                        self.retract();
                        return Token::DefaultPattern(DEFAULT),
                    }
                    match Tag::from_keyword(&lexema, keywords::KeywordContext::Normal) {
                        Some(tag) => return Token::Keyword(tag, lexema),
                        _ => return Token::Identifier(IDENTIFIER, lexema),
                    }
                }
                12 => {
                    // Macro
                    return Token::Identifier(IDENTIFIER, lexema),
                }

                100 => {
                    match self.current_char() {
                        Some('=') => {
                            self.advance();
                            state = 1001;
                            continue;
                        }
                        _  => {
                            return NOT; // !	!expr	Bitwise or logical complement	Not
                        }
                    }
                }
                1001 => {
                    return NEQ; // !=	expr != expr	Nonequality comparison	PartialEq                        
                }
                
                101 => {
                    match self.current_char() {
                        Some('=') => {
                            self.advance();
                            state = 1011;
                            continue;
                        }
                        _  => {
                            return REM; // %	expr % expr	Arithmetic remainder	Rem
                        }
                    }
                }
                1011 => {
                    return REMASSIGN; // %=	var %= expr	Arithmetic remainder and assignment	RemAssign
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
                            // &	&expr, &mut expr	Borrow	
                            // &	&type, &mut type, &'a type, &'a mut type	Borrowed pointer type	
                            return AMPERSAND; // &	expr & expr	Bitwise AND	BitAnd
                        }
                    }
                }
                1021 => {
                    return BITANDASSIGN; // &=	var &= expr	Bitwise AND and assignment	BitAndAssign
                }
                1022 => {
                    return AND; // &&	expr && expr	Short-circuiting logical AND
                }
                
                103 => {
                    match self.current_char() {
                        Some('=') => {
                            self.advance();
                            state = 1031;
                            continue;
                        }
                        _  => {
                            // * 	expr * expr	Arithmetic multiplication	Mul
                            // *	*expr	Dereference	Deref
                            // *	*const type, *mut type	Raw pointer	
                            return STAR;
                        }
                    }
                }
                1031 => {
                    return MulAssign; // *=	var *= expr	Arithmetic multiplication and assignment	MulAssign
                }
                
                104 => {
                    match self.current_char() {
                        Some('=') => {
                            self.advance();
                            state = 1041;
                            continue;
                        }
                        _  => {
                            // +	trait + trait, 'a + trait	Compound type constraint	
                            // +	expr + expr	Arithmetic addition	Add
                            return PLUS;
                        }
                    }
                }
                1041 => {
                    return ADDASSIGN; // +=	var += expr	Arithmetic addition and assignment	AddAssign
                }
                
                105 => {
                    return COMMA; // ,   expr, expr	Argument and element separator	
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
                            return MINUS; // - 	- expr	Arithmetic negation	Neg
                                          // -	expr - expr	Arithmetic subtraction	Sub
                        }
                    }
                }
                1061 => {
                    return SUBASSIGN; // -=	var -= expr	Arithmetic subtraction and assignment	SubAssign
                }
                1062 => {
                    return ReturnType; // ->	fn(...) -> type, |…| -> type	Function and closure return type
                }
                
                107 => {
                    match self.current_char() {
                        Some('.') => {
                            self.advance();
                            state = 1071;
                            continue;
                        }
                        _  => {
                            // .	expr.ident	Field access	
                            // .	expr.ident(expr, ...)	Method call	
                            // .	expr.0, expr.1, and so on	Tuple indexing	
                            return PT;
                        }
                    }
                }
                1071 => {
                        Some('=') => {
                            self.advance();
                            state = 10711;
                            continue;
                        }
                        _  => {
                            // ..	.., expr.., ..expr, expr..expr	Right-exclusive range literal	PartialOrd
                            // ..	..expr	Struct literal update syntax	
                            // ..	variant(x, ..), struct_type { x, .. }	“And the rest” pattern binding	
                            return PTPT;
                        }
                }
                10711 => {
                    return RINRANGE; // ..=	..=expr, expr..=expr	Right-inclusive range literal	PartialOrd
                }
                
                108 => {
                    match self.current_char() {
                        Some('=') => {
                            self.advance();
                            state = 1081;
                            continue;
                        }
                        _  => {
                            return DIV; // /	expr / expr	Arithmetic division	Div
                        }
                    }
                }
                1081 => {
                    return DIVASSIGN; // /=	var /= expr	Arithmetic division and assignment	DivAssign
                }

                109 => {
                    match self.current_char() {
                        _  => {
                            // :	pat: type, ident: type	Constraints	
                            // :	ident: expr	Struct field initializer	
                            // :	'a: loop {...}	Loop label
                            return COLON;
                        }
                    }
                }

                110 => {
                    // ;	expr;	Statement and item terminator	
                    // ;	[...; len]	Part of fixed-size array syntax	
                    return SEMICOLON;
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
                            return LT; // <	expr < expr	Less than comparison	PartialOrd
                        }
                    }
                }
                1111 => {
                    return LTE;     // <=	expr <= expr	Less than or equal to comparison	PartialOrd
                }
                1112 => {
                    match self.current_char() {
                        Some('=') => {
                            self.advance();
                            state = 11121;
                            continue;
                        }
                        _  => {
                            return SHL; // <<	<	expr << expr	Left-shift	Shl
                        }
                    }
                }
                11121 => {
                    return SHLASSIGN; // <<=	var <<= expr	Left-shift and assignment	ShlAssign
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
                            // =	var = expr, ident = type	Assignment/equivalence	
                            return EQUAL;
                        }
                    }
                }
                11211 => {
                    return EQ; // ==	expr == expr	Equality comparison	PartialEq
                }
                11212 => {
                    return MatchArm; // =>	pat => expr	Part of match arm syntax
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
                            return GT; // >	expr > expr	Greater than comparison	PartialOrd
                        }
                    }
                }
                1131 => {
                    return GTE;  // >=	expr >= expr	Greater than or equal to comparison	PartialOrd
                }
                1132 => {
                    match self.current_char() {
                        Some('=') => {
                            self.advance();
                            state = 11311;
                            continue;
                        }
                        _  => {
                            return SHR; // >>	expr >> expr	Right-shift	Shr
                        }
                    }
                }
                11311 => {
                    return SHRASSIGN; // >>=	var >>= expr	Right-shift and assignment	ShrAssign
                }

                114 => {
                    // @	ident @ pat	Pattern binding	
                }

                115 => {
                    match self.current_char() {
                        Some('=') => {
                            self.advance();
                            state = 1151;
                            continue;
                        }
                        _  => {
                            return BITXOR; // ^	expr ^ expr	Bitwise exclusive OR	BitXor
                        }
                    }
                }
                1151 => {
                    return BITXORASSIGN; // ^=	var ^= expr	Bitwise exclusive OR and assignment	BitXorAssign
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
                            // |	pat | pat	Pattern alternatives	
                            // |	expr | expr	Bitwise OR	BitOr
                            return VBAR;
                        }
                    }
                }
                1161 => {
                    return BITORASSIGN; // |=	var |= expr	Bitwise OR and assignment	BitOrAssign
                }
                1162 => {
                    return OR; // ||	expr || expr	Short-circuiting logical OR	
                }

                117 => {
                    return ErrorPropagation; //    expr?	Error propagation
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

pub enum SymbolContext {
    Expression,
    Declaration,
    CompoundTypeConstraint,
    Pattern,
    
!	ident!(...), ident!{...}, ident![...]	Macro expansion
    
!	!expr	Bitwise or logical complement	Not                                Expression,
!=	expr != expr	Nonequality comparison	PartialEq                          Expression,
%	expr % expr	Arithmetic remainder	Rem                                    Expression,
%=	var %= expr	Arithmetic remainder and assignment	RemAssign                  Expression,

    &	&expr, &mut expr	Borrow	                                           Expression,
    &	&type, &mut type, &'a type, &'a mut type	Borrowed pointer type	   Declaration,
&&	expr && expr	Short-circuiting logical AND	                           Expression,
*	expr * expr	Arithmetic multiplication	Mul                                Expression,
*=	var *= expr	Arithmetic multiplication and assignment	MulAssign          Expression,
*	*expr	Dereference	Deref                                                  Expression,
*	*const type, *mut type	Raw pointer	                                       Declaration,
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
>>	expr >> expr	Right-shift	Shr
<<=	var <<= expr	Left-shift and assignment	ShlAssign
>>=	var >>= expr	Right-shift and assignment	ShrAssign
<	expr < expr	Less than comparison	PartialOrd
<=	expr <= expr	Less than or equal to comparison	PartialOrd
=	var = expr, ident = type	Assignment/equivalence	
==	expr == expr	Equality comparison	PartialEq
=>	pat => expr	Part of match arm syntax	
>	expr > expr	Greater than comparison	PartialOrd
>=	expr >= expr	Greater than or equal to comparison	PartialOrd
@	ident @ pat	Pattern binding	
|	pat | pat	Pattern alternatives	

&	expr & expr	Bitwise AND	BitAnd                                             Expression,
^	expr ^ expr	Bitwise exclusive OR	BitXor
|	expr | expr	Bitwise OR	BitOr

&=	var &= expr	Bitwise AND and assignment	BitAndAssign                       Expression,
^=	var ^= expr	Bitwise exclusive OR and assignment	BitXorAssign
|=	var |= expr	Bitwise OR and assignment	BitOrAssign

||	expr || expr	Short-circuiting logical OR	
?	expr?	Error propagation	    
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
