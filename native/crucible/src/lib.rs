mod errors;
mod types;

// mod imports;
mod semantic_imports;
mod utils;
mod symbols;

use rustler::{Encoder, Env, NifResult, Term};


// #[rustler::nif]
// fn parse_js_from_file(path: String) -> Result<String> {
//     let source = std::fs::read_to_string(path)?;
//     parse_javascript(source)
// }

// #[rustler::nif]
// fn parse_js_from_buffer(buffer: String) -> Result<String> {
//     parse_javascript(buffer)
// }


fn on_load(_env: Env, _info: Term) -> bool {
    true
}

rustler::init!("Elixir.Crucible.Native", load = on_load);
