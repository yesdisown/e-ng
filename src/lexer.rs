use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Token {
    /// Preprocessor directives, eg. `#include <iostream>` or existing `#define`s.
    Directive(String),
    /// Comments (`//` and `/* ... */`).
    Comment(String),
    /// Actual fucking code.
    Code(String),
}

/// Tokenizes C++ source code into individual tokens

#[derive(Debug, Clone)]
pub enum GeneratorMode {
    /// Character set treated like a base-N alphabet (default: `"ae"`).
    Charset(Vec<char>),
    /// List of phrases combined into macro names.
    Phraseset(Vec<String>),
}

impl GeneratorMode {
    /// Builds and validates a `Charset` mode.
    pub fn from_charset(raw: &str) -> Result<Self, String> {
        let mut chars: Vec<char> = raw.chars().collect();
        chars.sort_unstable();
        chars.dedup();

        if chars.is_empty() {
            return Err("Charset cannot be empty.".into());
        }
        if !chars.iter().all(|c| c.is_ascii_alphabetic() || *c == '_') {
            return Err("Charset can only contain ASCII letters (a-z, A-Z) and '_'.".into());
        }

        Ok(Self::Charset(chars))
    }

    /// Builds and validates a `Phraseset` mode from lines in a dictionary file.
    pub fn from_phrases(phrases: Vec<String>) -> Result<Self, String> {
        let mut cleaned: Vec<String> = phrases
            .into_iter()
            .map(|p| p.trim().to_string())
            .filter(|p| !p.is_empty())
            .collect();
        cleaned.sort();
        cleaned.dedup();

        if cleaned.is_empty() {
            return Err("Phraseset file has no non-empty phrases.".into());
        }
        if !cleaned
            .iter()
            .all(|p| p.chars().all(|c| c.is_ascii_alphanumeric() || c == '_'))
        {
            return Err("Phrases can only contain ASCII alphanumerics and '_'.".into());
        }

        Ok(Self::Phraseset(cleaned))
    }
}

pub struct MacroGenerator {
    mode: GeneratorMode,
    forbidden: HashSet<String>,
    counter: usize,
}

impl MacroGenerator {
    pub fn new(mode: GeneratorMode, forbidden: HashSet<String>) -> Self {
        Self {
            mode,
            forbidden,
            counter: 1,
        }
    }

    /// Returns the next macro name that doesn't collide with keywords or existing code tokens.
    pub fn next_name(&mut self) -> String {
        loop {
            let candidate = match &self.mode {
                GeneratorMode::Charset(alphabet) => {
                    let base = alphabet.len();
                    let mut n = self.counter;
                    let mut out = String::new();
                    while n > 0 {
                        n -= 1;
                        out.push(alphabet[n % base]);
                        n /= base;
                    }
                    out.chars().rev().collect()
                }
                GeneratorMode::Phraseset(phrases) => {
                    let base = phrases.len();
                    let mut n = self.counter;
                    let mut chunks = Vec::new();
                    while n > 0 {
                        n -= 1;
                        chunks.push(phrases[n % base].clone());
                        n /= base;
                    }
                    chunks.reverse();
                    chunks.concat()
                }
            };

            self.counter += 1;

            // C++ macros can't start with a digit.
            let starts_with_digit = candidate
                .chars()
                .next()
                .map(|c| c.is_ascii_digit())
                .unwrap_or(true);

            if !starts_with_digit && !self.forbidden.contains(&candidate) {
                return candidate;
            }
        }
    }
}

