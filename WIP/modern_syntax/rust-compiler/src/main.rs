use std::{env, path::Path};

use crate::PeekMode::{FunIdent, VarIdent};

#[derive(Debug)]
pub enum Token {
    Letter,
    Digit,
    VarIdent { id: u64 },
    FunIdent { idx: usize, name_hash: u64 },
    Parameter { id: u64 },
    // keywords
    Fun,
    Let,
    If,
    Else,
    While,
    Return,
    Uint,
    Bool,
    Char,
    UintArray,
    CharsArray,
    UintLiteral,
    BooleanLiteral,
    CharLiteral,
    // operators
    Plus,
    Minus,
    Star,
    Slash,
    Mod,
    Equal,
    // controllers
    SemiColon,
    LParen,
    RParen,
    LCurly,
    RCurly,
    LBracket,
    RBracket,
    RightArrow,
    // bitwise
    BitwiseNot,
    BitwiseOr,
    BitwiseAnd,
    BitwiseXor,
    BitShiftLeft,
    BitShiftRight,
    // logicals
    LogicalOr,
    LogicalAnd,
    LogicalEqual,
    LogicalNotEqual,
    LogicalGreator,
    LogicalGreatorOrEqual,
    LogicalLess,
    LogicalLessOrEqual,
}

#[derive(PartialEq)]
pub enum PeekMode {
    None,
    VarIdent,
    FunIdent,
    Conditional,
    ScopeOpen,
}

