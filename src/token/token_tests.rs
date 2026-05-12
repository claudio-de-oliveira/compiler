#[cfg(test)]
mod tests {
    use crate::tags::{rust_tags::Tag};
    use crate::token::{Rust, Scanner};
    use crate::token::token::{AssignOp, Token, StringLiteralType, IntegerLiteralType, FloatLiteralType};

    #[test]
    fn test_symbols_token() {

        let mut expr = Rust::new(" ( ) # / ! % = && || & | ^ ? -> . .. ..= * + , - ; @ : _ += &= ^= |= /= *= %= -= >>= <<= ");

        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::LPAR);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::RPAR);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::END);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::DIV);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::NOT);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::REM);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::EQUAL);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::AND);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::OR);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::AMPERSAND);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::VBAR);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::BITXOR);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::INTERROGATION);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::ARROW);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::SGLPT);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::DBLPT);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::INRANGE);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::STAR);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::PLUS);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::COMMA);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::MINUS);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::SEMICOLON);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::AT);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::COLON);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::DEFAULT);
        //
        let token = expr.next_token();        assert_eq!(token.get_assign_op(), Some(AssignOp::AddAssign));
        let token = expr.next_token();        assert_eq!(token.get_assign_op(), Some(AssignOp::BitAndAssign));
        let token = expr.next_token();        assert_eq!(token.get_assign_op(), Some(AssignOp::BitXorAssign));
        let token = expr.next_token();        assert_eq!(token.get_assign_op(), Some(AssignOp::BitOrAssign));
        let token = expr.next_token();        assert_eq!(token.get_assign_op(), Some(AssignOp::DivAssign));
        let token = expr.next_token();        assert_eq!(token.get_assign_op(), Some(AssignOp::MulAssign));
        let token = expr.next_token();        assert_eq!(token.get_assign_op(), Some(AssignOp::RemAssign));
        let token = expr.next_token();        assert_eq!(token.get_assign_op(), Some(AssignOp::SubAssign));
        let token = expr.next_token();        assert_eq!(token.get_assign_op(), Some(AssignOp::ShrAssign));
        let token = expr.next_token();        assert_eq!(token.get_assign_op(), Some(AssignOp::ShlAssign));


    }

    #[test]
    fn test_reserved_words_tokens() {
        let mut expr = Rust::new(" as break const continue crate else enum extern false fn for if impl in let loop match mod move mut pub ref return self Self static struct super trait true type unsafe use where while async await dyn abstract become box do final macro override priv typeof unsized virtual yield try        ");

        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::AS);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::BREAK);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::CONST);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::CONTINUE);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::CRATE);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::ELSE);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::ENUM);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::EXTERN);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::FALSE);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::FN);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::FOR);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::IF);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::IMPL);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::IN);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::LET);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::LOOP);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::MATCH);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::MOD);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::MOVE);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::MUT);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::PUB);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::REF);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::RETURN);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::SELF);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::SELFTYPE);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::STATIC);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::STRUCT);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::SUPER);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::TRAIT);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::TRUE);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::TYPE);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::UNSAFE);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::USE);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::WHERE);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::WHILE);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::ASYNC);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::AWAIT);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::DYN);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::ABSTRACT);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::BECOME);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::BOX);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::DO);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::FINAL);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::MACRO);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::OVERRIDE);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::PRIV);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::TYPEOF);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::UNSIZED);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::VIRTUAL);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::YIELD);
        let token = expr.next_token();        assert_eq!(token.get_tag(), Tag::TRY);
    }

    #[test]
    fn test_char_tokens() {
        let mut expr = Rust::new(" '\\u{3b4}' 'a' '\\n' '\\r' '\\t' '\\\\' '\\\'' '\\\"' '\\u{41}' '\\u{1f980}' '_' '?' ' ' ");

        let token = expr.next_token();        assert!(token.get_tag() == Tag::CHARACTER && token.get_content() == "δ".into());
        let token = expr.next_token();        assert!(token.get_tag() == Tag::CHARACTER && token.get_content() == "a".into());
        let token = expr.next_token();        assert!(token.get_tag() == Tag::CHARACTER && token.get_content() == "\n".into());
        let token = expr.next_token();        assert!(token.get_tag() == Tag::CHARACTER && token.get_content() == "\r".into());
        let token = expr.next_token();        assert!(token.get_tag() == Tag::CHARACTER && token.get_content() == "\t".into());
        let token = expr.next_token();        assert!(token.get_tag() == Tag::CHARACTER && token.get_content() == "\\".into());
        let token = expr.next_token();        assert!(token.get_tag() == Tag::CHARACTER && token.get_content() == "'".into());
        let token = expr.next_token();        assert!(token.get_tag() == Tag::CHARACTER && token.get_content() == "\"".into());
        let token = expr.next_token();        assert!(token.get_tag() == Tag::CHARACTER && token.get_content() == "A".into());
        let token = expr.next_token();        assert!(token.get_tag() == Tag::CHARACTER && token.get_content() == "🦀".into());
        let token = expr.next_token();        assert!(token.get_tag() == Tag::CHARACTER && token.get_content() == "_".into());
        let token = expr.next_token();        assert!(token.get_tag() == Tag::CHARACTER && token.get_content() == "?".into());
    }

