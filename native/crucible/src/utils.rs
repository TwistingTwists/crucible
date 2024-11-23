use std::path::Path;

use oxc_allocator::Allocator;
use oxc_semantic::{Semantic, SemanticBuilder};


use crate::errors::CrucibleError;
use crate::types::CrucibleResult;
use oxc_ast::ast::Program;
use oxc_parser::{ParseOptions, Parser};
use oxc_span::SourceType;
use rustler::ResourceArc;


// pub fn parse_semantic(source_code: &str, filename: &str) -> Result<Semantic<'static>, rustler::Error>{
//     let allocator = Allocator::default();
//     let ret = parse_source(&allocator, source_code)?;
//     let path = Path::new(filename);

//     let semantic_builder_ret = SemanticBuilder::new()
//         .build_module_record(path, &ret.program)
//         .with_check_syntax_error(true)
//         .build(&ret.program);

//     Ok(semantic_builder_ret.semantic)
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