



use crate::{
    diag,
    diagnostics::Diagnostic,
    expansion::{
        ast::{self as E, Address, Fields, ModuleIdent, ModuleIdent_, SpecId},
    },
    parser::ast::{
        self as P, Ability, ConstantName, Field, FunctionName, ModuleName, StructName, Var,
    },
    shared::{known_attributes::AttributePosition, unique_map::UniqueMap, *},
    FullyCompiledProgram,
};
use move_command_line_common::parser::{parse_u16, parse_u256, parse_u32};
use move_ir_types::location::*;
use move_symbol_pool::Symbol;
use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    iter::IntoIterator,
};




//**************************************************************************************************
// Program
//**************************************************************************************************


pub fn program(
    compilation_env: &mut CompilationEnv,
    pre_compiled_lib: Option<&FullyCompiledProgram>,
    prog: E::Program,
) -> E::Program {
    println!("---- --- vmove program run.");

    

    prog
}