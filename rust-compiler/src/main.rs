use std::{env, hash, path::Path};

#[derive(Debug)]
pub enum Token {
    Letter,
    Digit,
    VarIdent { id: u64 },
    FunIdent { id: u64 },
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
    UintArray,
    BoolArray,
    CharsArray,
    UintLiteral,
    BooleanLiteral,
    CharsLiteral,
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

#[inline(always)]
pub fn hash_u8s(tok_buf: [u8; 16], tok_buf_ptr: usize) -> u64 {
    let mut r: u64 = 0;
    for i in 0..tok_buf_ptr {
        r += (tok_buf[i] as u64);
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

        let mut i: usize = 0;
        while i < contents.len() {
            //print!("{} ", i as char); // debug reading
            if contents[i] != (' ' as u8) {
                tok_buf[tok_buf_ptr] = contents[i];
                tok_buf_ptr += 1;
            } else {
                let cur_hash = hash_u8s(tok_buf, tok_buf_ptr);
                //println!("ptr:{}: {} <-- {} ", tok_buf_ptr, hash, hash_str("fun")); // debug hashing
                if cur_hash == hash_str("fun") {
                    tokens.push(Token::Fun);
                    i += 1;
                } else if cur_hash == hash_str("let") {
                    tokens.push(Token::Let);
                } else if cur_hash == hash_str("if") {
                    tokens.push(Token::If);
                } else if cur_hash == hash_str("else") {
                    tokens.push(Token::Else);
                } else if cur_hash == hash_str("while") {
                    tokens.push(Token::While);
                } else if cur_hash == hash_str("return") {
                    tokens.push(Token::Return);
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
            i += 1;
        }
        println!("{:?}", tokens);
    } else {
        println!("Enter filename as arg!");
    }
}
