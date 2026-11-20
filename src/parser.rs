use std::collections::HashMap;
use crate::mars_math::wrap;
use crate::parser_expr::eval_expr;
use crate::parser_line::{default_modifier, is_keyword, parse_modifier, parse_opcode, split_mode_and_expr};
use crate::types::{Instruction, MarsConfig, Mode, Opcode, ParseError, WarriorProgram};

struct RawInst {
    line: usize,
    op: Opcode,
    mod_opt: Option<crate::types::Modifier>,
    a_raw: String,
    b_raw: Option<String>,
}

pub fn assemble(source: &str, cfg: &MarsConfig) -> Result<WarriorProgram, ParseError> {
    let mut name = String::from("Unnamed");
    let mut author = String::from("Unknown");
    let mut strategy = String::new();
    let mut equs: HashMap<String, String> = HashMap::new();
    let mut symbols: HashMap<String, i64> = HashMap::new();
    insert_builtin_consts(&mut symbols, cfg);

    let mut raw_insts: Vec<RawInst> = Vec::new();
    let mut org_expr: Option<(usize, String)> = None;

    for (idx, raw_line) in source.lines().enumerate() {
        let line_no = idx + 1;
        if let Some(comment) = raw_line.split(';').nth(1) {
            parse_meta_comment(comment.trim(), &mut name, &mut author, &mut strategy);
        }
        let code = raw_line.split(';').next().unwrap_or("").trim();
        if code.is_empty() {
            continue;
        }
        let code_exp = expand_equs(code, &equs);
        let parts: Vec<&str> = code_exp.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        let mut cur_idx = 0;
        if parts.len() >= 2 && parts[1].eq_ignore_ascii_case("EQU") {
            let eq_pos = code_exp.to_ascii_uppercase().find("EQU").unwrap();
            let rhs = code_exp[eq_pos + 3..].trim();
            equs.insert(parts[0].to_string(), rhs.to_string());
            continue;
        }

        while cur_idx < parts.len() {
            let tok = parts[cur_idx].trim_end_matches(':');
            let base = tok.split('.').next().unwrap_or(tok);
            if is_keyword(base) {
                break;
            }
            symbols.insert(tok.to_string(), raw_insts.len() as i64);
            cur_idx += 1;
        }
        if cur_idx >= parts.len() {
            continue;
        }

        let op_tok = parts[cur_idx];
        let rest = parts[cur_idx + 1..].join(" ");
        let op_upper = op_tok.to_ascii_uppercase();
        if op_upper == "ORG" {
            org_expr = Some((line_no, rest));
            continue;
        } else if op_upper == "END" {
            if !rest.trim().is_empty() {
                org_expr = Some((line_no, rest));
            }
            break;
        } else if op_upper == "PIN" {
            continue;
        }

        let mut op_parts = op_tok.split('.');
        let op_name = op_parts.next().unwrap_or("");
        let op = parse_opcode(op_name).ok_or_else(|| ParseError {
            line: line_no,
            message: format!("Unknown opcode '{}'", op_name),
        })?;
        let mod_opt = match op_parts.next() {
            Some(m) => Some(parse_modifier(m).ok_or_else(|| ParseError {
                line: line_no,
                message: format!("Invalid modifier '.{}'", m),
            })?),
            None => None,
        };
        let (a_raw, b_raw) = split_operands(&rest, line_no)?;
        raw_insts.push(RawInst { line: line_no, op, mod_opt, a_raw, b_raw });
    }

    if raw_insts.is_empty() {
        return Err(ParseError { line: 1, message: "Warrior has no instructions".to_string() });
    }
    if raw_insts.len() > cfg.max_length as usize {
        return Err(ParseError {
            line: raw_insts.last().map(|r| r.line).unwrap_or(1),
            message: format!("Warrior exceeds max length of {}", cfg.max_length),
        });
    }

    let mut instructions = Vec::with_capacity(raw_insts.len());
    for (pc, r) in raw_insts.iter().enumerate() {
        symbols.insert("__CONST_CURLINE".to_string(), pc as i64);
        let (a_mode, a_expr) = split_mode_and_expr(&r.a_raw);
        let (b_mode, b_expr_opt) = match &r.b_raw {
            Some(b) => {
                let (m, e) = split_mode_and_expr(b);
                (m, Some(e))
            }
            None => (Mode::Immediate, None),
        };
        let (final_a_mode, final_a_val, final_b_mode, final_b_val) = if r.op == Opcode::Dat && b_expr_opt.is_none() {
            let v = eval_expr(a_expr, &symbols, pc as i64).map_err(|e| ParseError { line: r.line, message: e })?;
            (Mode::Immediate, 0, a_mode, wrap(v, cfg.core_size))
        } else {
            let av = eval_expr(a_expr, &symbols, pc as i64).map_err(|e| ParseError { line: r.line, message: e })?;
            let bv = match b_expr_opt {
                Some(be) => eval_expr(be, &symbols, pc as i64).map_err(|e| ParseError { line: r.line, message: e })?,
                None => 0,
            };
            (a_mode, wrap(av, cfg.core_size), b_mode, wrap(bv, cfg.core_size))
        };
        let modifier = r.mod_opt.unwrap_or_else(|| default_modifier(r.op, final_a_mode, final_b_mode));
        instructions.push(Instruction {
            opcode: r.op,
            modifier,
            a_mode: final_a_mode,
            a_val: final_a_val,
            b_mode: final_b_mode,
            b_val: final_b_val,
        });
    }

    let org = if let Some((line, expr)) = org_expr {
        let v = eval_expr(&expr, &symbols, 0).map_err(|e| ParseError { line, message: e })?;
        if v < 0 || v as usize >= instructions.len() {
            return Err(ParseError { line, message: format!("ORG offset {} out of bounds", v) });
        }
        v as usize
    } else {
        0
    };

    Ok(WarriorProgram { name, author, strategy, org, instructions })
}

