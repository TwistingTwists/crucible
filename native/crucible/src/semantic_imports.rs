use oxc_allocator::Allocator;
use crate::{errors::CrucibleError, parse_source};
use oxc_semantic::SemanticBuilder;
use oxc_span::{SourceType, Span};
use oxc_syntax::module_record::ImportEntry;
use rustler::ResourceArc;
use std::path::Path;

////////////////////////////
// structs
////////////////////////////

#[derive(rustler::NifStruct)]
#[module = "Crucible.Imports"]
pub struct Imports {
    pub resource: ResourceArc<SemanticImportsRef>,
}
pub struct SemanticImportsRef(pub ImportEntries);

#[rustler::resource_impl]
impl rustler::Resource for SemanticImportsRef {}

#[derive(Debug)]
pub struct ImportEntries(Vec<ImportEntry>);

impl From<ImportEntries> for Imports {
    fn from(data: ImportEntries) -> Self {
        Self {
            resource: rustler::ResourceArc::new(SemanticImportsRef(data)),
        }
    }
}

/////////////////////
/// creators
////////////////////

#[rustler::nif]
pub fn ast_for_imports_from_buffer(
    source_code: &str,
    filename: &str,
) -> Result<Imports, rustler::Error> {
    let allocator = Allocator::default();
    let ret = parse_source(&allocator, source_code)?;
    let path = Path::new(filename);

    let semantic_builder_ret = SemanticBuilder::new()
        .build_module_record(path, &ret.program)
        .with_check_syntax_error(true)
        .build(&ret.program);

    let semantic = semantic_builder_ret.semantic;
    let import_entries_1 = ImportEntries(semantic.module_record().import_entries.clone());
    dbg!(&import_entries_1);
    Ok(import_entries_1.into())
}

#[rustler:: nif]
pub fn local_name_exists(ast_for_import: Imports , import_name: String) -> bool{
    let import_entries = ast_for_import.resource.0.0.iter();
    let matches = import_entries
    .filter(|entry| (*entry.local_name.name()) == import_name)
    .collect::<Vec<_>>();

    // dbg!(&matches);

    !matches.is_empty()
}


/////////////////////
/// getters
////////////////////


/////////////////////
/// setters
////////////////////



#[cfg(test)]
mod semantic_import_tests {
    use super::*;
    use oxc_syntax::module_record::ImportImportName;

    #[test]
    fn test_basic_imports() {
        let js_code = r#"
            import { Socket } from "phoenix.js";
            import React from 'react';
        "#;

        let result = analyze_imports(js_code, "test.js").unwrap();
        assert_eq!(result.len(), 2);
        assert!(result
            .iter()
            .any(|info| {
                dbg!(info);
                info.module_request.name() == "phoenix.js"}));
        assert!(result
            .iter()
            .any(|info| info.module_request.name() == "react"));
    }

    #[test]
    fn test_no_imports() {
        let js_code = r#"
            const x = 1;
            console.log(x);
        "#;

        let result = analyze_imports(js_code, "test.js").unwrap();
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_multiple_imports_same_line() {
        let js_code = r#"
            import { useState, useEffect } from 'react';
        "#;
        let result = analyze_imports(js_code, "test.js").unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].module_request.name(), "react");
        if let ImportImportName::Name(import_name) = &result[0].import_name {
            assert_eq!(import_name.name(), "useState");
        };
    }
}
