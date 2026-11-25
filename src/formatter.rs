use crate::lexer::Token;
use std::collections::HashMap;

/// Keeps structural tokens and operators raw so expressions like `6 + 8` stay `aa + ae`.
pub fn is_structural_token(tok: &str) -> bool {
    matches!(
        tok,
        "(" | ")"
            | "{"
            | "}"
            | "["
            | "]"
            | ";"
            | ","
            | ":"
            | "::"
            | "."
            | "?"
            | "+"
            | "-"
            | "*"
            | "/"
            | "%"
            | "="
            | "=="
            | "!="
            | "<"
            | ">"
            | "<="
            | ">="
            | "&&"
            | "||"
            | "!"
            | "&"
            | "|"
            | "^"
            | "~"
            | "<<"
            | ">>"
            | "++"
            | "--"
            | "+="
            | "-="
            | "*="
            | "/="
            | "%="
            | "&="
            | "|="
            | "^="
            | "->"
    )
}

/// Renders directives, `#define`s, and the obfuscated body with basic indentation.
pub fn render_output(
    tokens: &[Token],
    defines: &[(String, String)],
    token_to_macro: &HashMap<String, String>,
) -> String {
    let mut output = String::new();

    // Put preprocessor directives at the top first.
    for token in tokens {
        if let Token::Directive(dir) = token {
            output.push_str(dir);
            output.push('\n');
        }
    }

    // Emit shared `#define` macros.
    for (macro_name, original) in defines {
        output.push_str(&format!("#define {} {}\n", macro_name, original));
    }

    if !defines.is_empty() {
        output.push('\n');
    }

    // Reconstruct body with simple brace and semicolon indentation.
    let code_tokens: Vec<&str> = tokens
        .iter()
        .filter_map(|t| match t {
            Token::Code(s) => Some(s.as_str()),
            _ => None,
        })
        .collect();

    let mut indent: usize = 0;
    let mut at_line_start = true;
    let mut prev_no_space_after = false;

    for (idx, &tok) in code_tokens.iter().enumerate() {
        let rendered = token_to_macro.get(tok).map(String::as_str).unwrap_or(tok);

        if tok == "}" {
            indent = indent.saturating_sub(1);
            if !at_line_start {
                output.push('\n');
            }
            output.push_str(&"  ".repeat(indent));
            output.push_str(rendered);
            output.push('\n');
            at_line_start = true;
            prev_no_space_after = false;
            continue;
        }

        if at_line_start {
            output.push_str(&"  ".repeat(indent));
            at_line_start = false;
        } else if !prev_no_space_after
            && !matches!(tok, "(" | ")" | ";" | "," | "[" | "]")
            && code_tokens
                .get(idx.saturating_sub(1))
                .is_none_or(|prev| !matches!(*prev, "(" | "["))
        {
            output.push(' ');
        }

        output.push_str(rendered);
        prev_no_space_after = matches!(tok, "(" | "[");

        if tok == "{" {
            indent += 1;
            output.push('\n');
            at_line_start = true;
            prev_no_space_after = false;
        } else if tok == ";" {
            output.push('\n');
            at_line_start = true;
            prev_no_space_after = false;
        }
    }

    output
}
