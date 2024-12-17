use crate::compiler::Tokens;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Function {
    pub id: Tokens,
    pub params: Vec<Tokens>,
    pub code: Vec<Tokens>,
}

#[derive(Debug)]
pub struct Compiler {
    pub functions: Vec<Function>,
}

#[derive(Debug)]
pub struct Environment<'a> {
    pub variables: &'a mut HashMap<String, i32>,
    pub code: Vec<Tokens>,
    pub received_params: Option<&'a [String]>,
    pub params: Option<&'a [String]>
}

impl<'a> Environment<'a> {
    pub fn new(
        code: Vec<Tokens>,
        vars: &'a mut HashMap<String, i32>,
        sent_params: Option<&'a [String]>,
        params: Option<&'a [String]>,
    ) -> Self {
        Environment {
            variables: vars,
            code,
            received_params: sent_params,
            params
        }
    }
}

#[derive(Debug)]
pub struct Interpreter {
    pub position: usize,
}
