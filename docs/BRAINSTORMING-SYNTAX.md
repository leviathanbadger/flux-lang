# FluxLang Syntax Brainstorming

This document lists a set of everyday programming tasks and considers how FluxLang's key technologies might shape their design. Each section will later be expanded with possible syntax ideas.


## 1. Logging to the Console
FluxLang will likely provide a standard library function to output text. Given the project's plan to leverage Rust ecosystems and LLVM for codegen, this might wrap Rust's `println!` or a runtime call. Reactive streams could allow log statements to be triggered by events.

Possible syntaxes for printing values might include:
* `print("hello")` a straightforward standard library call.
* `log "value = {x}"` a statement form that expands to a runtime print macro.
* `debug!(x)` using a macro-style invocation familiar to Rust developers.
* `emit log(x)` sending the value on a logging stream that listeners can observe.


## 2. Declaring a Variable
Variables may carry refinement or temporal annotations. The language might infer types similar to Rust, but also allow specifying refinements checked by Z3.

Possible syntaxes for variable declarations might include:
* `let count = 0` relying on inference to determine the type.
* `let id: Int = 42` specifying the primitive type explicitly.
* `let mut limit: Int [limit > 0] = 10` combining mutability with a refinement predicate.
* `let events@time: Stream<Event>` introducing a temporal qualifier on the variable.


## 3. Assigning to a Variable
Assignments must respect linear usage of temporal streams if involved. Basic mutable variables might behave like Rust's `let mut`.

Possible syntaxes for mutation and reassignment might include:
* `count = count + 1` the basic assignment operator.
* `count += 1` sugar for arithmetic updates.
* `next_value -> result` piping a computed value into a variable respecting linear usage.
* `state@next := update(state)` denoting a temporal update that takes effect at the next tick.


## 4. Conditional Execution
An `if` expression must be compatible with refinement types, ensuring branches satisfy constraints. Temporal logic might gate conditions based on event phases.

Possible syntaxes for `if`/`else` constructs might include:
* `if cond { ... } else { ... }` the classic block form.
* `when cond do { ... } else { ... }` emphasizing temporal checks.
* `cond { cond1 => { ... }; _ => { ... } }` inspired by Racket style conditionals.
* `if cond -> expr1; otherwise -> expr2` using arrow clauses to express branches concisely.
* `? cond { expr1 } : { expr2 }` a terse conditional expression reminiscent of C's ternary operator.

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

Possible syntaxes for iterating through a collection might include:
* `for item in collection { ... }` the straightforward `for` loop.
* `foreach item -> collection { ... }` echoing older BASIC style.
* `collection.each |item| { ... }` a method-call style inspired by Ruby.
* `for item <- collection do ...` using a comprehension form similar to Haskell.
* `loop item of collection { ... }` a free-form keyword that could be enabled by plugins.

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

Possible syntaxes for function calls and generics might include:
* `add(1, 2)` a direct call using inferred argument types.
* `parse::<Int>("42")` showing an explicit generic parameter similar to Rust.
* `await fetch(url)` invoking an async function that yields a stream of results.
* `plugin::transform!(input)` leveraging the macro system for compile-time code generation.


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

Possible syntaxes for struct definitions might include:
* `struct Point { x: Int, y: Int }` a conventional record style.
* `record Point(x: Int, y: Int)` inspired by Kotlin or C# record declarations.
* `type Point = { x: Int; y: Int }` borrowing from ML-family languages.
* `data Point { x :: Int; y :: Int }` using a Haskell-like syntax.
* `@refined struct Account { balance: Int [balance >= 0] }` demonstrating refinement annotations via attributes.

## 11. Instantiating a Struct
Constructors may validate refinements at compile time. Temporal types might restrict when certain fields are valid.

Possible syntaxes for creating struct values might include:
* `let pt = Point { x: 0, y: 1 }` the classic literal form.
* `let pt = Point(x = 0, y = 1)` using named parameters.
* `new Point { x=0; y=1 }` explicit construction keyword inspired by C#.
* `build Point with x <- 0, y <- 1` a builder-like syntax.
* `pt := Point::create(0, 1)` calling an associated constructor.

## 12. Accessing Struct Fields
Field access should be straightforward but also track temporal usage if fields are streams.

Possible syntaxes for field reads and writes might include:
* `pt.x` standard dot notation for access.
* `pt["x"]` using string-based indexing for dynamic field names.
* `pt@next.x = 5` performing a temporal update that takes effect later.
* `with pt { .x = 2 }` block-based mutation reminiscent of record-update syntax.
* `pt->x` using an arrow operator similar to C for pointer-like semantics.

## 13. Declaring an Enum
Enums allow sum types. Refinements could constrain which variants appear in certain contexts.

Possible syntaxes for enum definitions might include:
* `enum Color { Red, Green, Blue }` the classic closed set of variants.
* `enum Option<T> { Some(T), None }` demonstrating generics within an enum.
* `enum Phase @time { Start, Middle, End }` attaching temporal qualifiers to each variant.
* `enum Result<T, E> = Ok(T) | Err(E)` using a bar-separated shorthand form.


