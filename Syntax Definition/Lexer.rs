use regex::{Regex, Captures};
use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum TokenType {
    Number,
    UniqueOperator,
    Rune,                  
    StdLibFunction,        
    DataType,              
    ControlStructure,      
    AlgebraicFunction,
    HehnerSymbol,
    Identifier,
    Keyword,
    String,
    Comment,
}

#[derive(Debug, Clone)]
struct Token {
    token_type: TokenType,
    value: String,
    position: usize, // Position in source code
    line: usize,     // Line number
    column: usize,   // Column number
}

lazy_static! {
    static ref TOKEN_PATTERNS: HashMap<TokenType, Regex> = {
        let mut patterns = HashMap::new();
        patterns.insert(TokenType::Number, Regex::new(r"\b\d+(\.\d+)?\b").unwrap());
        patterns.insert(TokenType::UniqueOperator, Regex::new(r"[⊕⊗≺≡↬⧉⊛⊖]").unwrap());
        patterns.insert(TokenType::Rune, Regex::new(r"\b[ᚠᚢᚦᚨᚱᚷᚹᚺᚻᛁᛊᛏᛖᛗᛚᛜᛞᛟᛣᛤᛥ]+").unwrap());
        patterns.insert(TokenType::AlgebraicFunction, Regex::new(r"\b[a-zA-Z_][a-zA-Z0-9_]*ᛋ[a-zA-Z_][a-zA-Z0-9_]*\b").unwrap());
        patterns.insert(TokenType::HehnerSymbol, Regex::new(r"\bHehner_[a-zA-Z]+\b").unwrap());
        patterns.insert(TokenType::Identifier, Regex::new(r"\b[a-zA-Z_][a-zA-Z0-9_]*\b").unwrap());
        patterns.insert(TokenType::Keyword, Regex::new(r"\b(keyword1|keyword2|channelFehu|maintainRuneBalance)\b").unwrap());
        patterns.insert(TokenType::String, Regex::new(r#""[^"]*""#).unwrap());
        patterns.insert(TokenType::Comment, Regex::new(r"𓃀.*").unwrap()); // Using the Egyptian hieroglyph for comments
        patterns.insert(TokenType::StdLibFunction, Regex::new(r"\b[ᚹᛁᚷ]+\b").unwrap());
        patterns.insert(TokenType::DataType, Regex::new(r"\b[ᛒᛚᚩᚲ]+\b").unwrap());
        patterns.insert(TokenType::ControlStructure, Regex::new(r"\b[ᛁᚠ]+\b").unwrap());
        patterns
    };
}

struct Lexer {
    token_patterns: HashMap<TokenType, Regex>,
}

impl Lexer {
    fn new() -> Self {
        Lexer {
            token_patterns: TOKEN_PATTERNS.clone(),
        }
    }

    fn tokenize(&self, code: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let lines = code.lines().enumerate();

        for (line_number, line) in lines {
            for (token_type, pattern) in &self.token_patterns {
                for cap in pattern.captures_iter(line) {
                    let match_pos = cap.get(0).unwrap().start();
                    tokens.push(Token {
                        token_type: token_type.clone(),
                        value: cap[0].to_string(),
                        position: match_pos,
                        line: line_number + 1,
                        column: match_pos - line.as_ptr() as usize + 1,
                    });
                }
            }
        }

        tokens.sort_by(|a, b| a.position.cmp(&b.position).then_with(|| a.line.cmp(&b.line)));
        tokens
    }
}

fn main() {
    let lexer = Lexer::new();
    let code = r#"
        𓃀 Example of GLUE code
        ᚠ2 ⊕ x ⊗ ᚢᛋ4 ⊖ 7ᛋ✵ if ᚠ ≡ ᚢ { channelFehu() } ↬ { maintainRuneBalance() }
        let greeting = "Hello, ᚠᚢᚦ World!"
        ᚨᚱᚷ(ᛏᛖᛗᛈᛚᚱᚢᚾᛖ) -> ᚠᚢᚦᚲᚢᚱᚱᛖᚾᛏ
    "#; 
    let tokens = lexer.tokenize(code);

    for token in tokens {
        println!("{:?}: '{}' at position {}, line {}, column {}", token.token_type, token.value, token.position, token.line, token.column);
    }
}