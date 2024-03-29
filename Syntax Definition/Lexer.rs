use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum TokenType {
    Number,
    Operator,
    Rune, // Added for Norse runes
    AlgebraicFunction,
    HehnerSymbol, // Added for Hehner's algebra symbols
    Identifier, // For variable names, function names
    Keyword, // For GLUE-specific keywords
    // Extend with other necessary token types
}

#[derive(Debug, Clone)]
struct Token {
    token_type: TokenType,
    value: String,
    position: usize, // Token position in the source code
}

fn tokenize(code: &str) -> Vec<Token> {
    let token_patterns = HashMap::from([
        (TokenType::Number, Regex::new(r"\b\d+(\.\d+)?\b").unwrap()),
        (TokenType::Operator, Regex::new(r"[+\-*/]").unwrap()),
        (TokenType::Rune, Regex::new(r"regex_for_norse_runes").unwrap()), // Pattern for Norse runes
        (TokenType::AlgebraicFunction, Regex::new(r"regex_for_algebraic_functions").unwrap()), // Pattern for algebraic functions
        (TokenType::HehnerSymbol, Regex::new(r"regex_for_hehner_symbols").unwrap()), // Pattern for Hehner's symbols
        (TokenType::Identifier, Regex::new(r"\b[a-zA-Z_][a-zA-Z0-9_]*\b").unwrap()),
        (TokenType::Keyword, Regex::new(r"\b(keyword1|keyword2)\b").unwrap()), // Replace with actual GLUE keywords
        // Add more patterns as needed
    ]);

    let mut tokens = Vec::new();

    for (token_type, pattern) in &token_patterns {
        for cap in pattern.captures_iter(code) {
            tokens.push(Token {
                token_type: token_type.clone(),
                value: cap[0].to_string(),
                position: cap.get(0).unwrap().start(),
            });
        }
    }

    // Sort by position in the source code for accurate parsing order
    tokens.sort_by_key(|token| token.position);
    tokens
}

fn main() {
    // Example GLUE code with varied syntax elements
    let code = "ᚠ2 + x * ᚢ(4 / 7) if ᚠᚢᚦ else ᚠ"; // Includes Norse runes, algebraic functions, and keywords
    let tokens = tokenize(code);

    // Print tokens for verification
    for token in tokens {
        println!("{:?}: '{}' at position {}", token.token_type, token.value, token.position);
    }
}