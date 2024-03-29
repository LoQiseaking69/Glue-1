use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum TokenType {
    Number,
    UniqueOperator,        // Custom operators like ⊕, ⊗, etc.
    Rune,                  // Norse runes
    AlgebraicFunction,     // Algebraic functions
    HehnerSymbol,          // Hehner's algebra symbols
    Identifier,            // Variable and function names
    Keyword,               // GLUE-specific keywords
    String,                // String literals
    Comment,               // Comments
    // Extend with other token types as needed
}

#[derive(Debug, Clone)]
struct Token {
    token_type: TokenType,
    value: String,
    position: usize, // Position in source code
}

struct Lexer {
    token_patterns: HashMap<TokenType, Regex>,
}

impl Lexer {
    fn new() -> Self {
        let mut token_patterns = HashMap::new();
        token_patterns.insert(TokenType::Number, Regex::new(r"\b\d+(\.\d+)?\b").unwrap());
        token_patterns.insert(TokenType::UniqueOperator, Regex::new(r"[⊕⊗≺≡↬⧉⊛⊖]").unwrap());
        token_patterns.insert(TokenType::Rune, Regex::new(r"\b[ᚠᚢᚦᚨᚱᚷᚹᚺᚻᛁᛊᛏᛖᛗᛚᛜᛞᛟᛣᛤᛥ]+").unwrap());
        token_patterns.insert(TokenType::AlgebraicFunction, Regex::new(r"\b[a-zA-Z_][a-zA-Z0-9_]*\(\)").unwrap());
        token_patterns.insert(TokenType::HehnerSymbol, Regex::new(r"\bHehner_[a-zA-Z]+\b").unwrap());
        token_patterns.insert(TokenType::Identifier, Regex::new(r"\b[a-zA-Z_][a-zA-Z0-9_]*\b").unwrap());
        token_patterns.insert(TokenType::Keyword, Regex::new(r"\b(keyword1|keyword2|channelFehu|maintainRuneBalance)\b").unwrap());
        token_patterns.insert(TokenType::String, Regex::new(r#""[^"]*""#).unwrap());
        token_patterns.insert(TokenType::Comment, Regex::new(r"//.*").unwrap());

        Lexer { token_patterns }
    }

    fn tokenize(&self, code: &str) -> Vec<Token> {
        let mut tokens = Vec::new();

        for (token_type, pattern) in &self.token_patterns {
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
}

fn main() {
    let lexer = Lexer::new();
    let code = r#"
    // Example of Glue code
    ᚠ2 ⊕ x ⊗ ᚢ(4 ⊖ 7) ✵ if ᚠ ≡ ᚢ { channelFehu() } ↬ { maintainRuneBalance() }
    let greeting = "Hello, ᚠᚢᚦ World!"
    ᚨᚱᚷ(ᛏᛖᛗᛈᛚᚱᚢᚾᛖ) -> ᚠᚢᚦᚲᚢᚱᚱᛖᚾᛏ
    "#; 
    let tokens = lexer.tokenize(code);

    for token in tokens {
        println!("{:?}: '{}' at position {}", token.token_type, token.value, token.position);
    }
}