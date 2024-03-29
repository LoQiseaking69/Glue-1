use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum TokenType {
    Number,
    UniqueOperator,  // For custom operators like ⊕, ⊗, etc.
    Rune,            // For Norse runes
    AlgebraicFunction,
    HehnerSymbol,    // For Hehner's algebra symbols
    Identifier,      // Variable and function names
    Keyword,         // GLUE-specific keywords
    String,          // For string literals
    Comment,         // For comments
    // Extend with other token types as needed
}

#[derive(Debug, Clone)]
struct Token {
    token_type: TokenType,
    value: String,
    position: usize, // Position in source code
}

fn tokenize(code: &str) -> Vec<Token> {
    let token_patterns = HashMap::from([
        (TokenType::Number, Regex::new(r"\b\d+(\.\d+)?\b").unwrap()),
        (TokenType::UniqueOperator, Regex::new(r"[⊕⊗≺≡↬⧉⊛⊖]").unwrap()),
        (TokenType::Rune, Regex::new(r"[ᚠᚢᚦᚨ]+").unwrap()),
        (TokenType::AlgebraicFunction, Regex::new(r"\b\w+\(\)").unwrap()),
        (TokenType::HehnerSymbol, Regex::new(r"\bHehner_[a-zA-Z]+\b").unwrap()),
        (TokenType::Identifier, Regex::new(r"\b[a-zA-Z_][a-zA-Z0-9_]*\b").unwrap()),
        (TokenType::Keyword, Regex::new(r"\b(keyword1|keyword2)\b").unwrap()),
        (TokenType::String, Regex::new(r"\".*?\"").unwrap()),
        (TokenType::Comment, Regex::new(r"//.*?$").unwrap()),
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

    tokens.sort_by_key(|token| token.position);
    tokens
}

fn main() {
    let code = r#"
    // Example of Glue code
    ᚠ2 ⊕ x ⊗ ᚢ(4 ⊖ 7) ✵ if ᚠ ≡ ᚢ { channelFehu() } ↬ { maintainRuneBalance() }
    let greeting = "Hello, ᚠᚢᚦ World!"
    "#; 
    let tokens = tokenize(code);

    for token in tokens {
        println!("{:?}: '{}' at position {}", token.token_type, token.value, token.position);
    }
}