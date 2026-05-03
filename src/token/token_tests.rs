#[cfg(test)]
mod tests {
    use crate::tags::{rust_tags::Tag};
    use crate::token::{Rust, Scanner};
    use crate::token::token::{AssignOp, Token};

    #[test]
    fn test_simple_symbols_token() {

        let mut expr = Rust::new(" ( ) # / ! % = && || & | ^ ? -> . .. ..= * + , - ; @ : _ += &= ^= |= /= *= %= -= >>= <<= ");

        let token = expr.next_token();        assert_eq!(token, Token::LPar(Tag::LPAR));
        let token = expr.next_token();        assert_eq!(token, Token::RPar(Tag::RPAR));
        let token = expr.next_token();        assert_eq!(token, Token::EndMark(Tag::END));
        let token = expr.next_token();        assert_eq!(token, Token::Division(Tag::DIV));
        let token = expr.next_token();        assert_eq!(token, Token::Not(Tag::NOT));
        let token = expr.next_token();        assert_eq!(token, Token::Remainder(Tag::REM));
        let token = expr.next_token();        assert_eq!(token, Token::EqualSymbol(Tag::EQUAL));
        let token = expr.next_token();        assert_eq!(token, Token::LogicalAnd(Tag::AND));
        let token = expr.next_token();        assert_eq!(token, Token::LogicalOr(Tag::OR));
        let token = expr.next_token();        assert_eq!(token, Token::AmpersandSymbol(Tag::AMPERSAND));
        let token = expr.next_token();        assert_eq!(token, Token::VerticalBarSymbol(Tag::VBAR));
        let token = expr.next_token();        assert_eq!(token, Token::BitwiseXor(Tag::BITXOR));
        let token = expr.next_token();        assert_eq!(token, Token::InterrogationSymbol(Tag::INTERROGATION));
        let token = expr.next_token();        assert_eq!(token, Token::ReturnType(Tag::ARROW));
        let token = expr.next_token();        assert_eq!(token, Token::SglPtSymbol(Tag::SGLPT));
        let token = expr.next_token();        assert_eq!(token, Token::DblPtSymbol(Tag::DBLPT));
        let token = expr.next_token();        assert_eq!(token, Token::InclusiveRange(Tag::INRANGE));
        let token = expr.next_token();        assert_eq!(token, Token::StarSymbol(Tag::STAR));
        let token = expr.next_token();        assert_eq!(token, Token::PlusSymbol(Tag::PLUS));
        let token = expr.next_token();        assert_eq!(token, Token::CommaSymbol(Tag::COMMA));
        let token = expr.next_token();        assert_eq!(token, Token::MinusSymbol(Tag::MINUS));
        let token = expr.next_token();        assert_eq!(token, Token::SemicolonSymbol(Tag::SEMICOLON));
        let token = expr.next_token();        assert_eq!(token, Token::AtSymbol(Tag::AT));
        let token = expr.next_token();        assert_eq!(token, Token::ColonSymbol(Tag::COLON));
        let token = expr.next_token();        assert_eq!(token, Token::DefaultPattern(Tag::DEFAULT));
        //
        let token = expr.next_token();        assert_eq!(token, Token::OpAssignment(Tag::OPASSIGN, AssignOp::AddAssign));
        let token = expr.next_token();        assert_eq!(token, Token::OpAssignment(Tag::OPASSIGN, AssignOp::BitAndAssign));
        let token = expr.next_token();        assert_eq!(token, Token::OpAssignment(Tag::OPASSIGN, AssignOp::BitXorAssign));
        let token = expr.next_token();        assert_eq!(token, Token::OpAssignment(Tag::OPASSIGN, AssignOp::BitOrAssign));
        let token = expr.next_token();        assert_eq!(token, Token::OpAssignment(Tag::OPASSIGN, AssignOp::DivAssign));
        let token = expr.next_token();        assert_eq!(token, Token::OpAssignment(Tag::OPASSIGN, AssignOp::MulAssign));
        let token = expr.next_token();        assert_eq!(token, Token::OpAssignment(Tag::OPASSIGN, AssignOp::RemAssign));
        let token = expr.next_token();        assert_eq!(token, Token::OpAssignment(Tag::OPASSIGN, AssignOp::SubAssign));
        let token = expr.next_token();        assert_eq!(token, Token::OpAssignment(Tag::OPASSIGN, AssignOp::ShrAssign));
        let token = expr.next_token();        assert_eq!(token, Token::OpAssignment(Tag::OPASSIGN, AssignOp::ShlAssign));


    }

