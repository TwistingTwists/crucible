
mod errors;
mod types;

// use rustler::{Encoder, Env, NifResult, Term};
use oxc_allocator::Allocator;
use oxc_parser::{ParseOptions, Parser};
use oxc_span::SourceType;
use crate::types::{Result};
use crate::errors::CrucibleError;

#[rustler::nif]
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

#[rustler::nif]
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

rustler::init!("Elixir.Crucible");
