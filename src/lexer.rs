use logos::{Logos};

use codespan::Span;
use codespan_reporting::files::SimpleFile;

use crate::ast::M;

/// The tokens from a successful parse
#[derive(Debug, Clone)]
pub struct TokenData {
    pub file: SimpleFile<String, String>,
    pub tokens: Vec<M<Token>>
}

/// A collection of locations where the tokenizer failed to match a token
#[derive(Debug, Clone)]
pub struct TokenErrors {
    pub file: SimpleFile<String, String>,
    pub errors: Vec<Span>
}

pub fn tokenize(file: SimpleFile<String, String>) -> Result<TokenData, TokenErrors> {
    let tokens: Vec<M<Token>> = Token::lexer(file.source())
        .spanned()
        .map(|(token, span)| 
            M {
                value: token,
                span: Span::new(span.start as u32, span.end as u32)
            }
        )
        .collect();
    
    let errors: Vec<Span> = tokens.iter()
        .filter(|token_m| token_m.value == Token::Error)
        .map(|token_m| token_m.span)
        .collect();

    if errors.is_empty() {
        Ok(TokenData { file, tokens })
    } else {
        Err(TokenErrors { file, errors })
    }
}

/// The Token type for the language.
#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    #[error]
    #[regex(r"[ \n\t\f]+", logos::skip)]
    #[regex(r"//[^\n]*", logos::skip)]
    Error,

    /// Double-quoted string literal
    #[token("\"", parse_string_literal)]
    #[token("r", parse_raw_string_literal)]
    StringLiteral(String),

    /// A Decimal number literal
    #[regex(r"[0-9][_0-9]*", |lex| parse_decint_literal(lex.slice()))]
    DecIntLiteral(u64),

    /// A Decimal floating point literal
    #[regex(r"[0-9][_0-9]*\.[0-9][_0-9]*", |lex| parse_decfloat_literal(lex.slice()))]
    DecFloatLiteral(f64),

    /// A Binary number literal
    #[regex(r"0b[01][_01]*", |lex| parse_bin_literal(lex.slice()))]
    BinLiteral(u64),

    /// A Hexadecimal number literal
    #[regex(r"0x[0-9a-fA-F][_0-9a-fA-F]*", |lex| parse_hex_literal(lex.slice()))]
    HexLiteral(u64),

    /// An Identifier
    #[regex(r"[_a-zA-Z][_a-zA-Z0-9]*", |lex| String::from(lex.slice()))]
    Identifier(String),

    // Keywords -----------------------------------------
    
    /// The Export Keyword
    #[token("export")]
    Export,

    /// The Import Keyword
    #[token("import")]
    Import,
    
    /// The From Keyword
    #[token("from")]
    From,
    
    /// The Function "fn" Keyword
    #[token("fn")]
    Fn,
    
    /// The Table Keyword
    #[token("table")]
    Table,
    
    /// The Memory "mem" Keyword
    #[token("mem")]
    Memory,
    
    /// The If Keyword
    #[token("if")]
    If,
    
    /// The For Keyword
    #[token("for")]
    For,
    
    /// The In Keyword
    #[token("in")]
    In,
    
    /// The Loop Keyword
    #[token("loop")]
    Loop,
    
    /// The Break Keyword
    #[token("break")]
    Break,
    
    /// The Continue Keyword
    #[token("continue")]
    Continue,
    
    /// The Return Keyword
    #[token("return")]
    Return,

    /// The Pointer Type Keyword
    #[token("Ptr")]
    Ptr,
    
    /// The Slice Type Keyword
    #[token("Slice")]
    Slice,

    /// The Unsigned 8-bit Integer Type Keyword 
    #[token("u8")]
    U8,

    /// The Unsigned 16-bit Integer Type Keyword 
    #[token("u16")]
    U16,

    /// The Unsigned 32-bit Integer Type Keyword 
    #[token("u32")]
    U32,

    /// The Unsigned 64-bit Integer Type Keyword 
    #[token("u64")]
    U64,

    /// The Signed 8-bit Integer Type Keyword 
    #[token("s8")]
    S8,

    /// The Signed 16-bit Integer Type Keyword 
    #[token("s16")]
    S16,

    /// The Signed 32-bit Integer Type Keyword 
    #[token("s32")]
    S32,

    /// The Signed 32-bit Integer Type Keyword 
    #[token("s64")]
    S64,

    /// The 32-bit Integer Type Keyword 
    #[token("i32")]
    I32,

    /// The 32-bit Integer Type Keyword 
    #[token("i64")]
    I64,

    /// The 32-bit Floating-point Type Keyword
    #[token("f32")]
    F32,

    /// The 64-bit Floating-point Type Keyword
    #[token("f64")]
    F64,

    /// The As Keyword
    #[token("as")]
    As,

    /// The At Keyword
    #[token("at")]
    At,
    
    /// The Let Keyword
    #[token("let")]
    Let,
    
    /// The Mut Keyword
    #[token("mut")]
    Mut,
    
    /// The Bool Keyword
    #[token("bool")]
    Bool,
    
    /// The True Keyword
    #[token("true")]
    True,
    
    /// The False Keyword
    #[token("false")]
    False,

    // Symbols -----------------------------------------

    /// Left Parenthesis Symbol "("
    #[token("(")]
    LParen,

    /// Right Parenthesis Symbol ")"
    #[token(")")]
    RParen,

    /// Left Brace Symbol "{"
    #[token("{")]
    LBrace,

    /// Right Brace Symbol "}"
    #[token("}")]
    RBrace,

    /// Left Bracket Symbol "["
    #[token("[")]
    LBracket,

    /// Right Bracket Symbol "]"
    #[token("]")]
    RBracket,

    /// The Comma Delimiter ","
    #[token(",")]
    Comma,

    /// The Period or Dot Operator "."
    #[token(".")]
    Dot,

    /// The Range Operator ".."
    #[token("..")]
    Range,
    
    /// Colon Symbol ":"
    #[token(":")]
    Colon,
    
    /// Double Colon Symbol "::"
    #[token("::")]
    DoubleColon,

    /// Semicolon Symbol ";"
    #[token(";")]
    Semicolon,

    /// Assignment Operator "="
    #[token("=")]
    Assign,

    /// The Right Arrow Symbol "->"
    #[token("->")]
    Arrow,

    /// Addition Operator "+"
    #[token("+")]
    Add,

    /// Subtraction Operator "-"
    #[token("-")]
    Sub,

    /// Star Operator "*" (used for dereference and multiply)
    #[token("*")]
    Star,

    /// Division Operator "/"
    #[token("/")]
    Div,

    /// Modulo Operator "%"
    #[token("%")]
    Mod,

    /// Invert Operator "!"
    #[token("!")]
    Invert,

    /// Logical And Operator
    #[token("and")]
    LogicalAnd,

    /// Logical Or Operator
    #[token("or")]
    LogicalOr,

    /// Modulo Operator "|"
    #[token("|")]
    BitOr,

    /// Modulo Operator "&"
    #[token("&")]
    BitAnd,

    /// Modulo Operator "^"
    #[token("^")]
    BitXor,

    /// Bit Shift Left Operator "<<"
    #[token("<<")]
    BitShiftL,

    /// Bit Shift Right Operator ">>"
    #[token(">>")]
    BitShiftR,

    /// Arithmetic Shift Right Operator ">>>"
    #[token(">>>")]
    ArithShiftR,

    /// Bitwise-Or and Assign Operator "+="
    #[token("|=")]
    BitOrAssign,

    /// Bitwise-And and Assign Operator "+="
    #[token("&=")]
    BitAndAssign,

    /// Bitwsie-Xor and Assign Operator "+="
    #[token("^=")]
    BitXorAssign,

    /// Add and Assign Operator "+="
    #[token("+=")]
    AddAssign,

    /// Subtract and Assign Operator "-="
    #[token("-=")]
    SubAssign,

    /// Star Operator "*=" (used for multiply)
    #[token("*=")]
    StarAssign,

    /// Division Operator "/"
    #[token("/=")]
    DivAssign,

    /// Less-than Operator "<"
    #[token("<")]
    LT,

    /// Less-than or Equal Operator "<="
    #[token("<=")]
    LTE,

    /// Greater-than Operator ">"
    #[token(">")]
    GT,

    /// Greater-than or Equal Operator ">="
    #[token(">=")]
    GTE,

    /// Equals Operator "=="
    #[token("==")]
    EQ,

    // Not Equals Operator "!="
    #[token("!=")]
    NEQ
}



