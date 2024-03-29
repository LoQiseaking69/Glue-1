use regex::Regex;
use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, &'static str> = [
        ("VALKYRIE", "VALKYRIE"),   // Robotics-related keyword
        ("ENIGMA", "ENIGMA"),       // Robotics-related keyword
        ("MJOLNIR", "MJOLNIR"),     // Robotics-related keyword
        ("OBSIDIAN", "OBSIDIAN"),   // Robotics-related keyword
        ("QUANTUM", "QUANTUM"),     // Robotics-related keyword
        ("AETHER", "AETHER"),       // Robotics-related keyword
        ("CIPHER", "CIPHER"),       // Robotics-related keyword
        ("PHARAOH", "PHARAOH"),     // Robotics-related keyword
        ("SCARAB", "SCARAB"),       // Robotics-related keyword
        ("ANKH", "ANKH"),           // Robotics-related keyword
        ("NECRO", "NECRO"),         // Robotics-related keyword
    ]
    .iter()
    .cloned()
    .collect();
}

#[derive(Debug)]
enum Token {
    Rune(String),
    RoboticsKeyword(String),
    Identifier(String),
    Number(i32),
    Assign,
    If,
    Else,
    While,
    LParen,
    RParen,
    Colon,
    Comma,
    Plus,
    Minus,
    Times,
    Divide,
    Power,
    And,
    Or,
    Not,
    BitwiseAnd,
    LibraryAccess,
    ModuleImport,
    Comment,
}

fn lexer(input: &str) -> Vec<Token> {
    // Define regular expressions
    let regex_map = [
        (r"=", Token::Assign),
        (r"HORUS", Token::If),
        (r"PYRAMID", Token::Else),
        (r"ᚹᚨᚱᛖᛋ", Token::While),  // Norse rune for "while"
        (r"\(", Token::LParen),
        (r"\)", Token::RParen),
        (r":", Token::Colon),
        (r",", Token::Comma),
        (r"\+", Token::Plus),
        (r"-", Token::Minus),
        (r"\*", Token::Times),
        (r"/", Token::Divide),
        (r"\*\*", Token::Power),
        (r"ANKH", Token::And),
        (r"NECRO", Token::Or),
        (r"ᚾᚳᚱ", Token::Not),  // Norse rune for "not"
        (r"&", Token::BitwiseAnd),
        (r"\.", Token::LibraryAccess),
        (r"ᛗᚢᛞᚨᛚ", Token::ModuleImport),  // Norse rune for "module"
        (r"#.*", Token::Comment),
        (r"\[.*?\]", Token::Rune),
        (r"\b(?:VALKYRIE|ENIGMA|MJOLNIR|OBSIDIAN|QUANTUM|AETHER|CIPHER)\b", Token::RoboticsKeyword),
        (r"[a-zA-Z_][a-zA-Z_0-9]*", Token::Identifier),
        (r"\d+", Token::Number),
    ];

    let mut tokens = Vec::new();

    for line in input.lines() {
        for (pattern, token) in regex_map.iter() {
            let regex = Regex::new(pattern).unwrap();
            if let Some(captures) = regex.captures(line) {
                match token {
                    Token::Number(_) => {
                        if let Ok(num) = captures[0].parse::<i32>() {
                            tokens.push(Token::Number(num));
                        }
                    },
                    Token::Rune(_) | Token::RoboticsKeyword(_) | Token::Identifier(_) => {
                        tokens.push(token.clone(captures[0].to_string()));
                    },
                    _ => tokens.push(token.clone()),
                }
            }
        }
    }

    tokens
}

fn main() {
    let code = r#"
VALKYRIE x = 5
HORUS if x > 0:
    ENIGMA y = x + 1
PYRAMID else:
    ENIGMA y = x - 1

# This is a comment

LIBRARY.ACCESS.some_function()

ᛗᚢᛞᚨᛚ math_operations

PHARAOH robotics_function()
"#;

    let tokens = lexer(code);
    println!("{:?}", tokens);
}
