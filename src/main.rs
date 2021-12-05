#![allow(dead_code, unused_imports)]

use std::env;
use std::fs;
use std::io;
mod chunk;

// #[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier(String),
    Str(String),
    Number(i32),

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

fn scan(text: String) -> Vec<TokenType> {
    let mut tokens = Vec::new();
    let mut iter = text.chars().peekable();
    while let Some(c) = iter.next() {
        let t = match c {
            '(' => Some(TokenType::LeftParen),
            ')' => Some(TokenType::RightParen),
            '{' => Some(TokenType::LeftBrace),
            '}' => Some(TokenType::RightBrace),
            ',' => Some(TokenType::Comma),
            '.' => Some(TokenType::Dot),
            '-' => Some(TokenType::Minus),
            '+' => Some(TokenType::Plus),
            ';' => Some(TokenType::Semicolon),
            '*' => Some(TokenType::Star),
            '/' => match iter.peek() {
                Some('/') => {
                    while let Some(slash_c) = iter.next() {
                        if slash_c == '\n' {
                            break;
                        };
                    }
                    None
                }
                _ => Some(TokenType::Slash),
            },

            '!' => match iter.peek() {
                Some('=') => {
                    iter.next();
                    Some(TokenType::BangEqual)
                }
                _ => Some(TokenType::Bang),
            },
            '=' => match iter.peek() {
                Some('=') => {
                    iter.next();
                    Some(TokenType::EqualEqual)
                }
                _ => Some(TokenType::Equal),
            },
            '>' => match iter.peek() {
                Some('=') => {
                    iter.next();
                    Some(TokenType::GreaterEqual)
                }
                _ => Some(TokenType::Bang),
            },
            '<' => match iter.peek() {
                Some('=') => {
                    iter.next();
                    Some(TokenType::LessEqual)
                }
                _ => Some(TokenType::Bang),
            },
            '\n' => None,
            '\r' => None,
            '\t' => None,
            ' ' => None,

            '"' => unimplemented!(),

            _ => unimplemented!(),
        };
        if let Some(tok) = t {
            tokens.push(tok)
        }
    }
    tokens
}

fn run(line: String) {
    println!("{}", line)
}

fn run_prompt() {
    let mut l = String::new();

    io::stdin().read_line(&mut l).expect("Failed to read line");
    let tokens = scan(l);
    println!("{:?}", tokens)
}

fn run_file(path: String) {
    let content = fs::read_to_string(path).expect("Error while reading file");
    run(content)
}

fn main() {
    // let args = env::args();
    // let (n, _) = args.size_hint();
    // match n {
    //     1 => run_prompt(),
    //     2 => run_file(args.last().unwrap()),
    //     _ => println!("Usage: ./exe [Filename]"),
    // }

    let mut c = chunk::Chunk::create_chunk();
    let constant_index: usize = c.add_constant(1.5);
    c.write_chunk(chunk::OpCode::OpConstant(constant_index));
    c.write_chunk(chunk::OpCode::OpReturn);
    c.disassemble_chunk("chunk1".to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_single_dot() {
        assert_eq!(vec!(TokenType::Dot), scan(".".to_string()));
    }

    #[test]
    fn test_scan_comment_no_newline() {
        assert_eq!(
            vec!(TokenType::Dot, TokenType::Equal),
            scan(".= // =hi".to_string())
        );
    }

    #[test]
    fn test_scan_comment_newline_text() {
        assert_eq!(
            vec!(
                TokenType::Dot,
                TokenType::Dot,
                TokenType::Comma,
                TokenType::Comma
            ),
            scan(".. // yes\n,, // new".to_string())
        );
    }
}
