write idiomatic rust code which does the following 

1. uses NewType pattern in rust to encapsulate Program (here, ParseResult does that) 
2. initalises allocator and source
3. uses parse_js_from_file or parse_js_from_buffer to parse and return pointers.




Searching for Hooks:

1. Hooks is passed as parameters to new LiveSocket?
2. if yes, then search for the object in current file.  let liveSocket = new LiveSocket("/live", Socket, {uploaders: Uploaders,params: {_csrf_token: csrfToken}, hooks: Hooks})
3. if object not found in current file, then search for import with same name. example:  let liveSocket = new LiveSocket("/live", Socket, {uploaders: Uploaders,params: {_csrf_token: csrfToken}, hooks: hooks}) // where hooks is imported like 'import hooks from ./hooks'
4. if no (1.), then return CreateHook



This is the mermaid diagram.



use the docs for SymbolTable and ScopeTree 


Here is what you need to do:
Given this js code, 
` let liveSocket = new LiveSocket("/live", Socket, {uploaders: Uploaders,params: {_csrf_token: csrfToken}, hooks: Hooks})`


write a method which can tell whether or not hooks is defined in `LiveSocket`

So, for 
` let liveSocket = new LiveSocket("/live", Socket, {uploaders: Uploaders,params: {_csrf_token: csrfToken}})`
the function shoiuld return false.

