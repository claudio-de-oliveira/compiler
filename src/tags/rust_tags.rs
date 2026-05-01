use phf::phf_map;
use super::tag::TERMINAL;

macro_rules! R {
    ($value:expr) => {
        TERMINAL.wrapping_add($value as u16) as isize
    };
}
macro_rules! T {
    ($value:expr) => {
        TERMINAL.wrapping_add($value as u16) as isize
    };
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tag {
    AS = R!(0),
    BREAK = R!(1),
    CONST = R!(2),
    CONTINUE = R!(3),
    CRATE = R!(4),
    ELSE = R!(5),
    ENUM = R!(6),
    EXTERN = R!(7),
    FALSE = R!(8),
    FN = R!(9),
    FOR = R!(10),
    IF = R!(11),
    IMPL = R!(12),
    IN = R!(13),
    LET = R!(14),
    LOOP = R!(15),
    MATCH = R!(16),
    MOD = R!(17),
    MOVE = R!(18),
    MUT = R!(19),
    PUB = R!(20),
    REF = R!(21),
    RETURN = R!(22),
    SELF = R!(23),
    SELFTYPE = R!(24),
    STATIC = R!(25),
    STRUCT = R!(26),
    SUPER = R!(27),
    TRAIT = R!(28),
    TRUE = R!(29),
    TYPE = R!(30),
    UNSAFE = R!(31),
    USE = R!(32),
    WHERE = R!(33),
    WHILE = R!(34),
    ASYNC = R!(35),
    AWAIT = R!(36),
    DYN = R!(37),
    ABSTRACT = R!(38),
    BECOME = R!(39),
    BOX = R!(40),
    DO = R!(41),
    FINAL = R!(42),
    MACRO = R!(43),
    OVERRIDE = R!(44),
    PRIV = R!(45),
    TYPEOF = R!(46),
    UNSIZED = R!(47),
    VIRTUAL = R!(48),
    YIELD = R!(49),
    TRY = R!(50),
    UNION = R!(51),
    STATICLT = R!(52),
    //
    NUM = T!(53),
    LPAR = T!(54),
    RPAR = T!(55),
    END = T!(56),
    DEFAULT = T!(57),
    IDENTIFIER = T!(58),
    NOT = T!(59),
    EQUALITY = T!(60),
    REM = T!(61),
    OPASSIGN = T!(62),
    AMPERSAND = T!(63),
    AND = T!(64),
    STAR = T!(65),
    PLUS = T!(66),
    COMMA = T!(67),
    MINUS = T!(68),
    ARROW = T!(69),
    SGLPT = T!(70),
    DBLPT = T!(71),
    INRANGE = T!(72),
    DIV = T!(73),
    COLON = T!(74),
    SEMICOLON = T!(75),
    LT = T!(76),
    LTE = T!(77),
    SHIFTOP = T!(78),
    EQUAL = T!(79),
    MATCHARM = T!(80),
    GT = T!(81),
    GTE = T!(82),
    AT = T!(83),
    BITXOR = T!(84),
    VBAR = T!(85),
    BITOR = T!(86),
    OR = T!(87),
    INTERROGATION = T!(88),
    ERR = T!(89),
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

    static NORMAL_KEYWORD_TAGS: phf::Map<&'static str, Tag> = phf_map! {
        "as" => Tag::AS,
        "break" => Tag::BREAK,
        "const" => Tag::CONST,
        "continue" => Tag::CONTINUE,
        "crate" => Tag::CRATE,
        "else" => Tag::ELSE,
        "enum" => Tag::ENUM,
        "extern" => Tag::EXTERN,
        "false" => Tag::FALSE,
        "fn" => Tag::FN,
        "for" => Tag::FOR,
        "if" => Tag::IF,
        "impl" => Tag::IMPL,
        "in" => Tag::IN,
        "let" => Tag::LET,
        "loop" => Tag::LOOP,
        "match" => Tag::MATCH,
        "mod" => Tag::MOD,
        "move" => Tag::MOVE,
        "mut" => Tag::MUT,
        "pub" => Tag::PUB,
        "ref" => Tag::REF,
        "return" => Tag::RETURN,
        "self" => Tag::SELF,
        "Self" => Tag::SELFTYPE,
        "static" => Tag::STATIC,
        "struct" => Tag::STRUCT,
        "super" => Tag::SUPER,
        "trait" => Tag::TRAIT,
        "true" => Tag::TRUE,
        "type" => Tag::TYPE,
        "unsafe" => Tag::UNSAFE,
        "use" => Tag::USE,
        "where" => Tag::WHERE,
        "while" => Tag::WHILE,
        "async" => Tag::ASYNC,
        "await" => Tag::AWAIT,
        "dyn" => Tag::DYN,
    };

    static FUTURE_KEYWORD_TAGS: phf::Map<&'static str, Tag> = phf_map! {
        "abstract" => Tag::ABSTRACT,
        "become" => Tag::BECOME,
        "box" => Tag::BOX,
        "do" => Tag::DO,
        "final" => Tag::FINAL,
        "macro" => Tag::MACRO,
        "override" => Tag::OVERRIDE,
        "priv" => Tag::PRIV,
        "typeof" => Tag::TYPEOF,
        "unsized" => Tag::UNSIZED,
        "virtual" => Tag::VIRTUAL,
        "yield" => Tag::YIELD,
        "try" => Tag::TRY,
    };

    impl Tag {
        pub fn from_keyword(lexeme: &str, context: KeywordContext) -> Option<Self> {
            if context == KeywordContext::Lifetimes {
                if lexeme == "static" {
                    return Some(Tag::STATICLT);
                }
            }
            if context == KeywordContext::Union {
                if lexeme == "union" {
                    return Some(Tag::UNION);
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
