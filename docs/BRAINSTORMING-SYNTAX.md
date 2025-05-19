# FluxLang Syntax Brainstorming

This document lists common programming tasks and records the emerging syntax decisions for FluxLang.  Each section describes the preferred syntax along with short reasoning on why competing ideas were removed.
Statements are terminated with semicolons. Blocks do not require trailing semicolons.


## 1. Logging to the Console
FluxLang will provide a small standard library.  Logging will mirror Rust so that developers feel at home.

**Syntax**
```flux
print("hello");
log!("value = {}", x);
```

**Reasoning**: a function call and a macro cover simple printing and formatted debugging without new keywords like `emit`.

## 2. Declaring a Variable
Variables can carry refinements and temporal qualifiers.  We keep the familiar `let` keyword and use a `where` clause for refinements.

**Syntax**
```flux
let count = 0;
let id: Int = 42;
let mut limit: Int where limit > 0 = 10;
let events@time: Stream<Event>;
```

**Reasoning**: aligns with Rust while introducing refinements in a readable `where` clause.

## 3. Assigning to a Variable
Assignments behave like Rust with optional temporal indexing for future states.

**Syntax**
```flux
count = count + 1;
count += 1;
state[next] = update(state);
```

**Reasoning**: arrow based assignments conflicted with other uses of `->`; indexing with `[next]` keeps temporal updates explicit.

## 4. Conditional Execution
The traditional `if/else` form remains primary.  Temporal checks can be expressed with a `when` annotation.

**Syntax**
```flux
if cond { ... } else { ... }
when phase if cond { ... } else { ... }
```

**Reasoning**: removes less common forms (`cond { ... };`, ternaries) to keep the language concise.

## 5. Looping Over a Range
Numeric loops follow Rust's style and may attach refinements or temporal phases.

**Syntax**
```flux
for i in 0..n { ... }
for i in 0..n where 0 <= i { ... }
for await i in timer(0..n) { ... }
```

**Reasoning**: standard `for` loops are clear and work well with refinements; plugin based `repeat` forms were dropped.

## 6. Iterating Through a Collection
Collection iteration mirrors range loops and supports method chaining if desired.

**Syntax**
```flux
for item in collection { ... };
collection.each(|item| { ... });
```

**Reasoning**: removes `foreach` and `loop item of` alternatives to avoid multiple keywords for the same concept.

## 7. Defining a Function
Function definitions are Rust-like with optional refinement clauses and attributes for temporal behaviour.

**Syntax**
```flux
fn add(x: Int, y: Int) -> Int { x + y }
fn add_pos(x: Int where x >= 0, y: Int where y >= 0) -> Int where result >= 0 { ... }
async fn fetch(url: String) -> Stream<Response>
fn compute<T: Numeric>(val: T) -> T where val > 0
#[temporal(after = tick)] fn step(time: Time) -> Output @ (time + 1) { ... }
```
**Reasoning**: keeps one clear form for generics and refinements and relies on attributes for temporal semantics.

## 8. Invoking a Function
Calls look familiar and support explicit generics and async/await.

**Syntax**
```flux
add(1, 2);
parse::<Int>("42");
await fetch(url);
plugin::transform!(input);
```
**Reasoning**: retains Rust call syntax and macro invocation; other alternatives added little value.

## 9. Pattern Matching
Pattern matching follows Rust's `match` with optional guards.

**Syntax**
```flux
match value { Some(x) => x, None => default };
match event if event.tag == "click" { Click(x, y) => ... };
```
**Reasoning**: stream based `on` forms were removed to keep one clear construct; guards express refinement checks directly.

## 10. Declaring a Struct
Structs use a record syntax.  Refinements may appear on fields via `where`.

**Syntax**
```flux
struct Point { x: Int, y: Int }
struct Account { balance: Int where balance >= 0 }
```
**Reasoning**: record and data style alternatives were dropped to standardise on a single form.

## 11. Instantiating a Struct
Instances use brace literals or associated constructors.

**Syntax**
```flux
let pt = Point { x: 0, y: 1 };
let pt2 = Point::new(0, 1);
```
**Reasoning**: avoids special `new` or builder keywords in favour of conventional forms.

## 12. Accessing Struct Fields
Field access uses dot notation with optional temporal indexing.

**Syntax**
```flux
pt.x;
pt[next].x = 5;
```
**Reasoning**: arrow operators and update blocks were removed for clarity; temporal writes reuse the indexing syntax from assignments.

## 13. Declaring an Enum
Enums are declared with variant lists and can carry generics.

