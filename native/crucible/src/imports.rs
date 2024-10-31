use oxc_allocator::Allocator;
use oxc_ast::{ast::*, Visit};
use oxc_parser::Parser;
use oxc_span::SourceType;
use derivative::Derivative;

struct ImportAnalyzer {
    imports: Vec<ImportInfo>,
}

#[derive(Derivative)]
#[derivative(PartialEq, Debug)]
struct ImportInfo {
    /// The source of the import.
    source: String,
    /// The kind of import.
    kind: ImportKind,
    
    /// The span of the import.
    #[derivative(PartialEq="ignore")]
    span: Span,
}

#[derive(Debug, PartialEq)]
enum ImportKind {
    Named(Vec<String>),
    Default(String),
    Namespace(String),
    Dynamic,
}

// Implement the visitor
impl<'a> Visit<'a> for ImportAnalyzer {
    fn visit_import_declaration(&mut self, import_decl: &ImportDeclaration<'a>) {
        let source = import_decl.source.value.to_string().clone();

        // Handle different import types
        if let Some(specifiers) = &import_decl.specifiers {
            for specifier in specifiers {
                match specifier {
                    ImportDeclarationSpecifier::ImportSpecifier(specifier) => {
                        // let local = specifier.local.clone();
                        let imported = specifier.imported.clone();
                        // let import_kind = specifier.import_kind;
                        let span = specifier.span;
                        self.imports.push(ImportInfo {
                            source: source.clone(),
                            kind: ImportKind::Named(match imported {
                                ModuleExportName::IdentifierName(identifier) => {
                                    vec![identifier.name.to_string()]
                                }
                                ModuleExportName::IdentifierReference(identifier) => {
                                    vec![identifier.name.to_string()]
                                }
                                ModuleExportName::StringLiteral(string_literal) => {
                                    vec![string_literal.value.to_string()]
                                }
                            }),
                            span,
                        });
                    }
                    ImportDeclarationSpecifier::ImportDefaultSpecifier(default_specifier) => {
                        let local = default_specifier.local.clone();
                        let span = default_specifier.span;
                        self.imports.push(ImportInfo {
                            source: source.clone(),
                            kind: ImportKind::Default(local.name.to_string()),
                            span,
                        });
                    }
                    ImportDeclarationSpecifier::ImportNamespaceSpecifier(namespace_specifier) => {
                        let local = namespace_specifier.local.clone();
                        let span = namespace_specifier.span;
                        self.imports.push(ImportInfo {
                            source: source.clone(),
                            kind: ImportKind::Namespace(local.name.to_string()),
                            span,
                        });
                    }
                }
            }
        }
    }
}

fn analyze_file_imports(source_code: &str) -> Result<Vec<ImportInfo>, String> {
    let allocator = Allocator::default();
    let source_type = SourceType::default();

    let parse_result = Parser::new(&allocator, source_code, source_type).parse();

    if !parse_result.errors.is_empty() {
        return Err("Parsing errors encountered".to_string());
    }

    let mut analyzer = ImportAnalyzer {
        imports: Vec::new(),
    };

    // Visit the AST
    analyzer.visit_program(&parse_result.program);

    Ok(analyzer.imports)
}

mod tests {
    use super::*;
    // use oxc_span::Span;
    use pretty_assertions::{assert_eq, assert_ne};


    #[test]
    fn test_analyze_file_imports() {
        let source = r#"
        import defaultMember from "module-name1";
        import { member1, member2 } from "module-name2";
        import * as name from "module-name3";
        import "module-name4";
        
        const dynamic = await import("dynamic-module5");
    "#;

        let expected_imports =  vec![
            ImportInfo {
                source: "module-name1".to_string(),
                kind: ImportKind::Default("defaultMember".to_string()),
                span: Default::default(),
            },
            ImportInfo {
                source: "module-name2".to_string(),
                kind: ImportKind::Named(["member1".to_string()].to_vec()),
                span: Default::default(),
            },
            ImportInfo {
                source: "module-name2".to_string(),
                kind: ImportKind::Named(["member2".to_string()].to_vec()),
                span: Default::default(),
            },
            ImportInfo {
                source: "module-name3".to_string(),
                span: Default::default(),
                kind: ImportKind::Namespace("name".to_string()),
            },
        ];


        let imports = analyze_file_imports(source).unwrap();

        assert_eq!(imports, expected_imports);
    }
}
