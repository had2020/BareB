use std::{env, path::Path, thread::scope};

#[derive(Debug)]
enum Tok {
    Ident { id_hash: u64 },
    FunSig { sig_hash: u64 },
    Parameter { id_hash: u64 },
    // keywords
    Const,
    Let,
    If,
    Else,
    While,
    Return,
    Returns { value: bool },
    Void,
    Uint,
    UintArray,
    AtAddr,
    // literals
    UintLiteral { uint: u64 },
    // operators
    Plus,
    Minus,
    Star,
    Slash,
    Mod,
    Assign,
    // scoped
    LCurly,
    RCurly,
    // bitwise
    BitwiseNot,
    BitwiseOr,
    BitwiseAnd,
    BitwiseXor,
    BitShiftLeft,
    BitShiftRight,
    // logicals
    LogicalEqual,
    LogicalNotEqual,
    LogicalGreator,
    LogicalGreatorOrEqual,
    LogicalLess,
    LogicalLessOrEqual,
}

#[derive(PartialEq, Debug)]
enum TokType {
    Some,
    Scope,
    Ident,
    Keyword,
    Value,
    Assignment,
    Operation,
    Controller,
    Logical,
    ReturnType,
    CommentClose,
    FnSig,
    ParamIdent,
    FnIdent,
}

#[inline(always)]
pub fn hash_u8s(tok_buf: [u8; 16], tok_buf_ptr: usize) -> u64 {
    let mut r: u64 = 0;
    for i in 0..tok_buf_ptr {
        r += tok_buf[i] as u64;
    }
    return r;
}

#[inline(always)]
pub fn hash_str(s: &str) -> u64 {
    let mut tok_buf = [0_u8; 16];
    let mut j: usize = 0;
    for i in s.chars() {
        tok_buf[j] = i as u8;
        j += 1;
    }
    return hash_u8s(tok_buf, s.len());
}

