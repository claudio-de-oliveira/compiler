use phf::phf_map;

use std::fmt;

#[repr(u32)]
#[derive(Clone, Copy)] // Permite cópia para uso nas constantes
pub enum Type {
    TERMINAL = 0x8000,
    VARIABLE = 0x4000,
    ACTION = 0x2000
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Tag {
    value: u32,
}

impl fmt::Debug for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Tag")
            .field("value", &format!("0x{:04x}", self.value))
            .finish()
    }
}

impl Tag {
    pub const fn new(tp: Type, val: u32) -> Self {
        Tag {
            value: (tp as u32) | val
        }
    }

    pub fn is_terminal(self) -> bool {
        self.value & (Type::TERMINAL as u32) != 0
    }

    pub fn is_variable(self) -> bool {
        self.value & (Type::VARIABLE as u32) != 0
    }

    pub fn is_action(self) -> bool {
        self.value & (Type::ACTION as u32) != 0
    }

    pub fn get_value(self) -> u32 {
        self.value & 0x0fff
    }
}

// Palavras Reservadas
macro_rules! terminal {
    ($name:ident, $value:expr) => {
        pub const $name: Tag = Tag::new(crate::tags::Type::TERMINAL, $value);
    };
}

pub mod keywords {
    use super::*;

    #[derive(PartialEq, Eq, Hash)]
    pub enum KeywordContext {
        Normal,
        Future,
        Lifetimes,
        Union,
    }

    terminal!(AS, 0);
    terminal!(BREAK, 1);
    terminal!(CONST, 2);
    terminal!(CONTINUE, 3);
    terminal!(CRATE, 4);
    terminal!(ELSE, 5);
    terminal!(ENUM, 6);
    terminal!(EXTERN, 7);
    terminal!(FALSE, 8);
    terminal!(FN, 9);
    terminal!(FOR, 10);
    terminal!(IF, 11);
    terminal!(IMPL, 12);
    terminal!(IN, 13);
    terminal!(LET, 14);
    terminal!(LOOP, 15);
    terminal!(MATCH, 16);
    terminal!(MOD, 17);
    terminal!(MOVE, 18);
    terminal!(MUT, 19);
    terminal!(PUB, 20);
    terminal!(REF, 21);
    terminal!(RETURN, 22);
    terminal!(SELF, 23);
    terminal!(SELF_TYPE, 24);
    terminal!(STATIC, 25);
    terminal!(STRUCT, 26);
    terminal!(SUPER, 27);
    terminal!(TRAIT, 28);
    terminal!(TRUE, 29);
    terminal!(TYPE, 30);
    terminal!(UNSAFE, 31);
    terminal!(USE, 32);
    terminal!(WHERE, 33);
    terminal!(WHILE, 34);
    terminal!(ASYNC, 35);
    terminal!(AWAIT, 36);
    terminal!(DYN, 37);
    terminal!(ABSTRACT, 38);
    terminal!(BECOME, 39);
    terminal!(BOX, 40);
    terminal!(DO, 41);
    terminal!(FINAL, 42);
    terminal!(MACRO, 43);
    terminal!(OVERRIDE, 44);
    terminal!(PRIV, 45);
    terminal!(TYPEOF, 46);
    terminal!(UNSIZED, 47);
    terminal!(VIRTUAL, 48);
    terminal!(YIELD, 49);
    terminal!(TRY, 50);
    terminal!(UNION, 51);
    terminal!(STATIC_LT, 52);

    static NORMAL_KEYWORD_TAGS: phf::Map<&'static str, Tag> = phf_map! {
        "as" => AS,
        "break" => BREAK,
        "const" => CONST,
        "continue" => CONTINUE,
        "crate" => CRATE,
        "else" => ELSE,
        "enum" => ENUM,
        "extern" => EXTERN,
        "false" => FALSE,
        "fn" => FN,
        "for" => FOR,
        "if" => IF,
        "impl" => IMPL,
        "in" => IN,
        "let" => LET,
        "loop" => LOOP,
        "match" => MATCH,
        "mod" => MOD,
        "move" => MOVE,
        "mut" => MUT,
        "pub" => PUB,
        "ref" => REF,
        "return" => RETURN,
        "self" => SELF,
        "Self" => SELF_TYPE,
        "static" => STATIC,
        "struct" => STRUCT,
        "super" => SUPER,
        "trait" => TRAIT,
        "true" => TRUE,
        "type" => TYPE,
        "unsafe" => UNSAFE,
        "use" => USE,
        "where" => WHERE,
        "while" => WHILE,
        "async" => ASYNC,
        "await" => AWAIT,
        "dyn" => DYN,
    };

    static FUTURE_KEYWORD_TAGS: phf::Map<&'static str, Tag> = phf_map! {
        "abstract" => ABSTRACT,
        "become" => BECOME,
        "box" => BOX,
        "do" => DO,
        "final" => FINAL,
        "macro" => MACRO,
        "override" => OVERRIDE,
        "priv" => PRIV,
        "typeof" => TYPEOF,
        "unsized" => UNSIZED,
        "virtual" => VIRTUAL,
        "yield" => YIELD,
        "try" => TRY,
    };

    impl Tag {
        pub fn from_keyword(lexeme: &str, context: KeywordContext) -> Option<Self> {
            if context == KeywordContext::Lifetimes {
                if lexeme == "static" {
                    return Some(STATIC_LT);
                }
            }
            if context == KeywordContext::Union {
                if lexeme == "union" {
                    return Some(UNION);
                }
            }
            if context == KeywordContext::Future {
                if let Some(tag) = FUTURE_KEYWORD_TAGS.get(lexeme) {
                    return Some(*tag);
                }
            }
            if context == KeywordContext::Normal {
                if let Some(tag) = NORMAL_KEYWORD_TAGS.get(lexeme) {
                    return Some(*tag);
                }
            }

            return None;
        }
    }
}

terminal!(IDENTIFIER, 53);

terminal!(NUM, 54);
terminal!(ADD, 55);
terminal!(MUL, 56);
terminal!(LPR, 57);
terminal!(RPR, 58);
terminal!(END, 59);
terminal!(ERR, 60);
