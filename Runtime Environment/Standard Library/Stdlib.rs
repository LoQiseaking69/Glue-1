 // Declare each component as a module
mod lexer;
mod parser;
mod compiler;
mod stdlib;
mod runtime;

pub use lexer::{Lexer, Token, TokenType};
pub use parser::{Parser, ASTNode};
pub use compiler::Compiler;
pub use stdlib::StdLib;
pub use runtime::Runtime;

pub struct GlueLanguageConfig {
    // Configuration options (if any)
    // ...
}

pub enum GlueError {
    LexerError(String),
    ParserError(String),
    CompilerError(String),
    RuntimeError(String),
    // Other error types as needed
}

pub struct GlueLanguage {
    lexer: Option<Lexer>,
    parser: Option<Parser>,
    compiler: Option<Compiler>,
    stdlib: StdLib,
    runtime: Runtime,
    config: GlueLanguageConfig,
}

impl GlueLanguage {
    pub fn new(config: GlueLanguageConfig) -> Self {
        GlueLanguage {
            lexer: None,
            parser: None,
            compiler: None,
            stdlib: StdLib::new(config.clone()), // Assuming StdLib can be configured
            runtime: Runtime::new(config.clone()), // Assuming Runtime can be configured
            config,
        }
    }

    pub fn compile_code(&mut self, code: &str) -> Result<(), GlueError> {
        let lexer = self.lexer.get_or_insert_with(Lexer::new);
        let tokens = lexer.tokenize(code).map_err(GlueError::LexerError)?;

        let parser = self.parser.get_or_insert_with(Parser::new);
        let ast = parser.parse(&tokens).map_err(GlueError::ParserError)?;

        let compiler = self.compiler.get_or_insert_with(Compiler::new);
        let compiled_code = compiler.compile_ast_node(&ast).map_err(GlueError::CompilerError)?;
        
        self.runtime.execute(&compiled_code, &self.stdlib)
                    .map_err(GlueError::RuntimeError)?;

        Ok(())
    }

    // Additional methods for configuration, logging, etc.
    // ...
}

// Implement additional functionality as needed