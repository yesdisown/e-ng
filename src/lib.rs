pub mod formatter;
pub mod lexer;
pub mod macros;

use formatter::{is_structural_token, render_output};
use lexer::{Token, tokenize_cpp};
use macros::{GeneratorMode, MacroGenerator, cpp_keywords};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct ObfuscateConfig {
    pub mode: GeneratorMode,
    pub shuffle: bool,
}

impl Default for ObfuscateConfig {
    fn default() -> Self {
        Self {
            mode: GeneratorMode::from_charset("ae").expect("Default charset 'ae' is valid."),
            shuffle: true,
        }
    }
}

/// Obfuscates C++ source code while giving identical tokens one shared `#define`.
pub fn obfuscate(source: &str, config: &ObfuscateConfig) -> String {
    let tokens = tokenize_cpp(source);

    // Collect keywords and existing tokens so generated macros do not collide.
    let mut forbidden = cpp_keywords();
    for token in &tokens {
        if let Token::Code(text) = token {
            forbidden.insert(text.clone());
        }
    }

    // Keep one `#define` per unique non-structural token.
    let mut unique_obfuscatable = Vec::new();
    let mut seen = HashSet::new();
    for token in &tokens {
        if let Token::Code(text) = token {
            if !is_structural_token(text) && seen.insert(text.clone()) {
                unique_obfuscatable.push(text.clone());
            }
        }
    }

    let mut generator = MacroGenerator::new(config.mode.clone(), forbidden);
    let mut token_to_macro = HashMap::new();
    let mut defines = Vec::new();

    for tok in unique_obfuscatable {
        let macro_name = generator.next_name();
        token_to_macro.insert(tok.clone(), macro_name.clone());
        defines.push((macro_name, tok));
    }

    if config.shuffle {
        defines.shuffle(&mut thread_rng());
    }

    render_output(&tokens, &defines, &token_to_macro)
}
