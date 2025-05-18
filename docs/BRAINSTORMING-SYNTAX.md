# FluxLang Syntax Brainstorming

This document lists a set of everyday programming tasks and considers how FluxLang's key technologies might shape their design. Each section will later be expanded with possible syntax ideas.


## 1. Logging to the Console
FluxLang will likely provide a standard library function to output text. Given the project's plan to leverage Rust ecosystems and LLVM for codegen, this might wrap Rust's `println!` or a runtime call. Reactive streams could allow log statements to be triggered by events.

TODO: Brainstorm syntax for printing values.

## 2. Declaring a Variable
Variables may carry refinement or temporal annotations. The language might infer types similar to Rust, but also allow specifying refinements checked by Z3.

TODO: Brainstorm syntax for variable declarations with optional type and refinement.

## 3. Assigning to a Variable
Assignments must respect linear usage of temporal streams if involved. Basic mutable variables might behave like Rust's `let mut`.

TODO: Brainstorm syntax for mutation and reassignment.

## 4. Conditional Execution
An `if` expression must be compatible with refinement types, ensuring branches satisfy constraints. Temporal logic might gate conditions based on event phases.

TODO: Brainstorm syntax for `if`/`else` constructs.

## 5. Looping Over a Range
Loops can interact with reactive streams or linear types. The design should ensure iteration variables respect borrow-like rules.

Possible syntaxes for numeric `for` loops might include:
* `for i in 0..n { ... }` a familiar Rust-style range with implicit type inference.
* `for i: Int [0 <= i < n] in 0..n { ... }` adding a refinement predicate on `i`.
* `for t in 0..duration @ cycle { ... }` associating each iteration with a temporal phase.
* `repeat n times |i| { ... }` using a macro-like form that leverages the plugin system.
* `for await i in timer(0..n) { ... }` iterating over a stream of time-based events.

## 6. Iterating Through a Collection
Similar to ranges but using collection APIs. Type inference should work with generics and refinement annotations on elements.

TODO: Brainstorm syntax for `for item in collection` style loops.

## 7. Defining a Function
Functions may include dependent or refinement type signatures and possibly temporal effects. The plan's plugin system could allow custom annotations here.

Possible syntaxes for function definitions might include:
* `fn add(x: Int, y: Int) -> Int { x + y }` a straightforward signature with explicit argument and return types.
* `fn add_pos(x: Int [x >= 0], y: Int [y >= 0]) -> Int [result >= 0] { ... }` showing refinement predicates verified by Z3.
* `async fn fetch(url: String) -> Stream<Response>` illustrating an asynchronous function returning a reactive stream.
* `fn compute<T: Numeric>(val: T) -> T where [val > 0]` combining generics with refinement constraints.
* `#[temporal(after = tick)] fn step(time: Time) -> Output @ (time + 1) { ... }` annotating temporal behavior via attributes usable by plugins.

## 8. Invoking a Function
Call syntax should be familiar yet support proof obligations for refinements. The compiler might generate solver queries based on argument types.

TODO: Brainstorm syntax for function calls and generics.

## 9. Pattern Matching
A match expression can destructure enums or stream events. Refinement types might refine matched variants.

Possible syntaxes for pattern matching might include:
* `match value { Some(x) => x, None => default }` mirroring Rust's familiar enum patterns.
* `match event when event.tag == "click" { Click(x, y) => ... }` integrating guards that can leverage refinement predicates.
* `on stream => match * { Data(d) => handle(d), Error(e) => log(e) }` combining reactive stream events with match arms.
* `match { variant: V1[a], other } { V1(n) if n > 0 => ... }` allowing inline refinement filters on pattern variables.
* `case value of { pattern => expr }` using a more ML-style keyword that could be macro-expanded by plugins.

## 10. Declaring a Struct
Structs can carry field refinements and may be used in macros. Their definitions will inform the AST structure defined via LALRPOP.