**Syntax**
```flux
enum Color { Red, Green, Blue };
enum Option<T> { Some(T), None };
enum Result<T, E> { Ok(T), Err(E) };
```
**Reasoning**: bar separated and temporal enum forms were removed to keep the declaration style simple.

## 14. Matching on an Enum
Matching values is done with the same `match` keyword used earlier.

**Syntax**
```flux
match value { Ok(v) => v, Err(e) => handle(e) };
```
**Reasoning**: alternatives using `is` or `select` introduced redundant keywords without clear benefit.

## 15. Using Generics
Generics use angle brackets after the function name.  Bounds appear with `:` and refinements use `where` clauses.

**Syntax**
```flux
fn identity<T>(val: T) -> T;
fn add<T: Numeric>(x: T, y: T) -> T;
```
**Reasoning**: choosing one placement for generics keeps parsing straightforward and matches common Rust practice.

## 16. Creating a Reactive Stream
Streams can be built from ranges, generators or channels.

**Syntax**
```flux
stream numbers = 0..n;
let s = stream { yield 1; yield 2 };
channel<T>();
```
**Reasoning**: the custom `flow` form conflicted with the desire to keep streams as first class values.

## 17. Subscribing to Stream Events
Consumption of streams uses an async-style `for await` loop or a `subscribe` helper.

**Syntax**
```flux
for await value in my_stream { ... };
subscribe(my_stream, |value| { ... });
```
**Reasoning**: a single keyword based approach is clearer than multiple custom constructs like `on` or `when emit`.

## 18. Composing Streams
A pipeline operator expresses common combinators.

**Syntax**
```flux
stream |> map(f) |> filter(g);
```
**Reasoning**: chaining with `|>` reads left to right and avoids introducing numerous infix variants.

## 19. Using Refinement Types
Refinement predicates appear in `where` clauses or new type definitions.

**Syntax**
```flux
let x: Int where x > 0 = 1;
type PosInt = Int where self > 0;
```
**Reasoning**: natural language styles were dropped to keep the syntax concise and solver friendly.

## 20. Using Temporal Types
Temporal qualifiers reuse the indexing notation and `@` markers.

**Syntax**
```flux
value@time;
state[next];
Stream<Event>@phase;
```
**Reasoning**: a uniform notation reduces confusion compared to keywords like `future`.

## 21. Importing Modules
Imports follow Rust-style paths and `export` marks public items.

**Syntax**
```flux
import math::trig::{sin, cos};
export fn calc();
```
**Reasoning**: avoids Python-like `from` and keeps module syntax consistent with the rest of the language.

## 22. Using Macros
Macros use a `macro` keyword with arrow expansion and are invoked with `!` when they behave like functions.

**Syntax**
```flux
macro greet(name) => { print("hi {name}") };
greet!("world");
```
**Reasoning**: standardising on one definition form simplifies macro tooling.

## 23. Handling Errors
FluxLang provides `Result` and a `try` block with the `?` operator for propagation.

**Syntax**
```flux
try { risky()? };
```
**Reasoning**: stream-specific error combinators can be built as library functions; keeping the core syntax minimal helps reasoning about control flow.

## 24. Resource Management
Scoped resource blocks and a `defer` statement manage cleanup.

**Syntax**
```flux
with file = open(path) { ... };
let file = open(path); defer file.close();
```
**Reasoning**: these two forms cover common RAII patterns without extra keywords like `resource` or `dispose`.

## 25. Concurrency and Tasks
Asynchronous tasks resemble Rust's async model with lightweight spawning.

**Syntax**
```flux
spawn { ... }
let handle = async_run(f());
await join(handle);
```
**Reasoning**: dropping the `go` keyword and parallel for loops keeps concurrency primitives orthogonal to the rest of the language.

## 26. Unit Testing and Assertions
Testing helps validate both semantics and refinement proofs.

Selected syntax ideas:
* `test "adds numbers" { assert(add(2,2) == 4) };` – lightweight inline test block.
* `#[test] fn adds() { assert(add(2,2) == 4) };` – Rust‑style attribute test.
* `spec add_positive(x: Int, y: Int) [x > 0, y > 0] => { add(x, y) > 0 };` – property specification leveraging refinements.

**Reasoning**: providing both block and attribute styles allows quick checks and
integration with tooling. A specification form illustrates how refinement logic
can drive property tests.

## 27. Calling Foreign Functions
Interoperability with existing libraries is essential for adoption.

Selected syntax ideas:
* `extern fn c_func(arg: Int) -> Int`; – direct declaration of an external function.
* `@ffi("libm") fn sin(x: Float) -> Float`; – attribute specifying the foreign library.
* `link "c" { fn printf(format: *const u8, ...) };` – grouped declarations inside a link block.

