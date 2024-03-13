use regex::Regex;
use std::collections::HashMap;

// Keywords and symbols
lazy_static::lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, &'static str> = [
        ("VALKYRIE", "VALKYRIE"),
        ("ENIGMA", "ENIGMA"),
        ("MJOLNIR", "MJOLNIR"),
        ("OBSIDIAN", "OBSIDIAN"),
        ("QUANTUM", "QUANTUM"),
        ("AETHER", "AETHER"),
        ("CIPHER", "CIPHER"),
        ("PHARAOH", "PHARAOH"),  // Robotics-related keyword
        ("SCARAB", "SCARAB"),    // Robotics-related keyword
        ("ANKH", "ANKH"),        // Robotics-related keyword
        ("NECRO", "NECRO"),      // Robotics-related keyword
    ].iter().cloned().collect();
}

// Token definitions
#[derive(Debug)]
enum Token {
    RUNE(String),
    EGYPTIAN_KEYWORD(String),
    IDENTIFIER(String),
    NUMBER(i32),
    ASSIGN,
    IF,
    ELSE,
    WHILE,
    LPAREN,
    RPAREN,
    COLON,
    COMMA,
    PLUS,
    MINUS,
    TIMES,
    DIVIDE,
    POWER,
    AND,
    OR,
    NOT,
    BITWISE_AND,
    LIBRARY_ACCESS,
    MODULE_IMPORT,
    COMMENT,
    ROBOTICS_KEYWORD,  // Token for robotics-related keywords
}

// Rust lexer implementation
fn lexer(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    // Regular expressions for simple tokens
    let assign_regex = Regex::new(r"=").unwrap();
    let if_regex = Regex::new(r"HORUS").unwrap();
    let else_regex = Regex::new(r"PYRAMID").unwrap();
    let while_regex = Regex::new(r"ᚹᚨᚱᛖᛋ").unwrap();  // Norse rune for "while"
    let lparen_regex = Regex::new(r"\(").unwrap();
    let rparen_regex = Regex::new(r"\)").unwrap();
    let colon_regex = Regex::new(r":").unwrap();
    let comma_regex = Regex::new(r",").unwrap();
    let plus_regex = Regex::new(r"\+").unwrap();
    let minus_regex = Regex::new(r"-").unwrap();
    let times_regex = Regex::new(r"\*").unwrap();
    let divide_regex = Regex::new(r"/").unwrap();
    let power_regex = Regex::new(r"\*\*").unwrap();
    let and_regex = Regex::new(r"ANKH").unwrap();
    let or_regex = Regex::new(r"NECRO").unwrap();
    let not_regex = Regex::new(r"ᚾᚳᚱ").unwrap();  // Norse rune for "not"
    let bitwise_and_regex = Regex::new(r"&").unwrap();
    let library_access_regex = Regex::new(r"\.").unwrap();
    let module_import_regex = Regex::new(r"ᛗᚢᛞᚨᛚ").unwrap();  // Norse rune for "mudal"
    let comment_regex = Regex::new(r"#.*").unwrap();
    let rune_regex = Regex::new(r"[ᚠ-ᛟ]+").unwrap();  // Norse runes
    let egyptian_keyword_regex = Regex::new(r"\b(?:VALKYRIE|ENIGMA|MJOLNIR|OBSIDIAN|QUANTUM|AETHER|CIPHER)\b").unwrap();
    let identifier_regex = Regex::new(r"[a-zA-Z_][a-zA-Z_0-9]*").unwrap();
    let number_regex = Regex::new(r"\d+").unwrap();

    for line in input.lines() {
        if let Some(_) = assign_regex.find(line) {
            tokens.push(Token::ASSIGN);
        } else if let Some(_) = if_regex.find(line) {
            tokens.push(Token::IF);
        } else if let Some(_) = else_regex.find(line) {
            tokens.push(Token::ELSE);
        } else if let Some(_) = while_regex.find(line) {
            tokens.push(Token::WHILE);
        } else if let Some(_) = lparen_regex.find(line) {
            tokens.push(Token::LPAREN);
        } else if let Some(_) = rparen_regex.find(line) {
            tokens.push(Token::RPAREN);
        } else if let Some(_) = colon_regex.find(line) {
            tokens.push(Token::COLON);
        } else if let Some(_) = comma_regex.find(line) {
            tokens.push(Token::COMMA);
        } else if let Some(_) = plus_regex.find(line) {
            tokens.push(Token::PLUS);
        } else if let Some(_) = minus_regex.find(line) {
            tokens.push(Token::MINUS);
        } else if let Some(_) = times_regex.find(line) {
            tokens.push(Token::TIMES);
        } else if let Some(_) = divide_regex.find(line) {
            tokens.push(Token::DIVIDE);
        } else if let Some(_) = power_regex.find(line) {
            tokens.push(Token::POWER);
        } else if let Some(_) = and_regex.find(line) {
            tokens.push(Token::AND);
        } else if let Some(_) = or_regex.find(line) {
            tokens.push(Token::OR);
        } else if let Some(_) = not_regex.find(line) {
            tokens.push(Token::NOT);
        } else if let Some(_) = bitwise_and_regex.find(line) {
            tokens.push(Token::BITWISE_AND);
        } else if let Some(_) = library_access_regex.find(line) {
            tokens.push(Token::LIBRARY_ACCESS);
        } else if let Some(_) = module_import_regex.find(line) {
            tokens.push(Token::MODULE_IMPORT);
        } else if let Some(_) = comment_regex.find(line) {
            tokens.push(Token::COMMENT);
        } else if let Some(captures) = rune_regex.captures(line) {
            tokens.push(Token::RUNE(captures[0].to_string()));
        } else if let Some(captures) = egyptian_keyword_regex.captures(line) {
            tokens.push(Token::EGYPTIAN_KEYWORD(captures[0].to_string()));
        } else if let Some(captures) = identifier_regex.captures(line) {
            tokens.push(Token::IDENTIFIER(captures[0].to_string()));
        } else if let Some(captures) = number_regex.captures(line) {
            if let Ok(num) = captures[0].parse::<i32>() {
                tokens.push(Token::NUMBER(num));
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