## 14. Matching on an Enum
Matching must exhaustively handle variants, and temporal rules might require handling of end-of-stream cases.

Possible syntaxes for enum pattern matching might include:
* `match value { Ok(v) => v, Err(e) => handle(e) }` a direct case split.
* `case value of Ok(v) -> v | Err(e) -> handle(e)` using a succinct arrow form.
* `when Ok(v) := value { ... } else { ... }` pattern binding within a condition.
* `if value is Ok then ... else ...` an `is` keyword reminiscent of Python.
* `select value { Ok(v): ..., Err(e): ... }` an alternative keyword driven by plugins.

## 15. Using Generics
Generics interact with the type system's refinements and linearity. Z3 may check generic constraints.

Possible syntaxes for generic type parameters might include:
* `fn identity<T>(val: T) -> T` the traditional angle-bracket form.
* `fn<T> identity(val: T) -> T` placing generics before the function name as in C++.
* `fun identity['a](x: 'a): 'a` an ML-style tick notation.
* `generic[T: Numeric] fn add(x: T, y: T) -> T` using a leading keyword with trait bounds.
* `def wrap(val T) -> Box<T>` where the type parameter is implied by a free identifier.

## 16. Creating a Reactive Stream
Core to FluxLang, streams will carry temporal and linear type information. Integration with petgraph IR will model stream graphs.

Possible syntaxes for declaring and producing streams might include:
* `stream numbers = 0..n` constructing a stream from a range literal.
* `let s = stream { yield 1; yield 2 }` a generator block that produces events.
* `source events := event_source()` wiring up a named input source.
* `flow x from data { emit x }` a DSL-style declaration for custom flows.
* `channel<T>()` explicitly creating a typed channel that can be pushed to.

## 17. Subscribing to Stream Events
Consumers of streams must respect temporal sequencing. The compiler might enforce that subscribers handle events in order.

Possible syntaxes for listening to streams might include:
* `subscribe(my_stream, |value| { ... })` registering a closure to run for each event.
* `on my_stream as value { ... }` using an `on` keyword to bind the value and execute a block.
* `for await value in my_stream { ... }` drawing from async-style loops to process events sequentially.
* `when my_stream.emit(value) then { ... }` emphasizing temporal semantics when an event arrives.

## 18. Composing Streams
Operators like map, filter, and merge will use the reactive runtime and may rely on macros for concise expression.

Possible syntaxes for stream combinators might include:
* `mapped = map(f, stream)` a direct function call style.
* `stream |> map(f) |> filter(g)` chaining operations via a pipeline operator.
* `stream >> map f >> merge other` using arrow combinators inspired by F#.
* `stream{ .map(f).filter(g) }` method-chaining reminiscent of JavaScript.
* `stream1 combine stream2 |> fold(start, f)` creating custom infix operators via plugins.

## 19. Using Refinement Types
Values can be annotated with logical predicates verified by Z3. Design should make predicates readable but concise.

Possible syntaxes for refinement annotations on variables and functions might include:
* `let x: Int where x > 0` an inline `where` clause.
* `type PosInt = Int { self > 0 }` defining a new refined type.
* `fn add(a: Int, b: Int) -> Int { result >= a }` using a post-condition block.
* `let n: Int satisfying n % 2 == 0` natural language style constraints.
* `val age: Int requiring age >= 18` a keyword-based predicate for clarity.

## 20. Using Temporal Types
Temporal annotations specify when values or streams are valid. This intersects with linear usage tracking.

Possible syntaxes for temporal qualifiers might include:
* `value@time` attaching an explicit timestamp.
* `state[next]` indexing the value in the next tick.
* `Stream<Event> @ phase` annotating the phase at which events occur.
* `future value` designating a value available only in the future.
* `value[t + 1]` expressing relative temporal offsets.

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

Possible syntaxes for defining and invoking macros might include:
* `macro_rules! greet { ( $name:expr ) => { print("hi", $name) } }` a Rust-inspired definition.
* `macro greet(name) => { print("hi {name}") }` using an arrow to expand into code.
* `define macro inc(x) { x + 1 }` with a keyword-driven style.
* `@macro json { ... }` attribute-style macros that transform the annotated item.
* `macro <<pipeline>> { ... }` employing custom delimiters for DSL-like macros.

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

Possible syntaxes for defining and dropping resources might include:
* `with file = open(path) { ... }` automatically closes when the block ends.
* `let file = open(path); defer file.close()` mirroring Go's `defer` statement.
* `use file = File.open(path)` RAII style acquisition.
* `resource File(path) -> file { ... }` an explicit resource block.
* `drop file` or `dispose(file)` to release resources manually.

## 25. Concurrency and Tasks
If FluxLang supports async tasks or threads, the interaction with reactive streams and temporal logic becomes important.

Possible syntaxes for spawning and managing concurrent tasks might include:
* `spawn task { ... }` a lightweight syntax for launching asynchronous work.
* `go { ... }` a minimal keyword inspired by Go.
* `let handle = async_run(f())` creating a task handle to await later.
* `parallel for item in items { ... }` parallelized loop semantics.
* `await join(handle)` waiting for a task to complete.


