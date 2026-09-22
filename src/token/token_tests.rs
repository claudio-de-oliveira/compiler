#[cfg(test)]
mod tests {
    use crate::tags::{rust_tags::Tag};
    use crate::token::{Rust, Scanner};
    use crate::token::token::{AssignOp, Token, IntegerLiteralType, FloatLiteralType, StringLiteralType};

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
        match expr.next_token() {
            Token::OpAssignment(Tag::OPASSIGN, _, _, assign_op) => assert_eq!(assign_op, AssignOp::AddAssign),
            _ => panic!("Esperava OpAssignment, mas recebeu algo diferente"),
        };
        match expr.next_token() {
            Token::OpAssignment(Tag::OPASSIGN, _, _, assign_op) => assert_eq!(assign_op, AssignOp::BitAndAssign),
            _ => panic!("Esperava OpAssignment, mas recebeu algo diferente"),
        };
        match expr.next_token() {
            Token::OpAssignment(Tag::OPASSIGN, _, _, assign_op) => assert_eq!(assign_op, AssignOp::BitXorAssign),
            _ => panic!("Esperava OpAssignment, mas recebeu algo diferente"),
        };
        match expr.next_token() {
            Token::OpAssignment(Tag::OPASSIGN, _, _, assign_op) => assert_eq!(assign_op, AssignOp::BitOrAssign),
            _ => panic!("Esperava OpAssignment, mas recebeu algo diferente"),
        };
        match expr.next_token() {
            Token::OpAssignment(Tag::OPASSIGN, _, _, assign_op) => assert_eq!(assign_op, AssignOp::DivAssign),
            _ => panic!("Esperava OpAssignment, mas recebeu algo diferente"),
        };
        match expr.next_token() {
            Token::OpAssignment(Tag::OPASSIGN, _, _, assign_op) => assert_eq!(assign_op, AssignOp::MulAssign),
            _ => panic!("Esperava OpAssignment, mas recebeu algo diferente"),
        };
        match expr.next_token() {
            Token::OpAssignment(Tag::OPASSIGN, _, _, assign_op) => assert_eq!(assign_op, AssignOp::RemAssign),
            _ => panic!("Esperava OpAssignment, mas recebeu algo diferente"),
        };
        match expr.next_token() {
            Token::OpAssignment(Tag::OPASSIGN, _, _, assign_op) => assert_eq!(assign_op, AssignOp::SubAssign),
            _ => panic!("Esperava OpAssignment, mas recebeu algo diferente"),
        };
        match expr.next_token() {
            Token::OpAssignment(Tag::OPASSIGN, _, _, assign_op) => assert_eq!(assign_op, AssignOp::ShrAssign),
            _ => panic!("Esperava OpAssignment, mas recebeu algo diferente"),
        };
        match expr.next_token() {
            Token::OpAssignment(Tag::OPASSIGN, _, _, assign_op) => assert_eq!(assign_op, AssignOp::ShlAssign),
            _ => panic!("Esperava OpAssignment, mas recebeu algo diferente"),
        };
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
        let mut expr = Rust::new(" 'a' '\\u{3b4}' '\\n' '\\r' '\\t' '\\\\' '\\\'' '\\\"' '\\u{41}' '\\u{1f980}' '_' '?' ' ' ");