#[derive(Debug)]
pub struct Fun {
    pub name_hash: u64,
    pub paras_hashes: [u64; 4],
    pub paras_used: u64,
    pub returned: bool,
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
    // cargo run /Users/hadrian/Developer/GitRepos/BareB/tests/hello_world.bb BBareVM
    if env::args().len() > 1 {
        let filepath: String = env::args().nth(1).expect("Failed to specfiy filepath!");
        let arch: String = env::args().nth(2).expect("Failed to specfiy arch!");

        let contents: Vec<u8> = std::fs::read(Path::new(&filepath)).expect("Failed to read file!");
        let mut tok_buf: [u8; 16] = [0_u8; 16];
        let mut tok_buf_ptr: usize = 0;

        let mut tokens: Vec<Token> = Vec::new();
        let mut funs: Vec<Fun> = Vec::new();

        let mut peek_mode: PeekMode = PeekMode::None;
        let mut not_in_main_scope = false;

        let mut i: usize = 0;
        let mut compile_pass: bool = true;
        while i < contents.len() && compile_pass {
            print!(" {}: ", i); // step debug
            print!("{} ", contents[i] as char); // debug reading
            if !contents[i].is_ascii_whitespace() {
                tok_buf[tok_buf_ptr] = contents[i];
                if tok_buf_ptr <= 16 {
                    tok_buf_ptr += 1;
                } else {
                    println!("Compile Error: Words must be smaller than 16 chars!");
                    compile_pass = false;
                }
            } else if peek_mode == PeekMode::None {
                let cur_hash = hash_u8s(tok_buf, tok_buf_ptr);
                //println!("ptr:{}: {} <-- {} ", tok_buf_ptr, hash, hash_str("fun")); // debug hashing
                if cur_hash == hash_str("fun") {
                    tokens.push(Token::Fun);
                    peek_mode = PeekMode::FunIdent;
                } else if cur_hash == hash_str("let") {
                    tokens.push(Token::Let);
                    peek_mode = PeekMode::VarIdent;
                } else if cur_hash == hash_str("if") {
                    tokens.push(Token::If);
                    peek_mode = PeekMode::Conditional;
                } else if cur_hash == hash_str("else") {
                    tokens.push(Token::Else);
                    peek_mode = PeekMode::ScopeOpen;
                } else if cur_hash == hash_str("while") {
                    tokens.push(Token::While);
                    peek_mode = PeekMode::Conditional;
                } else if cur_hash == hash_str("return") {
                    tokens.push(Token::Return);
                    peek_mode = PeekMode::VarIdent;
                } else if cur_hash == hash_str("+") {
                    tokens.push(Token::Plus);
                } else if cur_hash == hash_str("-") {
                    tokens.push(Token::Minus);
                } else if cur_hash == hash_str("*") {
                    tokens.push(Token::Star);
                } else if cur_hash == hash_str("/") {
                    tokens.push(Token::Slash);
                } else if cur_hash == hash_str("%") {
                    tokens.push(Token::Mod);
                } else if cur_hash == hash_str("=") {
                    tokens.push(Token::Equal);
                } else if cur_hash == hash_str(";") {
                    tokens.push(Token::SemiColon);
                } else if cur_hash == hash_str("(") {
                    tokens.push(Token::LParen);
                } else if cur_hash == hash_str(")") {
                    tokens.push(Token::RParen);
                } else if cur_hash == hash_str("{") {
                    tokens.push(Token::LCurly);
                } else if cur_hash == hash_str("}") {
                    tokens.push(Token::RCurly);
                    if not_in_main_scope {
                        not_in_main_scope = false;
                    }
                } else if cur_hash == hash_str("[") {
                    tokens.push(Token::LBracket);
                } else if cur_hash == hash_str("]") {
                    tokens.push(Token::RBracket);
                } else if cur_hash == hash_str("->") {
                    tokens.push(Token::RightArrow);
                } else if cur_hash == hash_str("!") {
                    tokens.push(Token::BitwiseNot);
                } else if cur_hash == hash_str("|") {
                    tokens.push(Token::BitwiseOr);
                } else if cur_hash == hash_str("&") {
                    tokens.push(Token::BitwiseAnd);
                } else if cur_hash == hash_str("^") {
                    tokens.push(Token::BitwiseXor);
                } else if cur_hash == hash_str("<<") {
                    tokens.push(Token::BitShiftLeft);
                } else if cur_hash == hash_str(">>") {
                    tokens.push(Token::BitShiftRight);
                } else if cur_hash == hash_str("||") {
                    tokens.push(Token::LogicalOr);
                } else if cur_hash == hash_str("&&") {
                    tokens.push(Token::LogicalAnd);
                } else if cur_hash == hash_str("==") {
                    tokens.push(Token::LogicalEqual);
                } else if cur_hash == hash_str("!=") {
                    tokens.push(Token::LogicalNotEqual);
                } else if cur_hash == hash_str(">") {
                    tokens.push(Token::LogicalGreator);
                } else if cur_hash == hash_str(">=") {
                    tokens.push(Token::LogicalGreatorOrEqual);
                } else if cur_hash == hash_str("<") {
                    tokens.push(Token::LogicalLess);
                } else if cur_hash == hash_str("<=") {
                    tokens.push(Token::LogicalLessOrEqual);
                } else {
                    println!("unknown token -> hash: {}, word: {:?}", cur_hash, tok_buf);
                }

                tok_buf = [0_u8; 16];
                tok_buf_ptr = 0;
            }
            if peek_mode != PeekMode::None {
                unsafe {
                    match peek_mode {
                        PeekMode::None => std::hint::unreachable_unchecked(),
                        PeekMode::FunIdent => {
                            i += 1;
                            let mut fun_name = 0;
                            let mut not_fin: bool = true;
                            while not_fin && tok_buf_ptr < 16 {
                                if contents[i].is_ascii_alphanumeric() {
                                    tok_buf[tok_buf_ptr] = contents[i];
                                } else if !contents[i].is_ascii_alphanumeric()
                                    && (contents[i] != '(' as u8)
                                {
                                    println!(
                                        "Compile Error: Function name is not alphanumeric! at char {}",
                                        contents[i] as char
                                    );
                                    compile_pass = false;
                                } else if contents[i] == '(' as u8 {
                                    fun_name = hash_u8s(tok_buf, tok_buf_ptr);
                                    not_fin = false;
                                } else {
                                    tok_buf[tok_buf_ptr] = contents[i];
                                }
                                tok_buf_ptr += 1;
                                i += 1;
                            }
                            tok_buf = [0_u8; 16];
                            tok_buf_ptr = 0;

                            let mut paras_hashes = [0_u64; 4];
                            let mut para_cnt: usize = 0;
                            while tok_buf_ptr < 16 {
                                if contents[i] == ')' as u8 {
                                    break;
                                } else if contents[i].is_ascii_alphanumeric()
                                    && (contents[i] != ',' as u8)
                                {
                                    tok_buf[tok_buf_ptr] = contents[i];
                                } else if contents[i] == ',' as u8 {
                                    if para_cnt > 4 {
                                        compile_pass = false;
                                        println!(
                                            "Compile Error: Functions can only have at max 4 parameters!"
                                        );
                                    } else {
                                        paras_hashes[para_cnt] = hash_u8s(tok_buf, tok_buf_ptr);
                                        para_cnt += 1;
                                        tok_buf = [0; 16];
                                        tok_buf_ptr = 0;
                                    }
                                }

                                tok_buf_ptr += 1;
                                i += 1;
                            }
                            tokens.push(Token::FunIdent {
                                idx: funs.len(),
                                name_hash: fun_name,
                            });
                            funs.push(Fun {
                                name_hash: fun_name,
                                paras_hashes: paras_hashes,
                                paras_used: para_cnt as u64,
                                returned: false, //TODO
                            });
                            peek_mode = PeekMode::ScopeOpen;
                            tok_buf = [0; 16];
                            tok_buf_ptr = 0;
                        }
                        PeekMode::VarIdent => {}
                        PeekMode::Conditional => {}
                        PeekMode::ScopeOpen => {
                            println!("{:?}", funs);
                            if contents[i].is_ascii_whitespace() {
                                i += 1;
                            } else if contents[i] == '{' as u8 {
                                i += 1;
                                not_in_main_scope = true;
                                peek_mode = PeekMode::None;
                            } else {
                                println!("Compile Error: A Scope was required to open!");
                                compile_pass = false;
                            }
                        }
                    }
                }
            }
            i += 1;
            println!("i is {}", i);
        }
        println!("{:?}", tokens);
    } else {
        println!("Enter filename as arg!");
    }
}
