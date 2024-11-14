mod errors;
mod types;

// mod imports;
mod semantic_imports;

use anyhow::anyhow;
use rustler::{Encoder, Env, NifResult, Term};
use crate::errors::CrucibleError;
use crate::types::CrucibleResult;
use oxc_allocator::Allocator;
use oxc_ast::ast::Program;
use oxc_parser::{ParseOptions, Parser};
use oxc_span::SourceType;
use rustler::ResourceArc;

use std::sync::{Arc, Mutex, RwLock};


// #[rustler::nif]
// fn parse_js_from_file(path: String) -> Result<String> {
//     let source = std::fs::read_to_string(path)?;
//     parse_javascript(source)
// }

// #[rustler::nif]
// fn parse_js_from_buffer(buffer: String) -> Result<String> {
//     parse_javascript(buffer)
// }


pub struct ParseResult<'a> {
    pub program: Program<'a>,
    pub errors: Vec<oxc_diagnostics::OxcDiagnostic>,
    pub allocator: &'a Allocator,
}

pub fn parse_source<'a>(
    allocator: &'a Allocator,
    source_code: &'a str,
) -> Result<ParseResult<'a>, rustler::Error> {
    let source_type = SourceType::default();

    let parse_result = Parser::new(allocator, source_code, source_type).parse();

    if !parse_result.errors.is_empty() {
        // return Err(CrucibleError::Other("Parsing errors encountered".to_string()));
        return Err(rustler::Error::BadArg);
    }

    Ok(ParseResult {
        program: parse_result.program,
        errors: parse_result.errors,
        allocator,
    })
}
fn on_load(_env: Env, _info: Term) -> bool {
    true
}

rustler::init!("Elixir.Crucible.Native", load = on_load);