fn main() {
    // cargo run /Users/hadrian/Developer/GitRepos/BareB/tests/hello.b BBareVM
    if env::args().len() > 1 {
        let filepath: String = env::args().nth(1).expect("Failed to specfiy filepath!");
        let arch: String = env::args().nth(2).expect("Failed to specfiy arch!");

        let text: Vec<u8> = std::fs::read(Path::new(&filepath)).expect("Failed to read file!");
        let mut tok_buf: [u8; 16] = [0_u8; 16];
        let mut tok_buf_ptr: usize = 0;

        let mut toks: Vec<Tok> = Vec::new();

        let mut scope_level = 0;
        let mut expected: TokType = TokType::Some;

        let mut compile_pass: bool = true;
        let mut i: usize = 0;

        // TODO operations...

        while i < text.len() && compile_pass {
            if !text[i].is_ascii_whitespace() {
                if text[i] != ';' as u8 {
                    tok_buf[tok_buf_ptr] = text[i];
                    if tok_buf_ptr < 16 {
                        tok_buf_ptr += 1;
                    } else {
                        println!("Compile Error: Words must be smaller than 16 chars!");
                        compile_pass = false;
                    }
                    print!("{} ", text[i] as char); // debug reading
                }
            } else if expected == TokType::Some {
                let cur_hash = hash_u8s(tok_buf, tok_buf_ptr);
                // I know this is ugly deal with it!
                if cur_hash == hash_str("const") {
                    toks.push(Tok::Const);
                    expected = TokType::Ident;
                } else if cur_hash == hash_str("returns") {
                    expected = TokType::ReturnType;
                } else if cur_hash == hash_str("/*") {
                    expected = TokType::CommentClose;
                } else if cur_hash == hash_str("let") {
                    toks.push(Tok::Let);
                    expected = TokType::Ident;
                } else if cur_hash == hash_str("+") {
                    toks.push(Tok::Plus);
                    expected = TokType::Ident;
                } else if cur_hash == hash_str("-") {
                    toks.push(Tok::Minus);
                    expected = TokType::Ident;
                } else if cur_hash == hash_str("*") {
                    toks.push(Tok::Star);
                    expected = TokType::Ident;
                } else if cur_hash == hash_str("/") {
                    toks.push(Tok::Slash);
                    expected = TokType::Ident;
                } else if cur_hash == hash_str("%") {
                    toks.push(Tok::Mod);
                    expected = TokType::Ident;
                } else if cur_hash == hash_str("!") {
                    toks.push(Tok::BitwiseNot);
                    expected = TokType::Ident;
                } else if cur_hash == hash_str("|") {
                    toks.push(Tok::BitwiseOr);
                    expected = TokType::Ident;
                } else if cur_hash == hash_str("&") {
                    toks.push(Tok::BitwiseAnd);
                    expected = TokType::Ident;
                } else if cur_hash == hash_str("^") {
                    toks.push(Tok::BitwiseXor);
                    expected = TokType::Ident;
                } else if cur_hash == hash_str("<<") {
                    toks.push(Tok::BitShiftLeft);
                    expected = TokType::Ident;
                } else if cur_hash == hash_str(">>") {
                    toks.push(Tok::BitShiftRight);
                    expected = TokType::Ident;
                } else if cur_hash == hash_str("@") {
                    toks.push(Tok::AtAddr);
                    expected = TokType::Ident;
                }

                /*
                else {
                    // Debuging ONLY!
                    let mut debug_line = String::new();
                    for j in tok_buf {
                        debug_line = format!("{}{}", debug_line, j as char);
                    }
                    print!("\nLine: {} ", debug_line);
                    println!(
                        "Uknown symbol: buf: {:?}, hash: {}, tokens: {:?}",
                        tok_buf,
                        hash_u8s(tok_buf, tok_buf_ptr),
                        toks,
                    )
                }*/

                tok_buf = [0; 16];
                tok_buf_ptr = 0;
                // I am also definitly not using comments to space things.
            } else if expected == TokType::Ident {
                toks.push(Tok::Ident {
                    id_hash: hash_u8s(tok_buf, tok_buf_ptr),
                });

                expected = TokType::Assignment;
                tok_buf = [0; 16];
                tok_buf_ptr = 0;
                // Would'nt you expect a value after a symbol?
            } else if expected == TokType::Assignment {
                if hash_u8s(tok_buf, tok_buf_ptr) == hash_str("=") {
                    toks.push(Tok::Assign);
                    expected = TokType::Value;
                }
                tok_buf = [0; 16];
                tok_buf_ptr = 0;
            } else if expected == TokType::Value {
                let mut i_cpy = i.clone() - (tok_buf_ptr + 1);
                /*
                println!("checking {}", text[i_cpy] as char);
                println!("buf: {:?}", tok_buf);
                println!("buf_ptr: {}", tok_buf_ptr);*/
                if text[i_cpy].is_ascii_alphabetic() {
                    toks.push(Tok::Ident {
                        id_hash: hash_u8s(tok_buf, tok_buf_ptr),
                    });
                    expected = TokType::Some;
                } else if text[i_cpy].is_ascii_digit() {
                    let mut pending: Vec<u8> = Vec::new();
                    let mut literal: u64 = 0;
                    let mut j: usize = 1;
                    while j <= 16 {
                        //println!("from value check: {}", text[i_cpy] as char);
                        if (text[i_cpy] as char).is_ascii_digit() {
                            pending.push(
                                (text[i_cpy] as char)
                                    .to_digit(10)
                                    .expect("lexical literal parse fail!")
                                    as u8,
                            );
                        } else {
                            j = 17;
                        }
                        i_cpy += 1;
                        j += 1;
                    }
                    //i += pending.len() - 1; // this moved my i pointer to far
                    for &digit in &pending {
                        literal = literal * 10 + digit as u64;
                    }
                    toks.push(Tok::UintLiteral { uint: literal });
                    expected = TokType::Some;
                    //println!("{:?}", pending);
                } else if text[i_cpy] == '[' as u8 {
                    todo!("arrays and strings");
                } else if !(text[i_cpy].is_ascii_whitespace()) {
                    compile_pass = false;
                    println!(
                        "Compile failed expected a Literal or Identifer! '{}'",
                        text[i] as char
                    )
                }

                tok_buf = [0; 16];
                tok_buf_ptr = 0;
            } else if expected == TokType::CommentClose {
                if hash_u8s(tok_buf, tok_buf_ptr) == hash_str("*/") {
                    expected = TokType::Some;
                    tok_buf = [0; 16];
                    tok_buf_ptr = 0;
                } else {
                    tok_buf = [0; 16];
                    tok_buf_ptr = 0;
                }
            } else if expected == TokType::ReturnType {
                let cur_hash = hash_u8s(tok_buf, tok_buf_ptr);
                if cur_hash == hash_str("uint") {
                    toks.push(Tok::Returns { value: true });
                    expected = TokType::FnSig;
                } else if cur_hash == hash_str("void") {
                    toks.push(Tok::Returns { value: false });
                    expected = TokType::FnSig;
                } else {
                    println!("Compile failed that's not a return type!");
                    compile_pass = false;
                }
            } else if expected == TokType::FnSig {
                let cur_hash = hash_u8s(tok_buf, tok_buf_ptr);
                if cur_hash == hash_str("param") {
                    expected = TokType::ParamIdent;
                } else if cur_hash == hash_str("fn") {
                    expected = TokType::FnIdent;
                }
                tok_buf = [0; 16];
                tok_buf_ptr = 0;
            } else if expected == TokType::ParamIdent {
                toks.push(Tok::Parameter {
                    id_hash: hash_u8s(tok_buf, tok_buf_ptr),
                });
                expected = TokType::FnSig;
                tok_buf = [0; 16];
                tok_buf_ptr = 0;
            } else if expected == TokType::FnIdent {
                toks.push(Tok::Parameter {
                    id_hash: hash_u8s(tok_buf, tok_buf_ptr),
                });
                expected = TokType::Some;
                tok_buf = [0; 16];
                tok_buf_ptr = 0;
            }
            //println!("{:?}", expected);

            i += 1;
        }
        println!("Tokens: {:?}", toks);
    } else {
        println!("Enter filename as arg!");
    }
}
