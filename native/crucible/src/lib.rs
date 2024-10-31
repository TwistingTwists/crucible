
mod errors;
mod types;

// mod imports;
mod semantic_imports;

use anyhow::anyhow;
// use rustler::{Encoder, Env, NifResult, Term};
use oxc_allocator::Allocator;
use oxc_parser::{ParseOptions, Parser};
use oxc_span::SourceType;
use oxc_ast::ast::Program;
use rustler::ResourceArc;
use crate::types::Result;
use crate::errors::CrucibleError;

use std::sync::{Arc, Mutex, RwLock};

pub struct ParseResult<'a> {
    pub program: Program<'a>,
    pub errors: Vec<oxc_diagnostics::OxcDiagnostic>,
    pub allocator: &'a Allocator,
}

pub fn parse_source<'a>(allocator: &'a Allocator, source_code: &'a str) -> anyhow::Result<ParseResult<'a>> {
    let source_type = SourceType::default();

    let parse_result = Parser::new(allocator, source_code, source_type).parse();

    if !parse_result.errors.is_empty() {
        return Err(anyhow::anyhow!("Parsing errors encountered".to_string()));
    }

    Ok(ParseResult {
        program: parse_result.program,
        errors: parse_result.errors,
        allocator,
    })
}


#[derive(Debug)]
pub struct ASTResourceRef<'a> {
    // pub inner: RwLock<oxc_parser::ParserReturn<'a>>,
    pub inner: Arc<RwLock<oxc_ast::ast::Program<'a>>>,
}


// #[derive(rustler::NifStruct)]
// #[module = "Crucible.AST"]
// pub struct ASTResource<'static> {
//     pub resource: ResourceArc<ASTResourceRef<'static>>,
// }

// #[rustler::resource_impl]
// impl rustler::Resource for ASTResourceRef<'static> {}


// #[rustler::nif]
fn parse_javascript(source: String) -> Result<String> {
    let allocator = Allocator::default();
    let source_type = SourceType::default();

    let ret = Parser::new(&allocator, &source, source_type)
        .with_options(ParseOptions { parse_regular_expression: true, ..ParseOptions::default() })
        .parse();

    if ret.errors.is_empty() {
        let ast_json = serde_json::to_string_pretty(&ret.program)
            .map_err(|e| CrucibleError::SerdeJson(e))?;
        Ok(ast_json)
    } else {
        let error_messages: Vec<String> = ret.errors
            .iter()
            .map(|error| format!("{:?}", error.clone().with_source_code(source.clone())))
            .collect();
        Err(CrucibleError::Internal(error_messages.join("\n")))
    }
}

// #[rustler::nif]
fn get_comments(source: String) -> Result<Vec<String>> {
    let allocator = Allocator::default();
    let source_type = SourceType::default();

    let ret = Parser::new(&allocator, &source, source_type).parse();

    let comments: Vec<String> = ret.program.comments
        .iter()
        .map(|comment| comment.real_span().source_text(&source).to_string())
        .collect();

    Ok(comments)
}

// rustler::init!("Elixir.Crucible");
