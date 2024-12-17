use std::{collections::HashMap, fs};

use compiler::{compile_code, compile_rules};
use interpreter::interprete_code;
use structs::{Compiler, Interpreter};

mod compiler;
mod interpreter;
mod structs;

fn main() {
    let rules: String = fs::read_to_string("./examples/DARULES!").unwrap();
    let code: String = fs::read_to_string("./examples/example.jax").unwrap();
    let mut compiler = Compiler { functions: vec![] };
    compile_rules(&rules, &mut compiler);
    let mut vars: HashMap<String, i32> = HashMap::new();
    let mut env = compile_code(&code, &compiler, &mut vars);
    let interpreter = &mut Interpreter { position: 0 };
    interprete_code(&compiler, &mut env, interpreter);
    //println!("{env:?}")
    for (var_id, val) in env.variables{
        println!("{var_id}: {val}")
    }
}
