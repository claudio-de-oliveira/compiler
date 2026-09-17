//! Scanner de literais de cadeia da linguagem Rust.
//!
//! Reconhece as seis formas:
//!     "..."      b"..."      c"..."
//!     r#"..."#   br#"..."#   cr#"..."#
//!
//! Toda a maquina de estados vive em `scan_string_literal`. O laco processa
//! um unico caractere por iteracao; o unico caso de retrocesso (fim da
//! continuacao de linha) e tratado sem avancar o indice naquela iteracao.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringKind {
    /// "..." e r"..."
    Str,
    /// b"..." e br"..."
    ByteStr,
    /// c"..." e cr"..."
    CStr,
}

#[derive(Debug, Clone)]
pub struct StringLiteral {
    pub kind: StringKind,
    /// true para as formas r, br, cr
    pub raw: bool,
    /// numero de cerquilhas da abertura (0 para r"...")
    pub hashes: u32,
    /// conteudo ja com os escapes resolvidos
    pub bytes: Vec<u8>,
    /// quantos caracteres do fluxo o literal consumiu
    pub consumed: usize,
}

#[derive(Debug, Clone)]
pub struct ScanError {
    /// posicao, em caracteres, onde o erro foi detectado
    pub pos: usize,
    pub msg: &'static str,
}