**Reasoning**: the FFI syntax mirrors Rust's approach but adds attributes for
explicit library names. Grouped declarations make large foreign interfaces more
manageable.

## 28. Defining a Trait
FluxLang traits describe shared behaviour similar to Rust's traits.

**Syntax**
```flux
trait Printable { fn print(self) };
```
**Reasoning**: adding traits fills a gap in the original list and provides a foundation for generics and interfaces.

## 29. Implementing a Trait
Implementations associate trait methods with concrete types.

**Syntax**
```flux
impl Printable for Point { fn print(self) { print("({},{})", self.x, self.y) } };
```
**Reasoning**: mirrors Rust's `impl` syntax so tooling and developers can easily adapt.


## 30. Lambda Expressions
Closures enable concise inline functions.

**Syntax**
```flux
let inc = |x| x + 1;
collection.map(|x| x * 2);
```
**Reasoning**: Lambdas are commonplace in modern languages and work well with stream combinators.

## 31. Documentation Comments
Rust-style documentation comments integrate with tooling.

**Syntax**
```flux
/// Adds two numbers.
fn add(x: Int, y: Int) -> Int { x + y };
```
**Reasoning**: Keeping docs next to code encourages comprehensive documentation without new keywords.

## 32. Compile-Time Constants
Constant values are declared with `const`.

**Syntax**
```flux
const MAX: Int = 10;
fn array(len: Int = MAX) -> [Int; len]
```
**Reasoning**: A clear `const` form avoids using macros for simple constants and enables compile-time evaluation.

## 33. Using Attributes
Attributes annotate declarations with metadata.

**Syntax**
```flux
#[inline]
fn compute() { ... };

#[temporal(after = tick)]
fn step(time: Time) -> Output @ (time + 1) { ... };
```
**Reasoning**: Attributes are a flexible mechanism to express hints and temporal semantics consistently across the language.

## 34. Draft Formal Syntax

The following grammar sketches the core FluxLang syntax in EBNF form. It
summarises the decisions above and aims to remove obvious ambiguities.

```ebnf
program      = { item } ;
item         = function_def
             | struct_def
             | enum_def
             | trait_def
             | impl_def
             | const_def
             | import ;

function_def = "fn" identifier "(" [ param { "," param } ] ")"
               [ "->" type ] block ;
param        = identifier ":" type ;

struct_def   = "struct" identifier "{" field { "," field } "}" ;
field        = identifier ":" type [ where_clause ] ;

enum_def     = "enum" identifier "{" variant { "," variant } "}" ;
variant      = identifier [ "(" type { "," type } ")" ] ;

trait_def    = "trait" identifier "{" { function_sig } "}" ;
function_sig = "fn" identifier "(" [ param { "," param } ] ")"
               [ "->" type ] ";" ;

impl_def     = "impl" identifier "for" type block ;
const_def    = "const" identifier ":" type "=" expression ";" ;
import       = "import" path ";" ;

block        = "{" { statement } "}" ;
statement    = variable_decl
             | assignment
             | expression ";"
             | if_stmt
             | for_loop
             | while_loop
             | match_stmt
             | return_stmt ;

variable_decl= "let" [ "mut" ] identifier
               [ ":" type ] [ where_clause ]
               [ "=" expression ] ";" ;
assignment   = place "=" expression ";" ;
place        = identifier
             | expression "." identifier
             | expression "[" expression "]" ;
if_stmt      = "if" expression block [ "else" block ] ;
for_loop     = "for" identifier "in" expression block ;
while_loop   = "while" expression block ;
match_stmt   = "match" expression "{" { match_arm } "}" ;
match_arm    = pattern "=>" expression ";" ;
return_stmt  = "return" [ expression ] ";" ;

expression   = literal
             | identifier
             | call
             | lambda
             | block ;
call         = identifier "(" [ expression { "," expression } ] ")" ;
lambda       = "|" [ param { "," param } ] "|" expression ;

type         = path [ "<" type { "," type } ">" ]
             | type "@" identifier ;
path         = identifier { "::" identifier } ;
pattern      = identifier | "_" ;
where_clause = "where" expression ;
literal      = integer | string ;
identifier   = /[a-zA-Z_][a-zA-Z0-9_]*/ ;
integer      = /[0-9]+/ ;
string       = '"' { character } '"' ;
```

This grammar is intentionally small and omits many details (such as operator
precedence) but provides a starting point for a consistent parser.