/// Parses a string according to the JSON string format in ECMA-404.
fn parse_string_literal<'src>(lex: &mut logos::Lexer<'src, Token>) -> Option<String> {
    let mut c_iter = lex.remainder().chars();
    let mut buf = String::new();

    while let Some(c) = c_iter.next() {
        // End the parse when you encounter another quote
        if c == '"' {
            lex.bump(1);
            return Some(buf);
        }

        // If slash, then parse an escaped character
        if c == '\\' {
            lex.bump(1);
            if let Some((c_esc, c_len)) = parse_escaped_char(&mut c_iter) {
                lex.bump(c_len);
                buf.push(c_esc);
            }
        } else {
            lex.bump(c.len_utf8());
            buf.push(c);
        }
    }

    None
}

/// Parses an escaped character according to the JSON string format in ECMA-404.
/// Takes in an iterator which starts after the beginning slash.
/// If successful, returns the produced char and the length of input consumed.
fn parse_escaped_char<'src>(lex: &mut std::str::Chars<'src>) -> Option<(char, usize)> {
    let res = match lex.next()? {
        '\"' => ('\"', 1),
        '\\' => ('\\', 1),
        '/'  => ('/',  1),
        'b'  => ('\u{0008}', 1),
        'f'  => ('\u{000C}', 1),
        'n'  => ('\n', 1),
        'r'  => ('\r', 1),
        't'  => ('\t', 1),
        'u'  => {
            // Combine next for characters together, fail if they can't be found
            let next_4: [Option<char>; 4] = [lex.next(), lex.next(), lex.next(), lex.next()];
            let next_4: Option<Vec<char>> = next_4.into_iter().map(|c| *c).collect();
            let next_4: String            = next_4?.into_iter().collect();

            let code_point = u32::from_str_radix(&next_4, 16).ok()?;
            let new_c: char = std::char::from_u32(code_point)?;

            (new_c, 5)
        },
        _ => return None,
    };

    Some(res)
}

