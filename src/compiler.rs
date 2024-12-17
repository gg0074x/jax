use std::{collections::HashMap, vec};

use crate::structs::*;

#[derive(Debug, Clone)]
pub enum Tokens {
    FUNCTION_DEC,
    FUNCTION_ID(String),
    PARAMETERS,
    PARAMETER(String),
    START,
    END,
    INCREMENT,
    DECREMENT,
    CALL(String),
    UNTIL,
    VAR((String, i32)),
}

pub fn compile_rules(rules: &str, compiler: &mut Compiler) {
    let mut functions: Vec<Function> = vec![];
    let rules = rules.split_whitespace();

    let mut inside_code = false;

    let mut id: Option<Tokens> = None;
    let mut params: Vec<Tokens> = vec![];
    let mut code: Vec<Tokens> = vec![];

    for rule in rules {
        if rule.starts_with('"') {
            let rule = rule.replace('"', "");
            id = Some(Tokens::FUNCTION_ID(rule));
            continue;
        }
        if rule.starts_with(('A'..='Z').collect::<Vec<char>>().as_slice()) && inside_code {
            code.push(Tokens::CALL(rule.to_string()));
            continue;
        }
        if rule.len() == 1 {
            let rule = rule.chars().last().unwrap();
            match rule {
                'a'..='z' => {
                    if inside_code {
                        code.push(Tokens::PARAMETER(rule.to_string()));
                    } else {
                        params.push(Tokens::PARAMETER(rule.to_string()));
                    }
                    continue;
                }
                '<' => {
                    inside_code = true;
                    continue;
                }
                '>' => {
                    inside_code = false;
                    let Some(use_id) = id else {
                        return;
                    };
                    functions.push(Function {
                        id: use_id,
                        params: params,
                        code: code,
                    });
                    id = None;
                    params = vec![];
                    code = vec![];
                    continue;
                }
                '+' => {
                    if inside_code {
                        code.push(Tokens::INCREMENT);
                    }
                    continue;
                }
                '-' => {
                    if inside_code {
                        code.push(Tokens::DECREMENT);
                    }
                    continue;
                }
                '?' => {
                    if inside_code {
                        code.push(Tokens::UNTIL);
                    }
                    continue;
                }
                _ => {
                    continue;
                }
            }
        }
    }

    compiler.functions = functions;
}

pub fn compile_code<'a>(
    code: &'a str,
    compiler: &'a Compiler,
    vars: &'a mut HashMap<String, i32>,
) -> Environment<'a> {
    let mut env_code: Vec<Tokens> = vec![];
    let statements: Vec<String> = code
        .split_whitespace()
        .map(std::string::ToString::to_string)
        .collect();

    for (i, statement) in statements.clone().into_iter().enumerate() {
        if statement.starts_with(('A'..='Z').collect::<Vec<char>>().as_slice()) {
            for function in &compiler.functions {
                if let Tokens::FUNCTION_ID(x) = &function.id {
                    if statement == *x {
                        env_code.push(Tokens::CALL(statement.to_string()));
                    }
                }
            }
            continue;
        }
        if statement.len() == 1 {
            let rule = statement.chars().last().unwrap();
            match rule {
                'a'..='z' => {
                    let Some(equal) = statements.get(i + 1) else {
                        env_code.push(Tokens::PARAMETER(statement.to_string()));
                        continue;
                    };
                    if equal == "=" {
                        let Some(val) = statements.get(i + 2) else {
                            continue;
                        };
                        if let Ok(n) = val.parse() {
                            env_code.push(Tokens::VAR((rule.to_string(), n)));
                        } else {
                            todo!()
                        }
                    } else {
                        env_code.push(Tokens::PARAMETER(statement.to_string()));
                    }
                    continue;
                }
                '?' => {
                    env_code.push(Tokens::UNTIL);
                    continue;
                }
                _ => {
                    continue;
                }
            }
        }
    }

    let env: Environment = Environment::new(env_code, vars, None, None);
    return env;
}