        let token = expr.next_token();        assert_eq!(token, Token::Character(Tag::CHARACTER, 0, 4, 'a'));
        let token = expr.next_token();        assert_eq!(token, Token::Character(Tag::CHARACTER, 0, 14, 'δ'));
        let token = expr.next_token();        assert_eq!(token, Token::Character(Tag::CHARACTER, 0, 19, '\n'));
        let token = expr.next_token();        assert_eq!(token, Token::Character(Tag::CHARACTER, 0, 24, '\r'));
        let token = expr.next_token();        assert_eq!(token, Token::Character(Tag::CHARACTER, 0, 29, '\t'));
        let token = expr.next_token();        assert_eq!(token, Token::Character(Tag::CHARACTER, 0, 34, '\\'));
        let token = expr.next_token();        assert_eq!(token, Token::Character(Tag::CHARACTER, 0, 39, '\''));
        let token = expr.next_token();        assert_eq!(token, Token::Character(Tag::CHARACTER, 0, 44, '\"'));
        let token = expr.next_token();        assert_eq!(token, Token::Character(Tag::CHARACTER, 0, 53, 'A'));
        let token = expr.next_token();        assert_eq!(token, Token::Character(Tag::CHARACTER, 0, 65, '🦀'));
        let token = expr.next_token();        assert_eq!(token, Token::Character(Tag::CHARACTER, 0, 69, '_'));
        let token = expr.next_token();        assert_eq!(token, Token::Character(Tag::CHARACTER, 0, 73, '?'));
    }

    // #[test]
    // fn test_string_tokens() {
    //     // let mut expr = Rust::new(" b r br   b\"A\\u{1f980}BC\\n\" r\"A\\u{1f980}BC\\n\" br\"A\\u{1f980}BC\\n\"   r###\"ABC\"### br###\"ABC\"### \"ABC\" ");
    //     // match expr.next_token() {
    //     //     Token::Identifier(Tag::IDENTIFIER, _, _, id) => assert_eq!(id, "b"),
    //     //     _ => panic!("Esperava Identifier, mas recebeu algo diferente"),
    //     // };

    //     // let mut expr = Rust::new(" r br   b\"A\\u{1f980}BC\\n\" r\"A\\u{1f980}BC\\n\" br\"A\\u{1f980}BC\\n\"   r###\"ABC\"### br###\"ABC\"### \"ABC\" ");
    //     // match expr.next_token() {
    //     //     Token::Identifier(Tag::IDENTIFIER, _, _, id) => assert_eq!(id, "r"),
    //     //     _ => panic!("Esperava Identifier, mas recebeu algo diferente"),
    //     // };

    //     // let mut expr = Rust::new("  br   b\"A\\u{1f980}BC\\n\" r\"A\\u{1f980}BC\\n\" br\"A\\u{1f980}BC\\n\"   r###\"ABC\"### br###\"ABC\"### \"ABC\" ");
    //     // match expr.next_token() {
    //     //     Token::Identifier(Tag::IDENTIFIER, _, _, id) => assert_eq!(id, "br"),
    //     //     _ => panic!("Esperava Identifier, mas recebeu algo diferente"),
    //     // };

    //     let mut expr = Rust::new("  b\"A\\u{1f980}BC\\n\" r\"A\\u{1f980}BC\\n\" br\"A\\u{1f980}BC\\n\"   r###\"ABC\"### br###\"ABC\"### \"ABC\" ");
    //     match expr.next_token() {
    //         Token::StringLiteral(Tag::STRING, _, _, tp, s) => {
    //             assert_eq!(tp, StringLiteralType::ByteString);
    //             assert_eq!(s, String::from_utf8_lossy(b"A\xF0\x9F\xA6\x80BC\\n").into_owned());
    //         }
    //         _ => panic!("Esperava StringLiteral, mas recebeu algo diferente")
    //     };

    //     // let mut expr = Rust::new("  r\"A\\u{1f980}BC\\n\" br\"A\\u{1f980}BC\\n\"   r###\"ABC\"### br###\"ABC\"### \"ABC\" ");
    //     // match expr.next_token() {
    //     //     Token::StringLiteral(Tag::STRING, _, _, tp, s) => {
    //     //         assert_eq!(tp, StringLiteralType::Raw(0));
    //     //         assert_eq!(s, "A🦀BC\n");
    //     //     }
    //     //     _ => panic!("Esperava StringLiteral, mas recebeu algo diferente"),
    //     // };

    //     // let mut expr = Rust::new("  br\"A\\u{1f980}BC\\n\"   r###\"ABC\"### br###\"ABC\"### \"ABC\" ");
    //     // match expr.next_token() {
    //     //     Token::StringLiteral(Tag::STRING, _, _, tp, s) => {
    //     //         assert_eq!(tp, StringLiteralType::RawByte(0));
    //     //         assert_eq!(s, "A🦀BC\n");
    //     //     }
    //     //     _ => panic!("Esperava StringLiteral, mas recebeu algo diferente"),
    //     // };

    //     // let mut expr = Rust::new("  r###\"ABC\"### br###\"ABC\"### \"ABC\" ");
    //     // match expr.next_token() {
    //     //     Token::StringLiteral(Tag::STRING, _, _, tp, s) => {
    //     //         assert_eq!(tp, StringLiteralType::Raw(3));
    //     //         assert_eq!(s, "ABC");
    //     //     }
    //     //     _ => panic!("Esperava StringLiteral, mas recebeu algo diferente"),
    //     // };

    //     // let mut expr = Rust::new("   br###\"ABC\"### \"ABC\" ");
    //     // match expr.next_token() {
    //     //     Token::StringLiteral(Tag::STRING, _, _, tp, s) => {
    //     //         assert_eq!(tp, StringLiteralType::RawByte(3));
    //     //         assert_eq!(s, "ABC" );
    //     //     }
    //     //     _ => panic!("Esperava StringLiteral, mas recebeu algo diferente"),
    //     // };

    //     // let mut expr = Rust::new("   \"ABC\" ");
    //     // match expr.next_token() {
    //     //     Token::StringLiteral(Tag::STRING, _, _, tp, s) => {
    //     //         assert_eq!(tp, StringLiteralType::Standard);
    //     //         assert_eq!(s, "ABC");
    //     //     }
    //     //     _ => panic!("Esperava StringLiteral, mas recebeu algo diferente"),
    //     // };

    // }

    #[test]
    fn test_string_tokens_b() {
        let mut expr = Rust::new(" b r br   b\"A\\u{1f980}BC\\n\" r\"A\\u{1f980}BC\\n\" br\"A\\u{1f980}BC\\n\"   r###\"ABC\"### br###\"ABC\"### \"ABC\" ");
        match expr.next_token() {
            Token::Identifier(Tag::IDENTIFIER, _, _, id) => assert_eq!(id, "b"),
            _ => panic!("Esperava Identifier, mas recebeu algo diferente"),
        };
    }

    #[test]
    fn test_string_tokens_r() {
        let mut expr = Rust::new(" r br   b\"A\\u{1f980}BC\\n\" r\"A\\u{1f980}BC\\n\" br\"A\\u{1f980}BC\\n\"   r###\"ABC\"### br###\"ABC\"### \"ABC\" ");
        match expr.next_token() {
            Token::Identifier(Tag::IDENTIFIER, _, _, id) => assert_eq!(id, "r"),
            _ => panic!("Esperava Identifier, mas recebeu algo diferente"),
        };
    }

    #[test]
    fn test_string_tokens_b4() {
        let mut expr = Rust::new("  br   b\"A\\u{1f980}BC\\n\" r\"A\\u{1f980}BC\\n\" br\"A\\u{1f980}BC\\n\"   r###\"ABC\"### br###\"ABC\"### \"ABC\" ");
        match expr.next_token() {
            Token::Identifier(Tag::IDENTIFIER, _, _, id) => assert_eq!(id, "br"),
            _ => panic!("Esperava Identifier, mas recebeu algo diferente"),
        };
    }

    #[test]
    fn test_string_tokens_b1() {
        let mut expr = Rust::new("  b\"A🦀BC\\n\" r\"A\\u{1f980}BC\\n\" br\"A\\u{1f980}BC\\n\"   r###\"ABC\"### br###\"ABC\"### \"ABC\" ");
        match expr.next_token() {
            Token::StringLiteral(Tag::STRING, _, _, tp, s) => {
                assert_eq!(tp, StringLiteralType::ByteString);
                assert_eq!(s, String::from_utf8_lossy(b"A\xF0\x9F\xA6\x80BC\\n").into_owned());
            }
            _ => panic!("Esperava StringLiteral, mas recebeu algo diferente")
        };
    }

    #[test]
    fn test_string_tokens_r1() {
        let mut expr = Rust::new("  r\"A\\u{1f980}BC\\n\" br\"A\\u{1f980}BC\\n\"   r###\"ABC\"### br###\"ABC\"### \"ABC\" ");
        match expr.next_token() {
            Token::StringLiteral(Tag::STRING, _, _, tp, s) => {
                assert_eq!(tp, StringLiteralType::Raw(0));
                assert_eq!(s, r"A\u{1f980}BC\n");
            }
            _ => panic!("Esperava StringLiteral, mas recebeu algo diferente"),
        };
    }

    #[test]
    fn test_string_tokens_br1() {
        let mut expr = Rust::new("  br\"A\\u{1f980}BC\\n\"   r###\"ABC\"### br###\"ABC\"### \"ABC\" ");
        match expr.next_token() {
            Token::StringLiteral(Tag::STRING, _, _, tp, s) => {
                assert_eq!(tp, StringLiteralType::RawByte(0));
                assert_eq!(s, "A🦀BC\n");
            }
            _ => panic!("Esperava StringLiteral, mas recebeu algo diferente"),
        };
    }

    #[test]
    fn test_string_tokens_r___() {
        let mut expr = Rust::new("  r###\"ABC\"### br###\"ABC\"### \"ABC\" ");
        match expr.next_token() {
            Token::StringLiteral(Tag::STRING, _, _, tp, s) => {
                assert_eq!(tp, StringLiteralType::Raw(3));
                assert_eq!(s, "ABC");
            }
            _ => panic!("Esperava StringLiteral, mas recebeu algo diferente"),
        };
    }

    #[test]
    fn test_string_tokens_br___() {
        let mut expr = Rust::new("   br###\"ABC\"### \"ABC\" ");
        match expr.next_token() {
            Token::StringLiteral(Tag::STRING, _, _, tp, s) => {
                assert_eq!(tp, StringLiteralType::RawByte(3));
                assert_eq!(s, "ABC" );
            }
            _ => panic!("Esperava StringLiteral, mas recebeu algo diferente"),
        };
    }

    #[test]
    fn test_string_tokens_abc() {
        let mut expr = Rust::new("   \"ABC\" ");
        match expr.next_token() {
            Token::StringLiteral(Tag::STRING, _, _, tp, s) => {
                assert_eq!(tp, StringLiteralType::Standard);
                assert_eq!(s, "ABC");
            }
            _ => panic!("Esperava StringLiteral, mas recebeu algo diferente"),
        };
    }



    #[test]
    fn test_peek_number_decimal_integers() {
        // Testa inteiros decimais básicos
        let mut expr = Rust::new("0 ");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 0, 0, "0".into(), IntegerLiteralType::ISIZE));

        let mut expr = Rust::new("1 ");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 0, 0, "1".into(), IntegerLiteralType::ISIZE));

        let mut expr = Rust::new("123 ");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 0, 0, "123".into(), IntegerLiteralType::ISIZE));

        let mut expr = Rust::new("999 ");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 0, 0, "999".into(), IntegerLiteralType::ISIZE));
    }

    #[test]
    fn test_peek_number_decimal_with_underscores() {
        // Testa inteiros decimais com separadores de dígitos
        let mut expr = Rust::new("1_000");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 0, 0, "1000".into(), IntegerLiteralType::ISIZE));

        let mut expr = Rust::new("1_000_000");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 0, 0, "1000000".into(), IntegerLiteralType::ISIZE));

        let mut expr = Rust::new("123_456_789");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 0, 0, "123456789".into(), IntegerLiteralType::ISIZE));
    }

    #[test]
    fn test_peek_number_binary() {
        // Testa inteiros binários
        let mut expr = Rust::new("0b101");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 0, 0, "101".into(), IntegerLiteralType::ISIZE));

        let mut expr = Rust::new("0b1010");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 0, 0, "1010".into(), IntegerLiteralType::ISIZE));

        let mut expr = Rust::new("0B11111111");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 0, 0, "11111111".into(), IntegerLiteralType::ISIZE));
    }

    #[test]
    fn test_peek_number_binary_with_underscores() {
        // Testa inteiros binários com separadores
        let mut expr = Rust::new("0b1010_0001");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 0, 0, "10100001".into(), IntegerLiteralType::ISIZE));

        let mut expr = Rust::new("0b1111_0000_1010_0101");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 0, 0, "1111000010100101".into(), IntegerLiteralType::ISIZE));
    }

    #[test]
    fn test_peek_number_octal() {
        // Testa inteiros octais
        let mut expr = Rust::new("0o77");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 0, 0, "77".into(), IntegerLiteralType::ISIZE));

        let mut expr = Rust::new("0o123");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 0, 0, "123".into(), IntegerLiteralType::ISIZE));

        let mut expr = Rust::new("0O755");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 0, 0, "755".into(), IntegerLiteralType::ISIZE));
    }

    #[test]
    fn test_peek_number_octal_with_underscores() {
        // Testa inteiros octais com separadores
        let mut expr = Rust::new("0o123_456");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 0, 0, "123456".into(), IntegerLiteralType::ISIZE));

        let mut expr = Rust::new("0o7777_7777");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 0, 0, "77777777".into(), IntegerLiteralType::ISIZE));
    }

    #[test]
    fn test_peek_number_hexadecimal() {
        // Testa inteiros hexadecimais
        let mut expr = Rust::new("0xFF");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 0, 0, "FF".into(), IntegerLiteralType::ISIZE));

        let mut expr = Rust::new("0x10");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 0, 0, "10".into(), IntegerLiteralType::ISIZE));

        let mut expr = Rust::new("0XABCD");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 0, 0, "ABCD".into(), IntegerLiteralType::ISIZE));
    }

    #[test]
    fn test_peek_number_hexadecimal_with_underscores() {
        // Testa inteiros hexadecimais com separadores
        let mut expr = Rust::new("0xDEAD_BEEF");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 0, 0, "DEADBEEF".into(), IntegerLiteralType::ISIZE));

        let mut expr = Rust::new("0x1234_5678");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 0, 0, "12345678".into(), IntegerLiteralType::ISIZE));
    }

    #[test]
    fn test_peek_number_i8_suffix() {
        let mut expr = Rust::new("0i8");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 0, 0, "0".into(), IntegerLiteralType::I8));

        let mut expr = Rust::new("127i8");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 0, 0, "127".into(), IntegerLiteralType::I8));
    }

    #[test]
    fn test_peek_number_u8_suffix() {
        let mut expr = Rust::new("0u8");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 0, 0, "0".into(), IntegerLiteralType::U8));

        let mut expr = Rust::new("255u8");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 0, 0, "255".into(), IntegerLiteralType::U8));
    }

    #[test]
    fn test_peek_number_i16_suffix() {
        let mut expr = Rust::new("0i16");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 0, 0, "0".into(), IntegerLiteralType::I16));

        let mut expr = Rust::new("32767i16");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 0, 0, "32767".into(), IntegerLiteralType::I16));
    }

    #[test]
    fn test_peek_number_u16_suffix() {
        let mut expr = Rust::new("0u16");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 0, 0, "0".into(), IntegerLiteralType::U16));

        let mut expr = Rust::new("65535u16");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 0, 0, "65535".into(), IntegerLiteralType::U16));
    }

    #[test]
    fn test_peek_number_i32_suffix() {
        let mut expr = Rust::new("0i32");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 0, 0, "0".into(), IntegerLiteralType::I32));

        let mut expr = Rust::new("2147483647i32");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 0, 0, "2147483647".into(), IntegerLiteralType::I32));
    }

    #[test]
    fn test_peek_number_u32_suffix() {
        let mut expr = Rust::new("0u32");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 0, 0, "0".into(), IntegerLiteralType::U32));

        let mut expr = Rust::new("4294967295u32");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 0, 0, "4294967295".into(), IntegerLiteralType::U32));
    }

    #[test]
    fn test_peek_number_i64_suffix() {
        let mut expr = Rust::new("0i64");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 0, 0, "0".into(), IntegerLiteralType::I64));

        let mut expr = Rust::new("9223372036854775807i64");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 0, 0, "9223372036854775807".into(), IntegerLiteralType::I64));
    }

    #[test]
    fn test_peek_number_u64_suffix() {
        let mut expr = Rust::new("0u64");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 0, 0, "0".into(), IntegerLiteralType::U64));

        let mut expr = Rust::new("18446744073709551615u64");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 0, 0, "18446744073709551615".into(), IntegerLiteralType::U64));
    }

    #[test]
    fn test_peek_number_i128_suffix() {
        let mut expr = Rust::new("0i128");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 0, 0, "0".into(), IntegerLiteralType::I128));

        let mut expr = Rust::new("170141183460469231731687303715884105727i128");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 0, 0, "170141183460469231731687303715884105727".into(), IntegerLiteralType::I128));
    }

    #[test]
    fn test_peek_number_u128_suffix() {
        let mut expr = Rust::new("0u128");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 0, 0, "0".into(), IntegerLiteralType::U128));

        let mut expr = Rust::new("340282366920938463463374607431768211455u128");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 0, 0, "340282366920938463463374607431768211455".into(), IntegerLiteralType::U128));
    }

    #[test]
    fn test_peek_number_isize_suffix() {
        let mut expr = Rust::new("0isize");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 0, 0, "0".into(), IntegerLiteralType::ISIZE));

        let mut expr = Rust::new("42isize");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 0, 0, "42".into(), IntegerLiteralType::ISIZE));
    }

    #[test]
    fn test_peek_number_usize_suffix() {
        let mut expr = Rust::new("0usize");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 0, 0, "0".into(), IntegerLiteralType::USIZE));

        let mut expr = Rust::new("42usize");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 0, 0, "42".into(), IntegerLiteralType::USIZE));
    }

    #[test]
    fn test_peek_number_binary_with_i_suffix() {
        let mut expr = Rust::new("0b101i32");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 0, 0, "101".into(), IntegerLiteralType::I32));

        let mut expr = Rust::new("0b1111_0000u8");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 0, 0, "11110000".into(), IntegerLiteralType::U8));
    }

    #[test]
    fn test_peek_number_octal_with_u_suffix() {
        let mut expr = Rust::new("0o755u32");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 0, 0, "755".into(), IntegerLiteralType::U32));

        let mut expr = Rust::new("0o123i64");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 0, 0, "123".into(), IntegerLiteralType::I64));
    }

    #[test]
    fn test_peek_number_hex_with_suffix() {
        let mut expr = Rust::new("0xFFu8");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 0, 0, "FF".into(), IntegerLiteralType::U8));

        let mut expr = Rust::new("0xDEAD_BEEFi32");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 0, 0, "DEADBEEF".into(), IntegerLiteralType::I32));
    }

    #[test]
    fn test_peek_number_float_basic() {
        // Testa floats com ponto decimal
        let mut expr = Rust::new("0.0");
        let token = expr.peek_number();
        assert_eq!(token, Token::Float(Tag::FLOAT, 0, 0, "0.0".into(), FloatLiteralType::F64));

        let mut expr = Rust::new("1.5");
        let token = expr.peek_number();
        assert_eq!(token, Token::Float(Tag::FLOAT, 0, 0, "1.5".into(), FloatLiteralType::F64));

        let mut expr = Rust::new("3.14159");
        let token = expr.peek_number();
        assert_eq!(token, Token::Float(Tag::FLOAT, 0, 0, "3.14159".into(), FloatLiteralType::F64));
    }

    #[test]
    fn test_peek_number_float_with_underscores() {
        let mut expr = Rust::new("1_000.5");
        let token = expr.peek_number();
        assert_eq!(token, Token::Float(Tag::FLOAT, 0, 0, "1000.5".into(), FloatLiteralType::F64));

        let mut expr = Rust::new("1.5_000");
        let token = expr.peek_number();
        assert_eq!(token, Token::Float(Tag::FLOAT, 0, 0, "1.5000".into(), FloatLiteralType::F64));

        let mut expr = Rust::new("123_456.789_012");
        let token = expr.peek_number();
        assert_eq!(token, Token::Float(Tag::FLOAT, 0, 0, "123456.789012".into(), FloatLiteralType::F64));
    }

    #[test]
    fn test_peek_number_float_scientific_notation() {
        // Testa floats com notação científica
        let mut expr = Rust::new("1e10");
        let token = expr.peek_number();
        assert_eq!(token, Token::Float(Tag::FLOAT, 0, 0, "1e10".into(), FloatLiteralType::F64));

        let mut expr = Rust::new("1.5e-10");
        let token = expr.peek_number();
        assert_eq!(token, Token::Float(Tag::FLOAT, 0, 0, "1.5e-10".into(), FloatLiteralType::F64));

        let mut expr = Rust::new("2.5e+5");
        let token = expr.peek_number();
        assert_eq!(token, Token::Float(Tag::FLOAT, 0, 0, "2.5e+5".into(), FloatLiteralType::F64));
    }

    #[test]
    fn test_peek_number_float_scientific_with_underscores() {
        let mut expr = Rust::new("1_000e10");
        let token = expr.peek_number();
        assert_eq!(token, Token::Float(Tag::FLOAT, 0, 0, "1000e10".into(), FloatLiteralType::F64));

        let mut expr = Rust::new("1.5e1_0");
        let token = expr.peek_number();
        assert_eq!(token, Token::Float(Tag::FLOAT, 0, 0, "1.5e10".into(), FloatLiteralType::F64));
    }

    #[test]
    fn test_peek_number_f32_suffix() {
        let mut expr = Rust::new("1.5f32");
        let token = expr.peek_number();
        assert_eq!(token, Token::Float(Tag::FLOAT, 0, 0, "1.5f32".into(), FloatLiteralType::F32));

        let mut expr = Rust::new("0.0f32");
        let token = expr.peek_number();
        assert_eq!(token, Token::Float(Tag::FLOAT, 0, 0, "0.0f32".into(), FloatLiteralType::F32));

        let mut expr = Rust::new("3.14f32");
        let token = expr.peek_number();
        assert_eq!(token, Token::Float(Tag::FLOAT, 0, 0, "3.14f32".into(), FloatLiteralType::F32));
    }

    #[test]
    fn test_peek_number_f64_suffix() {
        let mut expr = Rust::new("1.5f64");
        let token = expr.peek_number();
        assert_eq!(token, Token::Float(Tag::FLOAT, 0, 0, "1.5f64".into(), FloatLiteralType::F64));

        let mut expr = Rust::new("0.0f64");
        let token = expr.peek_number();
        assert_eq!(token, Token::Float(Tag::FLOAT, 0, 0, "0.0f64".into(), FloatLiteralType::F64));

        let mut expr = Rust::new("2.71828f64");
        let token = expr.peek_number();
        assert_eq!(token, Token::Float(Tag::FLOAT, 0, 0, "2.71828f64".into(), FloatLiteralType::F64));
    }

    #[test]
    fn test_peek_number_float_scientific_with_f32() {
        let mut expr = Rust::new("1.5e10f32");
        let token = expr.peek_number();
        assert_eq!(token, Token::Float(Tag::FLOAT, 0, 0, "1.5e10f32".into(), FloatLiteralType::F32));

        let mut expr = Rust::new("2.5e-5f32");
        let token = expr.peek_number();
        assert_eq!(token, Token::Float(Tag::FLOAT, 0, 0, "2.5e-5f32".into(), FloatLiteralType::F32));
    }

    #[test]
    fn test_peek_number_float_scientific_with_f64() {
        let mut expr = Rust::new("1.5e10f64");
        let token = expr.peek_number();
        assert_eq!(token, Token::Float(Tag::FLOAT, 0, 0, "1.5e10f64".into(), FloatLiteralType::F64));

        let mut expr = Rust::new("3.14e+2f64");
        let token = expr.peek_number();
        assert_eq!(token, Token::Float(Tag::FLOAT, 0, 0, "3.14e+2f64".into(), FloatLiteralType::F64));
    }

    #[test]
    fn test_peek_number_float_zero_variants() {
        let mut expr = Rust::new("0.");
        let token = expr.peek_number();
        assert_eq!(token, Token::Float(Tag::FLOAT, 0, 0, "0.f64".into(), FloatLiteralType::F64));

        let mut expr = Rust::new("0.f32");
        let token = expr.peek_number();
        assert_eq!(token, Token::Float(Tag::FLOAT, 0, 0, "0.f32".into(), FloatLiteralType::F32));

        let mut expr = Rust::new("0.f64");
        let token = expr.peek_number();
        assert_eq!(token, Token::Float(Tag::FLOAT, 0, 0, "0.f64".into(), FloatLiteralType::F64));
    }

    #[test]
    fn test_peek_number_complex_examples() {
        // Exemplos mais complexos
        let mut expr = Rust::new("1_000_000i64");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 0, 0, "1000000".into(), IntegerLiteralType::I64));

        let mut expr = Rust::new("0xFF_FF_FFu32");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 0, 0, "FFFFFF".into(), IntegerLiteralType::U32));

        let mut expr = Rust::new("0b1111_0000_1010_0101i16");
        let token = expr.peek_number();
        assert_eq!(token, Token::Integer(Tag::INTEGER, 0, 0, "1111000010100101".into(), IntegerLiteralType::I16));

        let mut expr = Rust::new("1_234.567_890e-5f32");
        let token = expr.peek_number();
        assert_eq!(token, Token::Float(Tag::FLOAT, 0, 0, "1234.567890e-5f32".into(), FloatLiteralType::F32));
    }
}