/// Parses a raw string literal
fn parse_raw_string_literal<'src>(lex: &mut logos::Lexer<'src, Token>) -> Option<String> {
    let mut c_iter = lex.remainder().chars();
    let mut buf = String::new();

    let mut starting_hashes = 0;
    let mut starting_quote = false;

    while let Some(c) = c_iter.next() {
        lex.bump(c.len_utf8());
        if c == '"' {
            starting_quote = true;
            break;
        }
        if c == '#' {
            starting_hashes += 1;
        } else {
            return None;
        }
    }

    if !starting_quote {
        return None;
    }

    let mut seen_quote = false;
    let mut hash_count = 0;

    while let Some(c) = c_iter.next() {
        lex.bump(c.len_utf8());
        if seen_quote && c == '#' {
            hash_count += 1;

            if hash_count == starting_hashes {
                return Some(buf);
            }
            continue;
        }
        
        // Append the unused marker quote
        if seen_quote {
            buf.push('"');
        }
        // Reset the seen quote flag
        seen_quote = false;

        // Append the unused marker hashes
        for _ in 0..hash_count {
            buf.push('#');
        }
        // Reset the hash count
        hash_count = 0;

        if c == '"' {
            seen_quote = true;
        } else {
            buf.push(c);
        }
    }

    None
}

fn parse_decint_literal(s: &str) -> Option<u64> {
    s.replace("_", "").parse().ok()
}

fn parse_decfloat_literal(s: &str) -> Option<f64> {
    s.replace("_", "").parse().ok()
}

fn parse_bin_literal(s: &str) -> Option<u64> {
    u64::from_str_radix(&s[2..].replace("_", ""),  2).ok()   
}

fn parse_hex_literal(s: &str) -> Option<u64> {
    u64::from_str_radix(&s[2..].replace("_", ""),  16).ok()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tokenize_fn_declaration() {
        let input = SimpleFile::new(
            "test".into(),
            "fn test(a: u32) -> u32".into()
        );
        let ident_test = Token::Identifier(String::from("test"));
        let ident_a = Token::Identifier(String::from("a"));
        let output = vec![
            M { value: Token::Fn,      span: Span::new(0, 2) },
            M { value: ident_test,     span: Span::new(3, 7) },
            M { value: Token::LParen,  span: Span::new(7, 8) },
            M { value: ident_a,        span: Span::new(8, 9) },
            M { value: Token::Colon,   span: Span::new(9, 10) },
            M { value: Token::U32,     span: Span::new(11, 14) },
            M { value: Token::RParen,  span: Span::new(14, 15) },
            M { value: Token::Arrow,   span: Span::new(16, 18) },
            M { value: Token::U32,     span: Span::new(19, 22) }
        ];

        match tokenize(input) {
            Ok(TokenData { file: _, tokens }) => assert_eq!(output, tokens),
            Err(_) => panic!("Should not have failed")
        }
    }

    #[test]
    fn tokenize_let() {
        let input = SimpleFile::new(
            "test".into(),
            r#"let a = "asdf\"";"#.into()
        );
        let ident_a = Token::Identifier(String::from("a"));
        let string_asdf = Token::StringLiteral(String::from("asdf\\\";"));
        let output = vec![
            M { value: Token::Let,       span: Span::new(0, 3) },
            M { value: ident_a,          span: Span::new(4, 5) },
            M { value: Token::Assign,    span: Span::new(6, 7) },
            M { value: string_asdf,      span: Span::new(8, 16) },
            M { value: Token::Semicolon, span: Span::new(16, 17) }
        ];

        match tokenize(input) {
            Ok(TokenData { file: _, tokens }) => assert_eq!(output, tokens),
            Err(_) => panic!("Should not have failed")
        }
    }
}