    #[test]
    fn test_reserved_words_tokens() {
        let mut expr = Rust::new(" as break const continue crate else enum extern false fn for if impl in let loop match mod move mut pub ref return self Self static struct super trait true type unsafe use where while async await dyn abstract become box do final macro override priv typeof unsized virtual yield try        ");

        let token = expr.next_token();        assert_eq!(token, Token::Keyword(Tag::AS, "as".into()));
        let token = expr.next_token();        assert_eq!(token, Token::Keyword(Tag::BREAK, "break".into()));
        let token = expr.next_token();        assert_eq!(token, Token::Keyword(Tag::CONST, "const".into()));
        let token = expr.next_token();        assert_eq!(token, Token::Keyword(Tag::CONTINUE, "continue".into()));
        let token = expr.next_token();        assert_eq!(token, Token::Keyword(Tag::CRATE, "crate".into()));
        let token = expr.next_token();        assert_eq!(token, Token::Keyword(Tag::ELSE, "else".into()));
        let token = expr.next_token();        assert_eq!(token, Token::Keyword(Tag::ENUM, "enum".into()));
        let token = expr.next_token();        assert_eq!(token, Token::Keyword(Tag::EXTERN, "extern".into()));
        let token = expr.next_token();        assert_eq!(token, Token::Keyword(Tag::FALSE, "false".into()));
        let token = expr.next_token();        assert_eq!(token, Token::Keyword(Tag::FN, "fn".into()));
        let token = expr.next_token();        assert_eq!(token, Token::Keyword(Tag::FOR, "for".into()));
        let token = expr.next_token();        assert_eq!(token, Token::Keyword(Tag::IF, "if".into()));
        let token = expr.next_token();        assert_eq!(token, Token::Keyword(Tag::IMPL, "impl".into()));
        let token = expr.next_token();        assert_eq!(token, Token::Keyword(Tag::IN, "in".into()));
        let token = expr.next_token();        assert_eq!(token, Token::Keyword(Tag::LET, "let".into()));
        let token = expr.next_token();        assert_eq!(token, Token::Keyword(Tag::LOOP, "loop".into()));
        let token = expr.next_token();        assert_eq!(token, Token::Keyword(Tag::MATCH, "match".into()));
        let token = expr.next_token();        assert_eq!(token, Token::Keyword(Tag::MOD, "mod".into()));
        let token = expr.next_token();        assert_eq!(token, Token::Keyword(Tag::MOVE, "move".into()));
        let token = expr.next_token();        assert_eq!(token, Token::Keyword(Tag::MUT, "mut".into()));
        let token = expr.next_token();        assert_eq!(token, Token::Keyword(Tag::PUB, "pub".into()));
        let token = expr.next_token();        assert_eq!(token, Token::Keyword(Tag::REF, "ref".into()));
        let token = expr.next_token();        assert_eq!(token, Token::Keyword(Tag::RETURN, "return".into()));
        let token = expr.next_token();        assert_eq!(token, Token::Keyword(Tag::SELF, "self".into()));
        let token = expr.next_token();        assert_eq!(token, Token::Keyword(Tag::SELFTYPE, "Self".into()));
        let token = expr.next_token();        assert_eq!(token, Token::Keyword(Tag::STATIC, "static".into()));
        let token = expr.next_token();        assert_eq!(token, Token::Keyword(Tag::STRUCT, "struct".into()));
        let token = expr.next_token();        assert_eq!(token, Token::Keyword(Tag::SUPER, "super".into()));
        let token = expr.next_token();        assert_eq!(token, Token::Keyword(Tag::TRAIT, "trait".into()));
        let token = expr.next_token();        assert_eq!(token, Token::Keyword(Tag::TRUE, "true".into()));
        let token = expr.next_token();        assert_eq!(token, Token::Keyword(Tag::TYPE, "type".into()));
        let token = expr.next_token();        assert_eq!(token, Token::Keyword(Tag::UNSAFE, "unsafe".into()));
        let token = expr.next_token();        assert_eq!(token, Token::Keyword(Tag::USE, "use".into()));
        let token = expr.next_token();        assert_eq!(token, Token::Keyword(Tag::WHERE, "where".into()));
        let token = expr.next_token();        assert_eq!(token, Token::Keyword(Tag::WHILE, "while".into()));
        let token = expr.next_token();        assert_eq!(token, Token::Keyword(Tag::ASYNC, "async".into()));
        let token = expr.next_token();        assert_eq!(token, Token::Keyword(Tag::AWAIT, "await".into()));
        let token = expr.next_token();        assert_eq!(token, Token::Keyword(Tag::DYN, "dyn".into()));
        let token = expr.next_token();        assert_eq!(token, Token::Keyword(Tag::ABSTRACT, "abstract".into()));
        let token = expr.next_token();        assert_eq!(token, Token::Keyword(Tag::BECOME, "become".into()));
        let token = expr.next_token();        assert_eq!(token, Token::Keyword(Tag::BOX, "box".into()));
        let token = expr.next_token();        assert_eq!(token, Token::Keyword(Tag::DO, "do".into()));
        let token = expr.next_token();        assert_eq!(token, Token::Keyword(Tag::FINAL, "final".into()));
        let token = expr.next_token();        assert_eq!(token, Token::Keyword(Tag::MACRO, "macro".into()));
        let token = expr.next_token();        assert_eq!(token, Token::Keyword(Tag::OVERRIDE, "override".into()));
        let token = expr.next_token();        assert_eq!(token, Token::Keyword(Tag::PRIV, "priv".into()));
        let token = expr.next_token();        assert_eq!(token, Token::Keyword(Tag::TYPEOF, "typeof".into()));
        let token = expr.next_token();        assert_eq!(token, Token::Keyword(Tag::UNSIZED, "unsized".into()));
        let token = expr.next_token();        assert_eq!(token, Token::Keyword(Tag::VIRTUAL, "virtual".into()));
        let token = expr.next_token();        assert_eq!(token, Token::Keyword(Tag::YIELD, "yield".into()));
        let token = expr.next_token();        assert_eq!(token, Token::Keyword(Tag::TRY, "try".into()));
    }

