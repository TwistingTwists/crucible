### Plan / Roadmap

Given a js file path
    - read the js file
    - parse the contents -> return the pointer to AST to Beam
    - write bindings to the manipulation functions so that one can do ____ in existing js file
      - import a new file
      - check if a dependency is already imported
      - write a new key/value pair to existing hooks



### How does this work?


from the oxc GitHub repo, pick the crates oxc_parser , oxc_ast and oxc_semantic.

Parser (oxc_parser)
Serves as the primary parser for JavaScript and TypeScript code4
Supports modern ECMAScript syntax, TypeScript, JSX/TSX, and Stage 3 Decorators4
Uses a minimal API that takes three inputs:
Memory arena
Source string
SourceType (JS/TS configuration)
Abstract Syntax Tree (oxc_ast)
Defines the Abstract Syntax Tree nodes for both JavaScript and TypeScript3
Differs from traditional ESTree in several ways:
Uses explicit identifiers like BindingIdentifier, IdentifierReference, and IdentifierName
Implements specialized assignment expressions and patterns3
Performance Features
The parser is notably fast, performing 3x faster than the SWC parser2
Optimizations include:
AST allocation in a memory arena (bumpalo) for fast AST operations
Use of u32 instead of usize for span offsets4
Deferred scope binding and symbol resolution to the semantic analyzer4
Workflow Integration
Parsing Process
```rust
let parser_return = Parser::new(&allocator, &source_text, source_type).parse();
```
Key Features
Handles .js(x) and .ts(x) files2
Passes comprehensive test suites:
All Test262 parser tests
99% of Babel and TypeScript tests2
Provides utility for ESM import/export data extraction2
Usage Examples
Basic JavaScript Parsing
```rust
use oxc_allocator::Allocator;
use oxc_parser::Parser;
use oxc_span::SourceType;

let allocator = Allocator::default();
let source_type = SourceType::from_path("example.js").unwrap();
let parser_return = Parser::new(&allocator, &source_text, source_type).parse();
```
The parser's output includes:
The AST (Abstract Syntax Tree)
Any syntax errors encountered
Comments from the source code4
This architecture allows for efficient parsing and analysis of JavaScript code while maintaining high performance and accuracy.

----
based on above, write a program that uses oxc_semantic and lists out all the imports for the given js file.