/// Le um literal de cadeia a partir do inicio de `src`.
///
/// Devolve o literal e quantos caracteres foram consumidos, de modo que o
/// chamador possa continuar a analise a partir de `consumed` (inclusive para
/// tratar um eventual sufixo de literal, que esta fora do escopo daqui).
pub fn scan_string_literal(src: &str) -> Result<StringLiteral, ScanError> {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum State {
        Start,     // q0  inicio
        AfterBc,   // q1  leu b ou c
        AfterR,    // q2  leu r
        CountHash, // q3  contando as cerquilhas de abertura
        Body,      // q4  corpo nao-raw
        Raw,       // q5  corpo raw
        Esc,       // q6  logo apos a barra invertida
        Hex1,      // q7  primeiro digito de \xHH
        Hex2,      // q7b segundo digito de \xHH
        UOpen,     // q8  esperando { de \u{...}
        UFirst,    // q8a primeiro digito dentro das chaves
        UDigits,   // q8b demais digitos e o fechamento }
        RawClose,  // q9  contando as cerquilhas de fechamento
        LineCont,  // q10 descartando espacos apos \<nova linha>
        Accept,    // qf  literal completo
    }

    use State::*;

    let chars: Vec<char> = src.chars().collect();

    let mut st = Start;
    let mut kind = StringKind::Str;
    let mut raw = false;
    let mut n: u32 = 0; // cerquilhas da abertura
    let mut k: u32 = 0; // cerquilhas ja casadas no fechamento
    let mut hex: u32 = 0; // acumulador de \xHH e \u{...}
    let mut ndig: u32 = 0; // digitos ja lidos dentro de \u{...}
    let mut buf: Vec<u8> = Vec::new();
    let mut i: usize = 0;

    // Emite um caractere no buffer em UTF-8.
    let enc = |ch: char, out: &mut Vec<u8>| {
        let mut tmp = [0u8; 4];
        out.extend_from_slice(ch.encode_utf8(&mut tmp).as_bytes());
    };

    macro_rules! err {
        ($m:expr) => {
            return Err(ScanError { pos: i, msg: $m })
        };
    }

    loop {
        if st == Accept {
            break;
        }

        let c = match chars.get(i) {
            Some(&c) => c,
            None => err!("literal de cadeia nao terminado"),
        };

        // Fica false apenas quando a transicao devolve o caractere ao fluxo.
        let mut advance = true;

        match st {
            // ---------- prefixo e abertura ----------
            Start => match c {
                '"' => st = Body,
                'b' => {
                    kind = StringKind::ByteStr;
                    st = AfterBc;
                }
                'c' => {
                    kind = StringKind::CStr;
                    st = AfterBc;
                }
                'r' => {
                    raw = true;
                    n = 0;
                    st = AfterR;
                }
                _ => err!("nao e o inicio de um literal de cadeia"),
            },

            AfterBc => match c {
                '"' => st = Body,
                'r' => {
                    raw = true;
                    n = 0;
                    st = AfterR;
                }
                _ => err!("esperado \" ou r apos o prefixo"),
            },

            AfterR => match c {
                '#' => {
                    n += 1;
                    st = CountHash;
                }
                '"' => st = Raw,
                _ => err!("esperado # ou \" apos r"),
            },

            CountHash => match c {
                '#' => n += 1,
                '"' => st = Raw,
                _ => err!("esperado # ou \" na abertura da raw string"),
            },

            // ---------- corpo nao-raw ----------
            Body => match c {
                '"' => st = Accept,
                '\\' => st = Esc,
                ch => {
                    if kind == StringKind::ByteStr && !ch.is_ascii() {
                        err!("b\"...\" so aceita caracteres ASCII");
                    }
                    enc(ch, &mut buf);
                }
            },

            Esc => {
                match c {
                    'n' => {
                        buf.push(b'\n');
                        st = Body;
                    }
                    'r' => {
                        buf.push(b'\r');
                        st = Body;
                    }
                    't' => {
                        buf.push(b'\t');
                        st = Body;
                    }
                    '\\' => {
                        buf.push(b'\\');
                        st = Body;
                    }
                    '\'' => {
                        buf.push(b'\'');
                        st = Body;
                    }
                    '"' => {
                        buf.push(b'"');
                        st = Body;
                    }
                    '0' => {
                        if kind == StringKind::CStr {
                            err!("c\"...\" nao pode conter o byte nulo");
                        }
                        buf.push(0);
                        st = Body;
                    }
                    'x' => {
                        hex = 0;
                        st = Hex1;
                    }
                    'u' => {
                        if kind == StringKind::ByteStr {
                            err!("\\u{...} nao e permitido em b\"...\"");
                        }
                        st = UOpen;
                    }
                    '\n' | '\r' => st = LineCont,
                    _ => err!("sequencia de escape desconhecida"),
                }
            }

            Hex1 => match c.to_digit(16) {
                Some(d) => {
                    hex = d;
                    st = Hex2;
                }
                None => err!("esperado digito hexadecimal apos \\x"),
            },

            Hex2 => match c.to_digit(16) {
                Some(d) => {
                    hex = hex * 16 + d;
                    if kind == StringKind::Str && hex > 0x7F {
                        err!("\\x acima de 7F em string comum; use \\u{...}");
                    }
                    if kind == StringKind::CStr && hex == 0 {
                        err!("c\"...\" nao pode conter o byte nulo");
                    }
                    if kind == StringKind::ByteStr {
                        buf.push(hex as u8);
                    } else {
                        enc(char::from_u32(hex).unwrap(), &mut buf);
                    }
                    st = Body;
                }
                None => err!("esperado segundo digito hexadecimal apos \\x"),
            },

            UOpen => match c {
                '{' => {
                    hex = 0;
                    ndig = 0;
                    st = UFirst;
                }
                _ => err!("esperado { apos \\u"),
            },

            UFirst => match c.to_digit(16) {
                Some(d) => {
                    hex = d;
                    ndig = 1;
                    st = UDigits;
                }
                None => err!("\\u{...} exige ao menos um digito hexadecimal"),
            },

            UDigits => match c {
                '_' => {}
                '}' => {
                    let ch = match char::from_u32(hex) {
                        Some(ch) => ch,
                        None => err!("code point invalido em \\u{...}"),
                    };
                    if kind == StringKind::CStr && hex == 0 {
                        err!("c\"...\" nao pode conter o byte nulo");
                    }
                    enc(ch, &mut buf);
                    st = Body;
                }
                _ => match c.to_digit(16) {
                    Some(d) => {
                        ndig += 1;
                        if ndig > 6 {
                            err!("\\u{...} aceita no maximo 6 digitos");
                        }
                        hex = hex * 16 + d;
                    }
                    None => err!("caractere invalido dentro de \\u{...}"),
                },
            },

            LineCont => match c {
                ' ' | '\t' | '\n' | '\r' => {}
                _ => {
                    // devolve o caractere ao fluxo
                    advance = false;
                    st = Body;
                }
            },

            // ---------- corpo raw ----------
            Raw => match c {
                '"' => {
                    if n == 0 {
                        st = Accept;
                    } else {
                        k = 0;
                        st = RawClose;
                    }
                }
                ch => {
                    if kind == StringKind::ByteStr && !ch.is_ascii() {
                        err!("br\"...\" so aceita caracteres ASCII");
                    }
                    enc(ch, &mut buf);
                }
            },

            // A aspa e as k cerquilhas ja lidas ainda podem ser conteudo:
            // se o fechamento falhar, elas voltam para o buffer.
            RawClose => match c {
                '#' => {
                    k += 1;
                    if k == n {
                        st = Accept;
                    }
                }
                '"' => {
                    buf.push(b'"');
                    for _ in 0..k {
                        buf.push(b'#');
                    }
                    k = 0;
                }
                ch => {
                    if kind == StringKind::ByteStr && !ch.is_ascii() {
                        err!("br\"...\" so aceita caracteres ASCII");
                    }
                    buf.push(b'"');
                    for _ in 0..k {
                        buf.push(b'#');
                    }
                    enc(ch, &mut buf);
                    k = 0;
                    st = Raw;
                }
            },

            Accept => unreachable!(),
        }

        if advance {
            i += 1;
        }
    }

    Ok(StringLiteral {
        kind,
        raw,
        hashes: n,
        bytes: buf,
        consumed: i,
    })
}

fn main() {
    let casos = [
        "\"ola\\n\\u{1F600}\"",
        r#"b"AB\x41""#,
        r#"c"caminho\\tmp""#,
        r#"r"sem \escape aqui""#,
        r##"r#"contem "aspas" e # solto"#"##,
        r##"br#"bytes crus"#"##,
        "\"quebra \\\n        de linha\"",
        r#""faltou fechar"#,
        r#"b"acentuacao nao vai: a""#,
    ];

    for src in casos {
        print!("{:<40} -> ", src);
        match scan_string_literal(src) {
            Ok(lit) => println!(
                "{:?} raw={} #={} consumiu={} conteudo={:?}",
                lit.kind,
                lit.raw,
                lit.hashes,
                lit.consumed,
                String::from_utf8_lossy(&lit.bytes)
            ),
            Err(e) => println!("erro na posicao {}: {}", e.pos, e.msg),
        }
    }
}