TODO: Brainstorm syntax for struct definitions.

## 11. Instantiating a Struct
Constructors may validate refinements at compile time. Temporal types might restrict when certain fields are valid.

TODO: Brainstorm syntax for creating struct values.

## 12. Accessing Struct Fields
Field access should be straightforward but also track temporal usage if fields are streams.

TODO: Brainstorm syntax for field reads and writes.

## 13. Declaring an Enum
Enums allow sum types. Refinements could constrain which variants appear in certain contexts.

TODO: Brainstorm syntax for enum definitions.

## 14. Matching on an Enum
Matching must exhaustively handle variants, and temporal rules might require handling of end-of-stream cases.

TODO: Brainstorm syntax for enum pattern matching.

## 15. Using Generics
Generics interact with the type system's refinements and linearity. Z3 may check generic constraints.

TODO: Brainstorm syntax for generic type parameters.

## 16. Creating a Reactive Stream
Core to FluxLang, streams will carry temporal and linear type information. Integration with petgraph IR will model stream graphs.

TODO: Brainstorm syntax for declaring and producing streams.

## 17. Subscribing to Stream Events
Consumers of streams must respect temporal sequencing. The compiler might enforce that subscribers handle events in order.

Possible syntaxes for listening to streams might include:
* `subscribe(my_stream, |value| { ... })` registering a closure to run for each event.
* `on my_stream as value { ... }` using an `on` keyword to bind the value and execute a block.
* `for await value in my_stream { ... }` drawing from async-style loops to process events sequentially.
* `when my_stream.emit(value) then { ... }` emphasizing temporal semantics when an event arrives.

## 18. Composing Streams
Operators like map, filter, and merge will use the reactive runtime and may rely on macros for concise expression.

TODO: Brainstorm syntax for stream combinators.

## 19. Using Refinement Types
Values can be annotated with logical predicates verified by Z3. Design should make predicates readable but concise.

TODO: Brainstorm syntax for refinement annotations on variables and functions.

## 20. Using Temporal Types
Temporal annotations specify when values or streams are valid. This intersects with linear usage tracking.

TODO: Brainstorm syntax for temporal qualifiers.

## 21. Importing Modules
The module system will integrate with the parser and CLI. Plugins might extend import behavior.

Possible syntaxes for module imports and exports might include:
* `import math::trig::{sin, cos}` using Rust-like paths with braces to select
  specific items.
* `from math.trig import sin as sine` providing a Python-style selective import
  with aliasing.
* `export fn calc()` or `pub fn calc()` to mark functions or values as
  accessible from other modules.
* `module utils;` declared at the top of a file to establish the module name and
  implicitly export its contents.
* `import plugin::json` loading a compile-time extension that can augment the
  import system via the plugin architecture.

## 22. Using Macros
Macros allow hygienic code generation. They must work well with the LALRPOP-based parser and avoid interfering with refinements.

TODO: Brainstorm syntax for defining and invoking macros.

## 23. Handling Errors
Result and Option types may exist with pattern matching. Refinements can express that a function never returns an error in certain cases.

Possible syntaxes for dealing with errors might include:
* A `Result<T, E>` type baked into the standard library with pattern matching on
  `Ok(value)` and `Err(e)` variants.
* A `try { ... }` block paired with the `?` operator to propagate failures
  directly from expressions.
* Stream-aware combinators like `on_error(stream, handler)` that route error
  values on a reactive channel to a recovery function.
* Function contracts such as `fn foo() -> !Error` to indicate, via refinement,
  that a call cannot fail under proven preconditions.

## 24. Resource Management
Linear types help ensure resources like streams are consumed correctly. The design might borrow from Rust's ownership model.

TODO: Brainstorm syntax for defining and dropping resources.

## 25. Concurrency and Tasks
If FluxLang supports async tasks or threads, the interaction with reactive streams and temporal logic becomes important.

TODO: Brainstorm syntax for spawning and managing concurrent tasks.


