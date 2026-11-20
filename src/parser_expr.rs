use std::collections::HashMap;

pub fn eval_expr(
    input: &str,
    symbols: &HashMap<String, i64>,
    cur_line: i64,
) -> Result<i64, String> {
    let tokens = tokenize_expr(input)?;
    let mut pos = 0;
    let val = parse_add_sub(&tokens, &mut pos, symbols, cur_line)?;
    if pos != tokens.len() {
        return Err(format!("Unexpected token in expression: '{}'", input.trim()));
    }
    Ok(val)
}

#[derive(Debug, Clone)]
enum Tok {
    Num(i64),
    Ident(String),
    Op(char),
    LParen,
    RParen,
}

fn tokenize_expr(s: &str) -> Result<Vec<Tok>, String> {
    let mut out = Vec::new();
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let c = bytes[i] as char;
        if c.is_ascii_whitespace() {
            i += 1;
        } else if c.is_ascii_digit() {
            let start = i;
            while i < bytes.len() && (bytes[i] as char).is_ascii_digit() {
                i += 1;
            }
            let n = s[start..i].parse::<i64>().map_err(|e| e.to_string())?;
            out.push(Tok::Num(n));
        } else if c.is_ascii_alphabetic() || c == '_' {
            let start = i;
            while i < bytes.len()
                && ((bytes[i] as char).is_ascii_alphanumeric() || bytes[i] == b'_')
            {
                i += 1;
            }
            out.push(Tok::Ident(s[start..i].to_string()));
        } else if c == '(' {
            out.push(Tok::LParen);
            i += 1;
        } else if c == ')' {
            out.push(Tok::RParen);
            i += 1;
        } else if matches!(c, '+' | '-' | '*' | '/' | '%') {
            out.push(Tok::Op(c));
            i += 1;
        } else {
            return Err(format!("Invalid character '{}' in expression", c));
        }
    }
    Ok(out)
}

fn parse_add_sub(
    toks: &[Tok],
    pos: &mut usize,
    syms: &HashMap<String, i64>,
    cur: i64,
) -> Result<i64, String> {
    let mut left = parse_mul_div(toks, pos, syms, cur)?;
    while let Some(Tok::Op(op @ ('+' | '-'))) = toks.get(*pos) {
        let op = *op;
        *pos += 1;
        let right = parse_mul_div(toks, pos, syms, cur)?;
        left = if op == '+' { left + right } else { left - right };
    }
    Ok(left)
}

fn parse_mul_div(
    toks: &[Tok],
    pos: &mut usize,
    syms: &HashMap<String, i64>,
    cur: i64,
) -> Result<i64, String> {
    let mut left = parse_unary(toks, pos, syms, cur)?;
    while let Some(Tok::Op(op @ ('*' | '/' | '%'))) = toks.get(*pos) {
        let op = *op;
        *pos += 1;
        let right = parse_unary(toks, pos, syms, cur)?;
        if (op == '/' || op == '%') && right == 0 {
            return Err("Division by zero in expression".to_string());
        }
        left = match op {
            '*' => left * right,
            '/' => left / right,
            '%' => left % right,
            _ => unreachable!(),
        };
    }
    Ok(left)
}

fn parse_unary(
    toks: &[Tok],
    pos: &mut usize,
    syms: &HashMap<String, i64>,
    cur: i64,
) -> Result<i64, String> {
    match toks.get(*pos) {
        Some(Tok::Op('+')) => {
            *pos += 1;
            parse_unary(toks, pos, syms, cur)
        }
        Some(Tok::Op('-')) => {
            *pos += 1;
            Ok(-parse_unary(toks, pos, syms, cur)?)
        }
        _ => parse_primary(toks, pos, syms, cur),
    }
}

fn parse_primary(
    toks: &[Tok],
    pos: &mut usize,
    syms: &HashMap<String, i64>,
    cur: i64,
) -> Result<i64, String> {
    match toks.get(*pos) {
        Some(Tok::Num(n)) => {
            *pos += 1;
            Ok(*n)
        }
        Some(Tok::Ident(name)) => {
            *pos += 1;
            if let Some(&v) = syms.get(name) {
                Ok(v - cur)
            } else if let Some(&v) = syms.get(&format!("__CONST_{}", name.to_uppercase())) {
                Ok(v)
            } else {
                Err(format!("Undefined label or symbol '{}'", name))
            }
        }
        Some(Tok::LParen) => {
            *pos += 1;
            let val = parse_add_sub(toks, pos, syms, cur)?;
            match toks.get(*pos) {
                Some(Tok::RParen) => {
                    *pos += 1;
                    Ok(val)
                }
                _ => Err("Missing closing ')'".to_string()),
            }
        }
        _ => Err("Expected number, label, or '('".to_string()),
    }
}
