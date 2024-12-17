use std::vec;

use crate::compiler::Tokens;
use crate::structs::{Compiler, Environment, Function, Interpreter};

pub fn interprete_code(compiler: &Compiler, env: &mut Environment, interpreter: &mut Interpreter) {
    //println!("{env:?}");
    let code = env.code.clone();

    while interpreter.position < code.len() {
        let Some(token) = code.get(interpreter.position) else {
            break;
        };
        match token {
            Tokens::CALL(function_id) => {
                let Some(function) = check_fn_exist(compiler, function_id.to_string()) else {
                    interpreter.position += 1;
                    continue;
                };
                let len = function.params.len();
                let mut env_vars: Vec<String> = vec![];
                let params: Vec<String> = function
                    .params
                    .iter()
                    .map(|tk| {
                        let Tokens::PARAMETER(id) = tk else {
                            return String::new();
                        };
                        return id.to_string();
                    })
                    .collect::<Vec<String>>();
                for i in 1..=len {
                    if let Some(list) = env.received_params {
                        let Some(Tokens::PARAMETER(id)) = code.get(interpreter.position + i) else {
                            interpreter.position += 1;
                            continue;
                        };
                        let Some(current_params) = env.params else {
                            continue;
                        };
                        let Some(id) = get_var_from_param(list, current_params, id.to_string())
                        else {
                            continue;
                        };
                        env_vars.push(id);
                    } else {
                        let Some(Tokens::PARAMETER(id)) = code.get(interpreter.position + i) else {
                            interpreter.position += 1;
                            continue;
                        };

                        env_vars.push(id.to_string());
                    }
                }

                if let Some(Tokens::UNTIL) = code.get(interpreter.position + len + 1) {
                    let until_id: String;
                    if let Some(list) = env.received_params {
                        let Some(params) = env.params else {
                            interpreter.position += 1;
                            continue;
                        };
                        let Some(Tokens::PARAMETER(until)) =
                            code.get(interpreter.position + len + 2)
                        else {
                            interpreter.position += 1;
                            continue;
                        };
                        let Some(until) = get_var_from_param(list, params, until.to_string())
                        else {
                            continue;
                        };
                        until_id = until;
                    } else {
                        let Some(Tokens::PARAMETER(until)) =
                            code.get(interpreter.position + len + 2)
                        else {
                            interpreter.position += 1;
                            continue;
                        };
                        until_id = until.to_string();
                    };

                    let until_val = env.variables.get(&until_id).map_or(&1, |v| v);
                    for _ in 1..=*until_val {
                        let interpreter = &mut Interpreter { position: 0 };
                        let mut env = Environment::new(
                            function.code.clone(),
                            env.variables,
                            Some(&env_vars),
                            Some(&params),
                        );
                        interprete_code(compiler, &mut env, interpreter);
                    }
                } else {
                    let interpreter = &mut Interpreter { position: 0 };
                    let mut env = Environment::new(
                        function.code.clone(),
                        &mut env.variables,
                        Some(&env_vars),
                        Some(&params),
                    );
                    interprete_code(compiler, &mut env, interpreter);
                }
                interpreter.position += 1;
                continue;
            }
            Tokens::VAR((var_id, value)) => {
                env.variables.insert(var_id.clone(), *value);
                interpreter.position += 1;
                continue;
            }
            Tokens::PARAMETER(parameter_id) => {
                let Some(list) = env.received_params else {
                    interpreter.position += 1;
                    continue;
                };
                let Some(params) = env.params else {
                    interpreter.position += 1;
                    continue;
                };
                let Some(parameter_id) = get_var_from_param(list, params, parameter_id.to_string())
                else {
                    continue;
                };
                let Some(&mut ref mut var) = env.variables.get_mut(&parameter_id) else {
                    interpreter.position += 1;
                    continue;
                };
                for tok in &code[interpreter.position..] {
                    match tok {
                        Tokens::INCREMENT => {
                            *var += 1;
                        }
                        Tokens::DECREMENT => *var -= 1,
                        _ => continue,
                    }
                }
                interpreter.position += 1
            }
            _ => {
                interpreter.position += 1;
                continue;
            }
        }
    }
}

fn check_fn_exist(compiler: &Compiler, function_id: String) -> Option<Function> {
    for function in &compiler.functions {
        let Tokens::FUNCTION_ID(id) = &function.id else {
            return None;
        };
        if *id == function_id {
            return Some(function.clone());
        } else {
            continue;
        }
    }
    None
}

fn get_var_from_param(rec_params: &[String], params: &[String], id: String) -> Option<String> {
    let Some(param_pos) = params.iter().position(|c| *c == id) else {
        println!("no parameter was found with id: {id}, {params:?}");
        return None;
    };
    let Some(var_id) = rec_params.get(param_pos) else {
        println!(
            "no received parameter was found with pos: {param_pos}, {params:?}, {rec_params:?}"
        );
        return None;
    };
    return Some(var_id.to_string());
}
