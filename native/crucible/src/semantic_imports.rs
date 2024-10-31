use oxc_allocator::Allocator;
// use oxc_parser::Parser;
use oxc_semantic::{SemanticBuilder};
use oxc_span::{Span, SourceType};
use std::path::Path;
use oxc_syntax::module_record::ImportEntry;
use crate::parse_source;
 

fn analyze_imports(
    source_code: &str,
    filename: &str,
) -> Result<Vec<ImportEntry>, Box<dyn std::error::Error>> {
    let allocator = Allocator::default();
    let ret = parse_source(&allocator, source_code).unwrap(); // Use parse_source
    let path = Path::new(filename);

    let semantic_builder_ret = SemanticBuilder::new()
        .build_module_record(path, &ret.program)
        .with_check_syntax_error(true)
        .build(&ret.program);

    let semantic = semantic_builder_ret.semantic;
    Ok(semantic.module_record().import_entries.clone())
 
}


#[cfg(test)]
mod tests {
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
        assert!(result.iter().any(|info| info.module_request.name() == "phoenix.js"));
        assert!(result.iter().any(|info| info.module_request.name() == "react"));
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
            assert_eq!(import_name.name(),  "useState");
        };
    }
}