fn insert_builtin_consts(syms: &mut HashMap<String, i64>, cfg: &MarsConfig) {
    syms.insert("__CONST_CORESIZE".into(), cfg.core_size as i64);
    syms.insert("__CONST_MAXCYCLES".into(), cfg.max_cycles as i64);
    syms.insert("__CONST_MAXPROCESSES".into(), cfg.max_processes as i64);
    syms.insert("__CONST_MAXLENGTH".into(), cfg.max_length as i64);
    syms.insert("__CONST_MINDISTANCE".into(), cfg.min_distance as i64);
    syms.insert("__CONST_WARRIORS".into(), 2);
}

fn parse_meta_comment(c: &str, name: &mut String, author: &mut String, strategy: &mut String) {
    let lower = c.to_ascii_lowercase();
    if lower.starts_with("name ") {
        *name = c[5..].trim().to_string();
    } else if lower.starts_with("author ") {
        *author = c[7..].trim().to_string();
    } else if lower.starts_with("strategy ") {
        if !strategy.is_empty() { strategy.push(' '); }
        strategy.push_str(c[9..].trim());
    }
}

fn expand_equs(code: &str, equs: &HashMap<String, String>) -> String {
    let mut out = code.to_string();
    for _ in 0..4 {
        let mut changed = false;
        for (k, v) in equs {
            if out.contains(k.as_str()) {
                let replaced = out.replace(k.as_str(), v.as_str());
                if replaced != out {
                    out = replaced;
                    changed = true;
                }
            }
        }
        if !changed { break; }
    }
    out
}

fn split_operands(rest: &str, line: usize) -> Result<(String, Option<String>), ParseError> {
    let trimmed = rest.trim();
    if trimmed.is_empty() {
        return Err(ParseError { line, message: "Missing operand(s)".to_string() });
    }
    if let Some((a, b)) = trimmed.split_once(',') {
        Ok((a.trim().to_string(), Some(b.trim().to_string())))
    } else {
        Ok((trimmed.to_string(), None))
    }
}