    #[test]
    fn test_characters_tokens() {
        let mut expr = Rust::new(" '\\u{3b4}' 'a' '\\n' '\\r' '\\t' '\\\\' '\\\'' '\\\"' '\\u{41}' '\\u{1f980}' '_' '?' ' ' ");

        let token = expr.next_token();        assert_eq!(token, Token::Character(Tag::CHARACTER, 'δ'));
        let token = expr.next_token();        assert_eq!(token, Token::Character(Tag::CHARACTER, 'a'));
        let token = expr.next_token();        assert_eq!(token, Token::Character(Tag::CHARACTER, '\n'));
        let token = expr.next_token();        assert_eq!(token, Token::Character(Tag::CHARACTER, '\r'));
        let token = expr.next_token();        assert_eq!(token, Token::Character(Tag::CHARACTER, '\t'));
        let token = expr.next_token();        assert_eq!(token, Token::Character(Tag::CHARACTER, '\\'));
        let token = expr.next_token();        assert_eq!(token, Token::Character(Tag::CHARACTER, '\''));
        let token = expr.next_token();        assert_eq!(token, Token::Character(Tag::CHARACTER, '\"'));
        let token = expr.next_token();        assert_eq!(token, Token::Character(Tag::CHARACTER, 'A'));
        let token = expr.next_token();        assert_eq!(token, Token::Character(Tag::CHARACTER, '🦀'));
        let token = expr.next_token();        assert_eq!(token, Token::Character(Tag::CHARACTER, '_'));
        let token = expr.next_token();        assert_eq!(token, Token::Character(Tag::CHARACTER, '?'));
    }

#[test]
    fn test_string_literals_tokens() {
        let mut expr = Rust::new(" b r br   b\"\" r\"\" br\"\"   r###\"\"### br###\"\"### ");

        let token = expr.next_token();        assert_eq!(token, Token::Identifier(Tag::IDENTIFIER, "b".into()));
        let token = expr.next_token();        assert_eq!(token, Token::Identifier(Tag::IDENTIFIER, "r".into()));
        let token = expr.next_token();        assert_eq!(token, Token::Identifier(Tag::IDENTIFIER, "br".into()));
        let token = expr.next_token();        assert_eq!(token, Token::StringLiteral(Tag::STRING, "".into()));
        let token = expr.next_token();        assert_eq!(token, Token::StringLiteral(Tag::STRING, "".into()));
        let token = expr.next_token();        assert_eq!(token, Token::StringLiteral(Tag::STRING, "".into()));
        let token = expr.next_token();        assert_eq!(token, Token::StringLiteral(Tag::STRING, "".into()));
        let token = expr.next_token();        assert_eq!(token, Token::StringLiteral(Tag::STRING, "".into()));
    }
}
