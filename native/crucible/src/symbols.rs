// get SymbolTable
// find declarations


use oxc_allocator::Allocator;
use crate::{errors::CrucibleError, utils::parse_source};
use oxc_semantic::{ScopeTree, SemanticBuilder, SymbolTable};
use oxc_span::{SourceType, Span};
use oxc_syntax::module_record::ImportEntry;
use rustler::ResourceArc;
use std::path::Path;

////////////////////////////
// structs
////////////////////////////

#[derive(rustler::NifStruct)]
#[module = "Crucible.Symbols"]
pub struct Symbols {
    pub resource: ResourceArc<SemanticSymbolsRef>,
}
pub struct SemanticSymbolsRef(pub SymbolAndScope);

#[rustler::resource_impl]
impl rustler::Resource for SemanticSymbolsRef {}

#[derive(Debug)]
pub struct SymbolAndScope(SymbolTable, ScopeTree);

impl From<SymbolAndScope> for Symbols {
    fn from(data: SymbolAndScope) -> Self {
        Self {
            resource: rustler::ResourceArc::new(SemanticSymbolsRef(data)),
        }
    }
}

/////////////////////
/// creators
////////////////////

#[rustler::nif]
pub fn ast_for_symbols_from_buffer(
    source_code: &str,
    filename: &str,
) -> Result<Symbols, rustler::Error> {
    let allocator = Allocator::default();
    let ret = parse_source(&allocator, source_code)?;
    let path = Path::new(filename);

    let semantic_builder_ret = SemanticBuilder::new()
        .build_module_record(path, &ret.program)
        .with_check_syntax_error(true)
        .build(&ret.program);

    let semantic = semantic_builder_ret.semantic;
    let (symbol_table, scope_tree) = (semantic.into_symbol_table_and_scope_tree());
    println!("{:#?}", symbol_table.names);
    println!("{:#?}", scope_tree);
    Ok(SymbolAndScope(symbol_table, scope_tree).into())
}


#[rustler::nif]
pub fn symbol_names(ast_for_symbols: Symbols) -> Vec<String> {
    let symbol_struct = ast_for_symbols.resource.0.0.names.clone();

    symbol_struct.iter().map(|s| s.as_str().to_string()).collect()

}




#[rustler::nif]
pub fn has_hooks(ast_for_symbols: Symbols) -> bool {
    let symbol_table = ast_for_symbols.resource.0.0;
    // let scope_tree = ast_for_symbols.resource.0.1;
    // Assuming LiveSocket is a globally accessible variable within the scope.
    // Adjust the lookup if it's within a specific function or block.
    // let livesocket_declarations = symbol_table.symbol_ids().map(|symbol_id| {
    //     // let result = scope_tree.find_binding(scope_id, "hooks");
    //     let result = symbol_table.get_name(symbol_id);
    //     dbg!(&result);
    //     result == "hooks"
    //  })
    //  .collect::<Vec<bool>>();

    let has_hooks = symbol_table.symbol_ids().fold(false, |acc, symbol_id| {
        let result = symbol_table.get_name(symbol_id);
        dbg!(&result);
        acc || result == "hooks"
    });

    
    has_hooks
    // let live_socket_symbol = scope_tree.find_binding("LiveSocket");

    // if let Some(live_socket_symbol) = live_socket_symbol {
    //     // Check if the LiveSocket symbol has a property named "hooks"
    //     live_socket_symbol.has_property("hooks")
    // } else {
    //     false // LiveSocket symbol not found
    // }
}


// ///////////////////
// / getters
// //////////////////


// ///////////////////
// / setters
// //////////////////