/*
Seguem as alterações mínimas — todas dentro dos estados já existentes, sem criar/remover estados ou mudar o formato geral da FSM — que corrigem o problema de \u{XXXX} não ser decodificado, junto com os bugs de "caractere extra" que aparecem ao longo do mesmo trecho.

1. Uniformizar a condição do estado 1624 com a do 1622

Hoje o 1622 manda ByteString para o estado de decodificação (1625), mas o 1624 (usado quando o \ não é o primeiro caractere da string, como no nosso caso) manda ByteString para o estado de cópia literal (1623). É essa assimetria que impede o \u{1f980} de ser decodificado.

rust
// Antes (~linha 1215)
Some('\\') if string_type == StringLiteralType::Standard || string_type == StringLiteralType::ByteString => {
    // se for ByteString não pode ter \u
    lexema.push('\\');
    self.advance();
    state = 1623;
    continue;
}

// Depois
Some('\\') if string_type == StringLiteralType::Standard => {
    lexema.push('\\');
    self.advance();
    state = 1623;
    continue;
}

Com isso, ByteString passa a cair sempre no outro braço (Some('\\') => { ...; state = 1625; }), igual já acontece no 1622.

2. Descartar a \ já empilhada quando o escape é \u

Como os estados 1622/1624 empilham o \ otimisticamente antes de saber qual escape é, ao detectarmos u precisamos remover essa barra — \u{...} vira um único caractere, não texto literal.

rust
// Antes (dentro do estado 1625)
Some('u') => {  // \u{...}
    self.advance();
    state = 1630;
    continue;
}

// Depois
Some('u') => {  // \u{...}
    lexema.pop(); // remove o '\' empilhado pelo chamador; \u{...} vira 1 char, não texto
    self.advance();
    state = 1630;
    continue;
}

3. Tornar \n, \r, \t literais para ByteString

O teste espera que, numa ByteString, só o \u{...} seja "traduzido"; os demais escapes devem permanecer como os dois caracteres originais (o \\n do teste vira \+n, não um \n de verdade).

rust
// Antes (dentro do estado 1625)
Some('n') => { lexema.push('\n'); self.advance(); state = 1650; continue; }
Some('r') => { lexema.push('\r'); self.advance(); state = 1650; continue; }
Some('t') => { lexema.push('\t'); self.advance(); state = 1650; continue; }

// Depois
Some('n') => { lexema.push(if string_type == StringLiteralType::ByteString { 'n' } else { '\n' }); self.advance(); state = 1650; continue; }
Some('r') => { lexema.push(if string_type == StringLiteralType::ByteString { 'r' } else { '\r' }); self.advance(); state = 1650; continue; }
Some('t') => { lexema.push(if string_type == StringLiteralType::ByteString { 't' } else { '\t' }); self.advance(); state = 1650; continue; }
````//`\\`, `\'` e `\"` não precisam mudar: o caractere empilhado é o mesmo esteja "decodificado" ou não.

**4. Coletar os dígitos hex em `auxiliary`, não em `lexema`**

`lexema` é o conteúdo final da string; os dígitos de `\u{...}` são só um valor intermediário a ser convertido. A variável `auxiliary` já existe na função (usada para detectar o prefixo `b`/`r`/`br`) e está livre nesse ponto.

```rust
// Estado 1630 — antes
Some('{') => { self.advance(); state = 1631; continue; }
// depois
Some('{') => { auxiliary.clear(); self.advance(); state = 1631; continue; }
```

E nos estados 1631 a 1636, trocar `lexema.push(c)` por `auxiliary.push(c)` no braço do dígito hexadecimal (6 ocorrências, mesma mudança em cada um):

```rust
// Antes
Some(c) if c.is_digit(16) => { lexema.push(c); self.advance(); state = 163X; continue; }
// Depois
Some(c) if c.is_digit(16) => { auxiliary.push(c); self.advance(); state = 163X; continue; }
```

**5. Decodificar de fato o hex no estado 1638**

Este é o bug do "caractere extra": hoje ele empilha `self.current_char()`, que nesse ponto já é o caractere seguinte ao `}` (ele foi consumido por `advance()` ao reconhecer o `}` nos estados 1632‑1637), em vez de gerar o char decodificado.

```rust
// Antes
1638 => {
    lexema.push(self.current_char().unwrap());
    self.advance();
    state = 1622;
    continue;
}

// Depois
1638 => {
    if let Some(ch) = std::char::from_u32(u32::from_str_radix(&auxiliary, 16).unwrap_or(0)) {
        lexema.push(ch);
    }
    auxiliary.clear();
    state = 1624;
    continue;
}
```
(sem `self.advance()`: a posição já está corretamente sobre o primeiro caractere não consumido logo após o `}`).

**6. Parar de voltar para o 1622 depois de já ter consumido um caractere**

O 1622 começa com `self.retract()`, o que só faz sentido para quem chega até ele tendo avançado "a mais" (caso do 1620). Os estados 1623, 1650 e o braço genérico do 1625 já avançaram corretamente depois de empilhar seu caractere, então mandá-los para 1622 faz o `retract()` reprocessar (e, em vários casos, duplicar) o caractere que acabou de ser tratado — foi isso que causava a duplicação de `u`/`n` que apareceria mesmo depois da correção nº1, se essa parte não for arrumada.

```rust
// Estado 1623 — antes
Some(c) => { lexema.push(c); self.advance(); state = 1622; continue; }
// depois
Some(c) => { lexema.push(c); self.advance(); state = 1624; continue; }

// Estado 1625, braço genérico (fallback) — antes
Some(c) => { lexema.push(c); self.advance(); state = 1622; continue; }
// depois
Some(c) => { lexema.push(c); self.advance(); state = 1624; continue; }

// Estado 1650 — antes
1650 => { lexema.push(self.current_char().unwrap()); self.advance(); state = 1622; continue; }
// depois (o caractere já foi tratado por quem chamou 1650; aqui é só retomar o loop normal)
1650 => { state = 1624; continue; }
```

O estado 1660 tem o mesmo padrão, mas é código morto (nada transiciona para ele hoje), então pode ficar como está.

Com essas seis mudanças, o fluxo para `b"A\u{1f980}BC\n"` fica: `A` empilhado normalmente; ao ver `\` antes do `u`, vai para 1625, descarta a barra, decodifica `1f980` via `auxiliary` e empilha o 🦀 real; `B` e `C` seguem normalmente; o `\n` final, por ser `ByteString`, é mantido literal (`\`+`n`). O resultado final é `"A🦀BC\n"` (com `\n` literal), que é exatamente `String::from_utf8_lossy(b"A\xF0\x9F\xA6\x80BC\\n")` — o valor que o teste espera. Nenhuma outra função, assinatura ou estado foi criado/removido; só os alvos de transição, a condição de tipo e o buffer usado para os dígitos hex foram ajustados.










refaça a análise para as cadeias 

r\"A\\u{1f980}BC\\n\" br\"A\\u{1f980}BC\\n\"
Executou 4 comandos, leu 2 arquivos, usou uma ferramenta

Agora vou escrever um pequeno programa de teste para rodar o código original (sem nenhuma correção) contra as duas cadeias e conferir empiricamente, em vez de confiar só na simulação manual.

Executou 12 comandos, criou um arquivo, leu 2 arquivos · 2 notas

Rodei essas duas entradas contra o código original (sem nenhuma das correções da mensagem anterior) para confirmar a análise na prática, e o resultado é mais grave que no caso da ByteString: 
nenhuma das duas retorna um token — as duas fazem next_token() sofrer panic (o parser trava) antes de conseguir devolver qualquer coisa. 
As duas seguem exatamente o mesmo caminho de estados (só o prefixo muda: "r" dá string_type = Raw(0), "br" dá RawByte(0); da aspa de abertura em diante o comportamento é idêntico), então a explicação vale igualmente para as duas.

O motivo é a combinação dos dois defeitos que já tínhamos identificado, mas agora expostos de um jeito diferente porque Raw/RawByte sempre caem no estado 1625 (eles nunca entram no ramo Standard || ByteString do estado 1624, então a assimetria que discutimos antes nem chega a importar aqui — o problema de fundo é outro).

Reconstruindo com posições reais (confirmado rodando o parser):

STATE=1636 pos=(0,11) cur='}'          -> advance(); state=1638      (pos vai para 12, 'B')
STATE=1638 pos=(0,12) cur='B'          -> lexema.push('B'); advance(); state=1622   (empurra 'B' — devia estar decodificando o hex, não copiando o próximo char)
STATE=1622 pos=(0,13) cur='C'          -> retract() volta pra pos 12 ('B' de novo!) -> lexema.push('B') OUTRA VEZ -> lexema="A\1f980BB"
STATE=1624                              -> processa 'C', depois '\'
STATE=1625 cur='n'                      -> lexema.push('\n')  (aqui SEMPRE decodifica para quebra de linha real — o código não faz distinção por tipo neste ramo) ; advance(); state=1650   (pos agora é a aspa de fechamento)
STATE=1650 cur='"'                      -> lexema.push('"')  (empurra a ASPA DE FECHAMENTO como se fosse conteúdo!) ; advance()  -> passa do fim da única linha: current_row vira 1
STATE=1622                              -> retract() -> current_col vira o tamanho da linha (posição "fantasma" de fim-de-linha, current_char() = '\n' virtual) -> empurra esse '\n' virtual também; advance() -> current_row vira 1 de novo
STATE=1624 cur='\0'                     -> current_char() já está retornando o sentinela de "fim do texto" ('\0'), mas nenhum estado trata '\0' como parada — cai no ramo genérico, empurra '\0' e chama advance() mais uma vez

Esse último advance() é o que derruba o processo: current_row já está em 1, mas self.text (resultado de text.lines()) só tem 1 linha, então a checagem em advance()

rust
if self.current_col >= self.text[self.current_row].chars().count() {

indexa self.text[1] num vetor de tamanho 1 e estoura: index out of bounds: the len is 1 but the index is 1 (token.rs, dentro de advance(), ~linha 257), chamado a partir do ramo genérico do estado 1624 em next_token(). Isso acontece igualzinho para r"..." e para br"..." — confirmei executando as duas entradas, ambas panicam com a mesma mensagem, sem chegar a devolver um Token.

Em resumo: é o mesmo bug de fundo de antes (o \u{1f980} nunca é decodificado — os dígitos hex só são empilhados literalmente em lexema pelos estados 1631‑1636 — e os estados-relé 1638/1650 empurram um caractere que não deveriam e mandam o controle de volta para o 1622, cujo retract() reprocessa o que acabou de ser consumido). No caso da ByteString isso só corrompia o conteúdo da string; aqui, como a entrada é curta (uma única linha, sem quebra de linha real), esse excesso de avanços empurra o cursor para além do fim do texto, e como advance() não tem a mesma proteção de limites que current_char() tem, o programa quebra em vez de simplesmente devolver um valor errado.

Apliquei as correções sugeridas na mensagem anterior sobre uma cópia do código e reexecutei as mesmas duas entradas: nenhuma das duas panica mais. 
r"A\u{1f980}BC\n" retorna StringLiteral(Raw(0), "A🦀BC\n") e br"A\u{1f980}BC\n" retorna StringLiteral(RawByte(0), "A🦀BC\n") — o \u{1f980} já sai decodificado como 🦀 nos dois casos. 
Só um detalhe fica em aberto: como o ajuste anterior só tornou \n/\r/\t literais especificamente para ByteString, aqui o \n final ainda vira uma quebra de linha de verdade (byte 0x0A) 
em vez de ficar como \+n literal. Se o comportamento desejado para strings "raw" for não interpretar escape nenhum (mais próximo do Rust de verdade, onde strings raw não processam \n, \u 
nem nada), bastaria estender a mesma condição dos ramos 'n'/'r'/'t' do estado 1625 para incluir também StringLiteralType::Raw(_) e RawByte(_), não só ByteString.
*/