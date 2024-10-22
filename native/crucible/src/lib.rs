// use rustler::{Encoder, Env, NifResult, Term};
// use oxc_allocator::Allocator;
// use oxc_parser::{Parser, ParserReturn};
// use oxc_span::SourceType;
// use serde_json::Value;

// #[rustler::nif]
// fn parse_javascript(source: String) -> NifResult<String> {
//     let allocator = Allocator::default();
//     let source_type = SourceType::default();

//     let ParserReturn { program, .. } = Parser::new(&allocator, &source, source_type).parse();

//     // Convert the AST to JSON for easier handling in Elixir
//     let ast_json = serde_json::to_string(&program).map_err(|e| rustler::Error::Term(Box::new(e.to_string())))?;

//     Ok(ast_json)
// }

// rustler::init!("Elixir.Crucible",[parse_javascript]);

// -----------------------

// use oxc_ast::AstNode; // Assuming oxc_ast defines the structure of the AST
// use oxc_parser::Parser; // To parse JavaScript
// use serde::Serialize;
// use serde_json;

// // The JavaScript code we want to parse
// const JS_CODE: &str = r#"
// function add(a, b) {
//     return a + b;
// }
// "#;

// fn main() {
//     // Step 1: Parse JavaScript and create an AST
//     let parser = Parser::default();
//     let ast = parser.parse(JS_CODE).unwrap();

//     // Step 2: Serialize the AST into JSON
//     let json_ast = serde_json::to_string_pretty(&ast).unwrap();

//     // Step 3: Print the JSON representation of the AST
//     println!("{}", json_ast);
// }

// -----------------------


use rustler::{Encoder, Env, NifResult, Term};
use oxc_allocator::Allocator;
use oxc_parser::{ParseOptions, Parser};
use oxc_span::SourceType;
use serde_json::Value;

#[rustler::nif]
fn parse_javascript(source: String) -> NifResult<String> {
    let allocator = Allocator::default();
    let source_type = SourceType::default();

    let ret = Parser::new(&allocator, &source, source_type)
        .with_options(ParseOptions { parse_regular_expression: true, ..ParseOptions::default() })
        .parse();

    if ret.errors.is_empty() {
        // Convert the AST to JSON for easier handling in Elixir
        let ast_json = serde_json::to_string_pretty(&ret.program)
            .map_err(|e| rustler::Error::Term(Box::new(e.to_string())))?;
        Ok(ast_json)
    } else {
        let error_messages: Vec<String> = ret.errors.iter()
            .map(|error| format!("{:?}", error.clone().with_source_code(source.clone())))
            .collect();
        Err(rustler::Error::Term(Box::new(error_messages.join("\n"))))
    }
}

#[rustler::nif]
fn get_comments(source: String) -> NifResult<Vec<String>> {
    let allocator = Allocator::default();
    let source_type = SourceType::default();

    let ret = Parser::new(&allocator, &source, source_type).parse();

    let comments: Vec<String> = ret.program.comments
        .iter()
        .map(|comment| comment.real_span().source_text(&source).to_string())
        .collect();

    Ok(comments)
}

rustler::init!("Elixir.Crucible", [parse_javascript, get_comments]);