#[test]
    fn test_string_tokens() {
        let mut expr = Rust::new(" b r br   b\"A\\u{1f980}BC\\n\" r\"A\\u{1f980}BC\\n\" br\"A\\u{1f980}BC\\n\"   r###\"ABC\"### br###\"ABC\"### \"ABC\" ");

        let token = expr.next_token();        assert!(token.get_tag() == Tag::IDENTIFIER && token.get_content() == "b".into());
        let token = expr.next_token();        assert!(token.get_tag() == Tag::IDENTIFIER && token.get_content() == "r".into());
        let token = expr.next_token();        assert!(token.get_tag() == Tag::IDENTIFIER && token.get_content() == "br".into());
        let token = expr.next_token();        assert!(token.get_tag() == Tag::STRING && token.get_content() == "ABC".into());
        let token = expr.next_token();        assert!(token.get_tag() == Tag::STRING && token.get_content() == "ABC".into());
        let token = expr.next_token();        assert!(token.get_tag() == Tag::STRING && token.get_content() == "ABC".into());
        let token = expr.next_token();        assert!(token.get_tag() == Tag::STRING && token.get_content() == "ABC".into());
        let token = expr.next_token();        assert!(token.get_tag() == Tag::STRING && token.get_content() == "ABC".into());
    }

    #[test]
    fn test_peek_number_decimal_integers() {
        // Testa inteiros decimais básicos
        let mut expr = Rust::new("0");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 1, 1, "0".into(), IntegerLiteralType::ISIZE));

        let mut expr = Rust::new("1");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 1, 1, "1".into(), IntegerLiteralType::ISIZE));

        let mut expr = Rust::new("123");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 1, 1, "123".into(), IntegerLiteralType::ISIZE));

        let mut expr = Rust::new("999");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 1, 1, "999".into(), IntegerLiteralType::ISIZE));
    }

    #[test]
    fn test_peek_number_decimal_with_underscores() {
        // Testa inteiros decimais com separadores de dígitos
        let mut expr = Rust::new("1_000");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 1, 1, "1000".into(), IntegerLiteralType::ISIZE));

        let mut expr = Rust::new("1_000_000");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 1, 1, "1000000".into(), IntegerLiteralType::ISIZE));

        let mut expr = Rust::new("123_456_789");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 1, 1, "123456789".into(), IntegerLiteralType::ISIZE));
    }

    #[test]
    fn test_peek_number_binary() {
        // Testa inteiros binários
        let mut expr = Rust::new("0b101");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 1, 1, "101".into(), IntegerLiteralType::ISIZE));

        let mut expr = Rust::new("0b1010");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 1, 1, "1010".into(), IntegerLiteralType::ISIZE));

        let mut expr = Rust::new("0B11111111");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 1, 1, "11111111".into(), IntegerLiteralType::ISIZE));
    }

    #[test]
    fn test_peek_number_binary_with_underscores() {
        // Testa inteiros binários com separadores
        let mut expr = Rust::new("0b1010_0001");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 1, 1, "10100001".into(), IntegerLiteralType::ISIZE));

        let mut expr = Rust::new("0b1111_0000_1010_0101");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 1, 1, "1111000010100101".into(), IntegerLiteralType::ISIZE));
    }

    #[test]
    fn test_peek_number_octal() {
        // Testa inteiros octais
        let mut expr = Rust::new("0o77");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 1, 1, "77".into(), IntegerLiteralType::ISIZE));

        let mut expr = Rust::new("0o123");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 1, 1, "123".into(), IntegerLiteralType::ISIZE));

        let mut expr = Rust::new("0O755");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 1, 1, "755".into(), IntegerLiteralType::ISIZE));
    }

    #[test]
    fn test_peek_number_octal_with_underscores() {
        // Testa inteiros octais com separadores
        let mut expr = Rust::new("0o123_456");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 1, 1, "123456".into(), IntegerLiteralType::ISIZE));

        let mut expr = Rust::new("0o7777_7777");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 1, 1, "77777777".into(), IntegerLiteralType::ISIZE));
    }

    #[test]
    fn test_peek_number_hexadecimal() {
        // Testa inteiros hexadecimais
        let mut expr = Rust::new("0xFF");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 1, 1, "FF".into(), IntegerLiteralType::ISIZE));

        let mut expr = Rust::new("0x10");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 1, 1, "10".into(), IntegerLiteralType::ISIZE));

        let mut expr = Rust::new("0XABCD");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 1, 1, "ABCD".into(), IntegerLiteralType::ISIZE));
    }

    #[test]
    fn test_peek_number_hexadecimal_with_underscores() {
        // Testa inteiros hexadecimais com separadores
        let mut expr = Rust::new("0xDEAD_BEEF");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 1, 1, "DEADBEEF".into(), IntegerLiteralType::ISIZE));

        let mut expr = Rust::new("0x1234_5678");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 1, 1, "12345678".into(), IntegerLiteralType::ISIZE));
    }

    #[test]
    fn test_peek_number_i8_suffix() {
        let mut expr = Rust::new("0i8");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 1, 1, "0".into(), IntegerLiteralType::I8));

        let mut expr = Rust::new("127i8");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 1, 1, "127".into(), IntegerLiteralType::I8));
    }

    #[test]
    fn test_peek_number_u8_suffix() {
        let mut expr = Rust::new("0u8");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 1, 1, "0".into(), IntegerLiteralType::U8));

        let mut expr = Rust::new("255u8");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 1, 1, "255".into(), IntegerLiteralType::U8));
    }

    #[test]
    fn test_peek_number_i16_suffix() {
        let mut expr = Rust::new("0i16");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 1, 1, "0".into(), IntegerLiteralType::I16));

        let mut expr = Rust::new("32767i16");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 1, 1, "32767".into(), IntegerLiteralType::I16));
    }

    #[test]
    fn test_peek_number_u16_suffix() {
        let mut expr = Rust::new("0u16");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 1, 1, "0".into(), IntegerLiteralType::U16));

        let mut expr = Rust::new("65535u16");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 1, 1, "65535".into(), IntegerLiteralType::U16));
    }

    #[test]
    fn test_peek_number_i32_suffix() {
        let mut expr = Rust::new("0i32");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 1, 1, "0".into(), IntegerLiteralType::I32));

        let mut expr = Rust::new("2147483647i32");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 1, 1, "2147483647".into(), IntegerLiteralType::I32));
    }

    #[test]
    fn test_peek_number_u32_suffix() {
        let mut expr = Rust::new("0u32");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 1, 1, "0".into(), IntegerLiteralType::U32));

        let mut expr = Rust::new("4294967295u32");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 1, 1, "4294967295".into(), IntegerLiteralType::U32));
    }

    #[test]
    fn test_peek_number_i64_suffix() {
        let mut expr = Rust::new("0i64");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 1, 1, "0".into(), IntegerLiteralType::I64));

        let mut expr = Rust::new("9223372036854775807i64");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 1, 1, "9223372036854775807".into(), IntegerLiteralType::I64));
    }

    #[test]
    fn test_peek_number_u64_suffix() {
        let mut expr = Rust::new("0u64");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 1, 1, "0".into(), IntegerLiteralType::U64));

        let mut expr = Rust::new("18446744073709551615u64");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 1, 1, "18446744073709551615".into(), IntegerLiteralType::U64));
    }

    #[test]
    fn test_peek_number_i128_suffix() {
        let mut expr = Rust::new("0i128");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 1, 1, "0".into(), IntegerLiteralType::I128));

        let mut expr = Rust::new("170141183460469231731687303715884105727i128");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 1, 1, "170141183460469231731687303715884105727".into(), IntegerLiteralType::I128));
    }

    #[test]
    fn test_peek_number_u128_suffix() {
        let mut expr = Rust::new("0u128");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 1, 1, "0".into(), IntegerLiteralType::U128));

        let mut expr = Rust::new("340282366920938463463374607431768211455u128");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 1, 1, "340282366920938463463374607431768211455".into(), IntegerLiteralType::U128));
    }

    #[test]
    fn test_peek_number_isize_suffix() {
        let mut expr = Rust::new("0isize");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 1, 1, "0".into(), IntegerLiteralType::ISIZE));

        let mut expr = Rust::new("42isize");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 1, 1, "42".into(), IntegerLiteralType::ISIZE));
    }

    #[test]
    fn test_peek_number_usize_suffix() {
        let mut expr = Rust::new("0usize");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 1, 1, "0".into(), IntegerLiteralType::USIZE));

        let mut expr = Rust::new("42usize");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 1, 1, "42".into(), IntegerLiteralType::USIZE));
    }

    #[test]
    fn test_peek_number_binary_with_i_suffix() {
        let mut expr = Rust::new("0b101i32");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 1, 1, "101".into(), IntegerLiteralType::I32));

        let mut expr = Rust::new("0b1111_0000u8");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 1, 1, "11110000".into(), IntegerLiteralType::U8));
    }

    #[test]
    fn test_peek_number_octal_with_u_suffix() {
        let mut expr = Rust::new("0o755u32");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 1, 1, "755".into(), IntegerLiteralType::U32));

        let mut expr = Rust::new("0o123i64");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 1, 1, "123".into(), IntegerLiteralType::I64));
    }

    #[test]
    fn test_peek_number_hex_with_suffix() {
        let mut expr = Rust::new("0xFFu8");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 1, 1, "FF".into(), IntegerLiteralType::U8));

        let mut expr = Rust::new("0xDEAD_BEEFi32");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 1, 1, "DEADBEEF".into(), IntegerLiteralType::I32));
    }

    #[test]
    fn test_peek_number_float_basic() {
        // Testa floats com ponto decimal
        let mut expr = Rust::new("0.0");
        let token = expr.peek_number();
        assert_eq!(token, Token::Float(Tag::FLOAT, 1, 1, "0.0".into(), FloatLiteralType::F64));

        let mut expr = Rust::new("1.5");
        let token = expr.peek_number();
        assert_eq!(token, Token::Float(Tag::FLOAT, 1, 1, "1.5".into(), FloatLiteralType::F64));

        let mut expr = Rust::new("3.14159");
        let token = expr.peek_number();
        assert_eq!(token, Token::Float(Tag::FLOAT, 1, 1, "3.14159".into(), FloatLiteralType::F64));
    }

    #[test]
    fn test_peek_number_float_with_underscores() {
        let mut expr = Rust::new("1_000.5");
        let token = expr.peek_number();
        assert_eq!(token, Token::Float(Tag::FLOAT, 1, 1, "1000.5".into(), FloatLiteralType::F64));

        let mut expr = Rust::new("1.5_000");
        let token = expr.peek_number();
        assert_eq!(token, Token::Float(Tag::FLOAT, 1, 1, "1.5000".into(), FloatLiteralType::F64));

        let mut expr = Rust::new("123_456.789_012");
        let token = expr.peek_number();
        assert_eq!(token, Token::Float(Tag::FLOAT, 1, 1, "123456.789012".into(), FloatLiteralType::F64));
    }

    #[test]
    fn test_peek_number_float_scientific_notation() {
        // Testa floats com notação científica
        let mut expr = Rust::new("1e10");
        let token = expr.peek_number();
        assert_eq!(token, Token::Float(Tag::FLOAT, 1, 1, "1e10".into(), FloatLiteralType::F64));

        let mut expr = Rust::new("1.5e-10");
        let token = expr.peek_number();
        assert_eq!(token, Token::Float(Tag::FLOAT, 1, 1, "1.5e-10".into(), FloatLiteralType::F64));

        let mut expr = Rust::new("2.5E+5");
        let token = expr.peek_number();
        assert_eq!(token, Token::Float(Tag::FLOAT, 1, 1, "2.5E+5".into(), FloatLiteralType::F64));
    }

    #[test]
    fn test_peek_number_float_scientific_with_underscores() {
        let mut expr = Rust::new("1_000e10");
        let token = expr.peek_number();
        assert_eq!(token, Token::Float(Tag::FLOAT, 1, 1, "1000e10".into(), FloatLiteralType::F64));

        let mut expr = Rust::new("1.5e1_0");
        let token = expr.peek_number();
        assert_eq!(token, Token::Float(Tag::FLOAT, 1, 1, "1.5e10".into(), FloatLiteralType::F64));
    }

    #[test]
    fn test_peek_number_f32_suffix() {
        let mut expr = Rust::new("1.5f32");
        let token = expr.peek_number();
        assert_eq!(token, Token::Float(Tag::FLOAT, 1, 1, "1.5f32".into(), FloatLiteralType::F32));

        let mut expr = Rust::new("0.0f32");
        let token = expr.peek_number();
        assert_eq!(token, Token::Float(Tag::FLOAT, 1, 1, "0.0f32".into(), FloatLiteralType::F32));

        let mut expr = Rust::new("3.14f32");
        let token = expr.peek_number();
        assert_eq!(token, Token::Float(Tag::FLOAT, 1, 1, "3.14f32".into(), FloatLiteralType::F32));
    }

    #[test]
    fn test_peek_number_f64_suffix() {
        let mut expr = Rust::new("1.5f64");
        let token = expr.peek_number();
        assert_eq!(token, Token::Float(Tag::FLOAT, 1, 1, "1.5f64".into(), FloatLiteralType::F64));

        let mut expr = Rust::new("0.0f64");
        let token = expr.peek_number();
        assert_eq!(token, Token::Float(Tag::FLOAT, 1, 1, "0.0f64".into(), FloatLiteralType::F64));

        let mut expr = Rust::new("2.71828f64");
        let token = expr.peek_number();
        assert_eq!(token, Token::Float(Tag::FLOAT, 1, 1, "2.71828f64".into(), FloatLiteralType::F64));
    }

    #[test]
    fn test_peek_number_float_scientific_with_f32() {
        let mut expr = Rust::new("1.5e10f32");
        let token = expr.peek_number();
        assert_eq!(token, Token::Float(Tag::FLOAT, 1, 1, "1.5e10f32".into(), FloatLiteralType::F32));

        let mut expr = Rust::new("2.5E-5f32");
        let token = expr.peek_number();
        assert_eq!(token, Token::Float(Tag::FLOAT, 1, 1, "2.5E-5f32".into(), FloatLiteralType::F32));
    }

    #[test]
    fn test_peek_number_float_scientific_with_f64() {
        let mut expr = Rust::new("1.5e10f64");
        let token = expr.peek_number();
        assert_eq!(token, Token::Float(Tag::FLOAT, 1, 1, "1.5e10f64".into(), FloatLiteralType::F64));

        let mut expr = Rust::new("3.14E+2f64");
        let token = expr.peek_number();
        assert_eq!(token, Token::Float(Tag::FLOAT, 1, 1, "3.14E+2f64".into(), FloatLiteralType::F64));
    }

    #[test]
    fn test_peek_number_float_zero_variants() {
        let mut expr = Rust::new("0.");
        let token = expr.peek_number();
        assert_eq!(token, Token::Float(Tag::FLOAT, 1, 1, "0.".into(), FloatLiteralType::F64));

        let mut expr = Rust::new("0.f32");
        let token = expr.peek_number();
        assert_eq!(token, Token::Float(Tag::FLOAT, 1, 1, "0.f32".into(), FloatLiteralType::F32));

        let mut expr = Rust::new("0.f64");
        let token = expr.peek_number();
        assert_eq!(token, Token::Float(Tag::FLOAT, 1, 1, "0.f64".into(), FloatLiteralType::F64));
    }

    #[test]
    fn test_peek_number_complex_examples() {
        // Exemplos mais complexos
        let mut expr = Rust::new("1_000_000i64");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 1, 1, "1000000".into(), IntegerLiteralType::I64));

        let mut expr = Rust::new("0xFF_FF_FFu32");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 1, 1, "FFFFFF".into(), IntegerLiteralType::U32));

        let mut expr = Rust::new("0b1111_0000_1010_0101i16");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 1, 1, "1111000010100101".into(), IntegerLiteralType::I16));

        let mut expr = Rust::new("1_234.567_890e-5f32");
        let token = expr.peek_number();
        assert_eq!(token, Token::Float(Tag::FLOAT, 1, 1, "1234.567890e-5f32".into(), FloatLiteralType::F32));
    }
}