/// C++ keywords we never want to overwrite with a macro.
pub fn cpp_keywords() -> HashSet<String> {
    [
        "alignas",
        "alignof",
        "and",
        "and_eq",
        "asm",
        "auto",
        "bitand",
        "bitor",
        "bool",
        "break",
        "case",
        "catch",
        "char",
        "char8_t",
        "char16_t",
        "char32_t",
        "class",
        "compl",
        "concept",
        "const",
        "consteval",
        "constexpr",
        "constinit",
        "const_cast",
        "continue",
        "co_await",
        "co_return",
        "co_yield",
        "decltype",
        "default",
        "delete",
        "do",
        "double",
        "dynamic_cast",
        "else",
        "enum",
        "explicit",
        "export",
        "extern",
        "false",
        "float",
        "for",
        "friend",
        "goto",
        "if",
        "inline",
        "int",
        "long",
        "mutable",
        "namespace",
        "new",
        "noexcept",
        "not",
        "not_eq",
        "nullptr",
        "operator",
        "or",
        "or_eq",
        "private",
        "protected",
        "public",
        "register",
        "reinterpret_cast",
        "requires",
        "return",
        "short",
        "signed",
        "sizeof",
        "static",
        "static_assert",
        "static_cast",
        "struct",
        "switch",
        "template",
        "this",
        "thread_local",
        "throw",
        "true",
        "try",
        "typedef",
        "typeid",
        "typename",
        "union",
        "unsigned",
        "using",
        "virtual",
        "void",
        "volatile",
        "wchar_t",
        "while",
        "xor",
        "xor_eq",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect()
}
pub fn tokenize_cpp(source: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = source.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        // Ignore whitespace characters.
        if chars[i].is_whitespace() {
            i += 1;
            continue;
        }

        // Preprocessor directives.
        if chars[i] == '#' {
            let start = i;
            while i < len {
                if chars[i] == '\\' && i + 1 < len && chars[i + 1] == '\n' {
                    i += 2;
                } else if chars[i] == '\n' {
                    break;
                } else {
                    i += 1;
                }
            }
            let dir: String = chars[start..i].iter().collect();
            tokens.push(Token::Directive(dir));
            continue;
        }

        // Single-line comments.
        if chars[i] == '/' && i + 1 < len && chars[i + 1] == '/' {
            let start = i;
            while i < len && chars[i] != '\n' {
                i += 1;
            }
            let comment: String = chars[start..i].iter().collect();
            tokens.push(Token::Comment(comment));
            continue;
        }

        // Block comments.
        if chars[i] == '/' && i + 1 < len && chars[i + 1] == '*' {
            let start = i;
            i += 2;
            while i + 1 < len && !(chars[i] == '*' && chars[i + 1] == '/') {
                i += 1;
            }
            i = (i + 2).min(len);
            let comment: String = chars[start..i].iter().collect();
            tokens.push(Token::Comment(comment));
            continue;
        }

        // String and char literals as a single token so spaces/quotes don't blow tf up.
        if chars[i] == '"' || chars[i] == '\'' {
            let quote = chars[i];
            let start = i;
            i += 1;
            while i < len {
                if chars[i] == '\\' {
                    i += 2;
                } else if chars[i] == quote {
                    i += 1;
                    break;
                } else {
                    i += 1;
                }
            }
            let lit: String = chars[start..i].iter().collect();
            tokens.push(Token::Code(lit));
            continue;
        }

        // Identifiers and numbers.
        if chars[i].is_alphanumeric() || chars[i] == '_' {
            let start = i;
            while i < len && (chars[i].is_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            let word: String = chars[start..i].iter().collect();
            tokens.push(Token::Code(word));
            continue;
        }

        // Multi-char operators.
        if i + 1 < len {
            let two: String = chars[i..=i + 1].iter().collect();
            let multi_ops = [
                "<<", ">>", "::", "==", "!=", "<=", ">=", "&&", "||", "->", "++", "--", "+=", "-=",
                "*=", "/=", "%=", "&=", "|=", "^=",
            ];
            if multi_ops.contains(&two.as_str()) {
                tokens.push(Token::Code(two));
                i += 2;
                continue;
            }
        }

        // Everything else (single punctuation / operators).
        tokens.push(Token::Code(chars[i].to_string()));
        i += 1;
    }

    tokens
}
