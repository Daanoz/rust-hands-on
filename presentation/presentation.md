title: Rust introduction
class: animation-fade
layout: true

<!-- This slide will serve as the base layout for all your slides -->
.bottom-bar[
  {{title}}
]

---

class: impact

# {{title}}
## Daan Sieben, Aad Rijnberg

---

# The Rust language

.col-6[
- Language properties
- Memory safety
- Types
- Variables
- Scopes
- Functions
- Ownership
- Cargo
- Tuples
- Structures
]
.col-6[
- Control flow
- Enumerations
- Generics
- Traits
- Arrays, Vectors, HashSets, HashMaps
- Closures
- Iterators
- Code generation
- References and lifetime
- Using modules / public / private
]

---

# Language properties

- Statically-typed language
--

- Memory safety
--

- Immutable by default
--

- Private visibility by default
--

- Stack allocation by default, heap allocation must be chosen explicitly (e.g. Box`<T>`)
--

- Not an OO language (e.g. no inheritance)!
--

- Type inference

--
- No exceptions, using Result enforces handling of errors

--
- One tool (cargo) for dependency management, building, running, formatting, ...

---

# Memory safety

- No garbage collection

--

- No manual allocation/freeing of memory

--

- Ownership (either transfer ownership or borrow)
    - Each value has an owner
    - There can only be one owner at a time
    - When the owner goes out of scope, the value is dropped

--

  Borrow checker: the part of the compiler responsible for ensuring that **references do not outlive the data they refer to**, and it helps eliminate entire classes of bugs caused by memory unsafety (e.g. **null pointer dereferences, data races, double frees, use-after-free**).

---
class: big
## Rust types

- **Primitive types**:<br/>
  bool, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, usize, isize, f32, f64, str, char, array

--
- **Stdlib types**:<br/>
  Box&lt;T>, String, Vec&lt;T>, Option&lt;T>, Result&lt;T, E>, Rc&lt;T>, Arc&lt;T>, Mutex&lt;T>, RwLock&lt;T>, HashMap&lt;K, V>

--
- **Pointer types**:<br/>
  &T, &mut T, Option&lt;&T>, Option&lt;&mut T>, Box&lt;T>, Option&lt;Box&lt;T>>,<br/>
  \*const T, \*mut T &larr; both unsafe

--
- **Tuples**:<br/>
  `(T, U)` or `(T, U, V)` or ...

--
- **User defined types**:<br/>
  `struct MyStruct(...)` or<br/>
  `struct MyStruct { ... }`,  and<br/>
  `enum MyEnum {}`

_Note: exercise_00_

---
class: big
## Variables

- Immutable variables:
```rust
let x = 5;
// x = 7; would result in compiler error
```

- Mutable variables:
```rust
let mut x = 5;
// Now x is allowed to be changed
x = 7;
```

_Note: exercise_01, exercise_02, exercise_03_

---
class: big
## Scopes
When an item goes out of scope, it is automatically cleaned up (memory is freed) and is no longer accessible. 
In rust this is called the lifetime of a variable, in the example below the lifetime is inferred from the code 
and doesn't require a lifetime declaration.

```rust
fn main() {
    let s = String::from("Outer scope");
    {
        let t = String::from("Inner scope");
        // t can be used inside these curly braces
    }
    // t has gone out of scope here, so cannot be used anymore
    pass_string(s);
}
```

_Note: exercise_04_

---
class: big
## Functions

Function are declared using `fn`. A function takes 0..n input parameters and an optional return type. 
The last statement of a function, without closing `;`, is the return value of that function.

.col-5[
```rust
fn print_integer(v: i32) {
    println!("{v}");
}
print_integer(42);
```

```rust
fn square(v: i32) -> i32 {
    v * v
}
let nine = square(3);
```
]
.col-1[
  &nbsp;
]
.col-6[
```rust
fn print_integer(v: i32) {
    if v < 0 { return; }
    println!("{v}");
}
print_integer(42);
```

```rust
fn as_lower(s: String) -> String {
    s.to_lowercase()
}
let lower = as_lower("UpPerCaSe".to_string());
```
]

_Note: exercise_05, exercise_06_

---
class: big
## Ownership - Passing ownership (1)

When assigning one variable to another, the original variable is no longer accessible.
```rust
fn main() {
    let s = String::from("Line");
    let t = s;          // ownership has moved from s to t
    // println!("{s}"); // would result in compiler error!
    println!("{t}");
}
```

_Note: primitive types and references have a Copy trait, which makes a copy behind the scenes_

_Note: exercise_07_

---
class: big
## Ownership - Passing ownership (2)

When passing a variable to a function, the original variable is no longer accessible.
```rust
fn pass_string(s: String) {
    println!("{}", s);
}

fn main() {
    let s = String::from("Line");
    pass_string(s);     // ownership has moved to function parameter
    // pass_string(s);  // would result in compiler error!
}
```

_Note: primitive types have a Copy trait, which makes a copy behind the scenes_

_Note: exercise_08_

---
class: big
## Ownership - Borrowing
Borrowing:

```rust
fn borrow_string(s: &String) {
    println!("{}", s);
}

fn main() {
    let s = String::from("Line");
    borrow_string(&s); // s borrowed by function borrow_string
    // s can still be used
    borrow_string(&s);
}
```

_Note: exercise_09, exercise_10, exercise_11_

---
class: lessbig
## Cargo

In the `exercises` folder, you can issue `cargo` commands in the terminal to achieve certain tasks:
- Building exercise_01: `cargo build --bin exercise_01`
- Running tests for exercise_01: `cargo test --bin exercise_01`
- Running exercise_01 (main method): `cargo run --bin exercise_01`

Other interesting commands
- Running the formatter: `cargo fmt --bin exercise_01`
- Running the linter: `cargo clippy --bin exercise_01`

Clone exercises and presentation using git with: <br/>`git clone https://github.com/Daanoz/rust-hands-on.git`

Start by making exercise 01 to 11. <br />Access our slides at `https://daanoz.github.io/rust-hands-on/`

_Note: This is a public repo, do not put any confidential information in here._

---
class: big
## Tuples

A tuple stores a collection of values of different types. Values are referred to by number (their order in the tuple).

```rust
let t = (29, "John", 6.1);
println!("{} is {} years old and is {} feet tall", t.1, t.0, t.2);
// t.3 // would lead to a compiler error
```
--
```rust
fn swap_pair(pair: (i32, i32)) -> (i32, i32) {
    let (x, y) = pair;
    (y, x)
}
let pair = (1, 2);
let swapped = swap_pair(pair);
println!("Original pair: ({}, {})", pair.0, pair.1);
println!("Swapped pair: ({}, {})", swapped.0, swapped.1);
```

_Note: exercise_12, exercise_13_

---
class: big
## Structures

.col-6[
```rust
struct Point { 
  x: f32, 
  y: f32 
}

impl Point {
    fn coords(&self) -> (f32,f32) {
        (self.x, self.y)
    }
}
let p = Point { 
  x: 3.0, 
  y: 4.0 
};
let (x, y) = p.coords();
println!("({}, {})", x, y);
```
]
.col-1[
  &nbsp;
]
--
.col-5[
```rust
struct Point(f32, f32)

impl Point {
  fn coords(&self) -> (f32,f32) {
    (self.0, self.1)
  }
}
let p = Point(3.0, 4.0);
let c = p.coords();
println!("({}, {})", c.0, c.1);
```

_Note: exercise_14, exercise_15, exercise_16, exercise_17_
]



---
class: big
## Control flow

- If statement
- Loops (for, while, loop)
- Enumerations 
- Pattern matching (match)
- Pattern matching alternative (if let)
  
---
class: big
## Control flow: if

If can be an assignment operation (must be exhaustive if used like that)!

.col-6[
```rust
let x = 10;
if x < 0 {
    println!("Negative number");
}
```
```rust
let x = 10;
if x < 0 {
    println!("Negative number");
} else {
    println!("Non-negative number");
}
```
]
.col-1[
  &nbsp;
]
--
.col-5[
```rust
let x = 10;
let text = if x < 0 {
    "Negative number"
} else {
    "Non-negative number"
};
println!("{text}");
```

_Note: exercise_18_
]


---
class: big
## Control flow - Loops

.col-3[
For loop:
```rust
let values = [1,2,3];
for i in values {
    println!("{i}");
}
```
]
--
.col-1[
  &nbsp;
]
.col-3[
While loop:
```rust
let mut i = 0;
while i < 10 {
    println!("{i}");
    i += 1;
}
```
]
--
.col-1[
  &nbsp;
]
.col-4[
Loop:
```rust
let mut i = 0;
loop {
    println!("{i}");
    i += 1;
    if i >= 10 {
       break;
    }
}
```
]

_Note: exercise_19_

---
class: big
## Enumerations

Defines a set of values, where a value can itself contain data.

```rust
enum MyEnum {
    NoValue,
    IntegerValue(i32),
    ComplexValue { x: f32, y: f32 },
}

let no_value = MyEnum::NoValue;
let integer_value = MyEnum::IntegerValue(10);
let complex_value = MyEnum::ComplexValue { x: 5.0, y: 7.0 };
```

_Note: exercise_20_

---
class: big
## Control flow: pattern matching

```rust
enum MyEnum {
    NoValue,
    IntegerValue(i32),
    ComplexValue { x: f32, y: f32 },
}

let value = MyEnum::ComplexValue { x: 5.0, y: 7.0 };

match value {
    MyEnum::NoValue => println!("NoValue"),
    MyEnum::IntegerValue(v) => println!("Integer value {v}"),
    MyEnum::ComplexValue { x, y } => println!("Complex value ({x}, {y})"),
};
```

_Note: exercise_21_

---
class: big
## Control flow: pattern matching alternative

```rust
enum MyEnum {
    NoValue,
    IntegerValue(i32),
    ComplexValue { x: f32, y: f32 },
}
let value = MyEnum::ComplexValue { x: 5f32, y: 7f32 };

if let MyEnum::ComplexValue { x, y } = value {
  println!("Only complex values are processed here: ({},{})", x, y);
}

if let MyEnum::IntegerValue(v) = value {
  println!("Only integer values are processed: {}", v);
} else {
  println!("Other values are processed differently");
}
```

---
class: big
## Standard enumerations - Option

Used to contain an optional value (there is no such thing as null pointers in Rust).<br/>

T is the type of the value in case it contains something.

.col-3[
```rust
enum Option<T> {
  Some(T),
  None,
}
```
]
--
.col-1[
  &nbsp;
]
.col-8[
```rust
fn div(n: f32, d: f32) -> Option<f32> {
  if d == 0f32 {
    None
  } else {
    Some(n / d)
  }
}
println!("y: {:?}", div(2f32, 3f32));
println!("y: {:?}", div(2f32, 0f32));
```
]

_Note: exercise_22_

---
class: big
## Standard enumerations - Result

Used to return a result of an operation that can succeed or fail.

T is the type of the value in case of success,<br/>
E is the type of the value in case of error.

.col-3[
```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
]
.col-1[
  &nbsp;
]
--
.col-8[
```rust
fn div(n: f32, d: f32) -> Result<f32, String> {
  if d == 0f32 {
    Err("Division by zero".to_string())
  } else {
    Ok(n / d)
  }
}
println!("y: {:?}", div(2f32, 3f32));
println!("y: {:?}", div(2f32, 0f32));
```
]

_Note: exercise_23_

---
class: big
## Next exercises

Continue with exercise 12 to 23.

---
class: lessbig
## Generics

Duplication of code can be prevented by making use of generics.

```rust
struct Container<T> {
    item: T,
}
impl<T> Container<T> {
    fn new(item: T) -> Container<T> {
        Container { item }
    }
    fn get_item(&self) -> &T {
        &self.item
    }
}
let int_container = Container::new(10);
let str_container = Container::new("Hello, world!");

println!("Integer: {}", int_container.get_item());
println!("String: {}", str_container.get_item());
```

_Note: exercise_25_

---
class: big
## Traits

Defines certain (group of) functionality, which can be implemented by `struct`s.

.col-5[
```rust
trait Speak {
    fn speak(&self) -> String;
}
struct Dog;
impl Speak for Dog {
    fn speak(&self) -> String {
        "Woof!".to_string()
    }
}
struct Cat;
impl Speak for Cat {
    fn speak(&self) -> String {
        "Meow!".to_string()
    }
}```
]
--
.col-1[
  &nbsp;
]
.col-6[
```rust
fn animal_speak<T: Speak>(animal: T) {
    println!("{}", animal.speak());
}

fn main() {
    let dog = Dog;
    let cat = Cat;

    animal_speak(dog);
    animal_speak(cat);
}```

_Note: exercise_24_
]

---
class: big
## Arrays, Vectors, HashSets, HashMaps

Different types are available for collections, each with their specific API.

```rust
let array_of_integers = [1, 2, 3, 3];
let vector_of_ints = vec![1, 2, 3, 3];
let hashset_of_ints = HashSet::from([1, 2, 3, 3]);
let hashmap_of_string_to_int = HashMap::from([
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("three", 3),
]);
println!("{:?}", array_of_integers);
println!("{:?}", vector_of_ints);
println!("{:?}", hashset_of_ints);
println!("{:?}", hashmap_of_string_to_int);
```

_Note: exercise_26_

---
class: big
## Closures

- Closures: `let f = |x: i32| -> i32 { x * 2 };`
- Or: `let f = |x| { x * 2 };`
- Or: `let f = |x| x * 2;`

```rust
fn main() {
    let double = |x| x * 2;
    let value = 5;
    println!("The double of {} is {}", value, double(value));
}
```

_Note: exercise_27_

---
class: big
## Iterators

There are two kinds of iterators, one that refers to objects and one that owns the objects.

```rust
let numbers = vec![1, 2, 3];
let referring_iterator: Iter<'_, i32> = numbers.iter();
let squared_1: Vec<_> =  referring_iterator
    .map(|n: &i32| n.pow(2))
    .collect();
println!("{:?}", squared_1);
let owning_iterator: IntoIter<i32> = numbers.into_iter();
let squared_2: Vec<_> =  owning_iterator
    .map(|n: i32| n.pow(2))
    .collect();
println!("{:?}", squared_2);
```
_Note: exchanging the order of referring and owning parts would result in compiler error_

---
class: lessbig
## Iterators - more operators

.col-5[
```rust
let values = [1, 2, 3];
values
  .into_iter()
  .filter(|value| *value % 2 != 0)
  .map(|value| value * value)
  .for_each(|value| println!("{value}"));
```
```rust
let somes_and_nones = [
    Some(1),
    None,
    Some(7)
];
let somes: Vec<_> = somes_and_nones
    .into_iter()
    .flatten()
    .collect();
println!("{:?}", somes);
```
]
--
.col-1[
  &nbsp;
]
.col-6[
```rust
let oks_and_errs = [
    Ok(1),
    Err(2.0),
    Ok(7),
    Err(9.0)];
let (oks, errs): (Vec<_>, Vec<_>) = oks_and_errs
    .into_iter()
    .partition(|v| v.is_ok());
println!("{:?}", oks);
println!("{:?}", errs);
```

_Note: exercise_28_
]

---
class: big
## Code generation

Macros can be used to let the compiler do code generation for us.

A frequently used macro is `derive`, which enriches `struct`s with "standard" functionality.

```rust
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Person {
    name: String,
    age: u32,
}
```
_Note: exercise_29_

---
class: big
## References and lifetimes

```rust
struct Person<'a> { name: &'a str }

impl<'a> Person<'a> {
    fn new(name: &'a str) -> Person<'a> {
      Person { name }
    }
    fn get_name(&self) -> &'a str {
      self.name
    }
}

fn main() {
    let name = String::from("Alice");
    let person = Person::new(&name);
    println!("Name is: {}", person.get_name());
}
```

_Note: exercise_30_

---
class: normal
## Using modules / public/ private

All code in rust is organized in modules, a file represents a module, which could be organized in sub modules using the `mod` keyword.

.col-7[
`src/data.rs`
```rust
pub fn connect() -> db::Db { db::Db { } }
pub mod db {
  pub struct Db { }
  impl Db {
    pub fn execute(&self, query: Query) -> Result<Vec<String>, String> {
      if !Self::is_valid(&query) {
        return Err(String::from("Invalid query"))
      }
      Ok(vec![String::from("Some"), String::from("Data")])
    }
    fn is_valid(query: &Query) -> bool {
      query.is_valid()
    }
  }
  pub struct Query {}
  impl Query {
    pub fn new() -> Self { Self {} }
    fn is_valid(&self) -> bool { true }
  }
}
```
]
--
.col-5[
`src/main.rs`
```rust
use data;

fn main() {
  let db = data::connect();
  let query = data::db::Query::new();
  let output = db.execute(query).expect("Valid query");
  println("Data: {:?}", output);
}
```
]


_Note: exercise_31_

---
class: big
## Advanced rust

- Macros
- Async / Futures
- RustDoc
- Cargo Crates / Workspaces
- Formatting / Static Code Analysis
- Building for different architectures
- Dependency injection
- Unit testing / Component testing / Doc testing
- Locking / Multithreading / Channels
- Into / From / TryInto / TryFrom
- Smart pointer (Arc, Box, Rc, ..)
- IO functions
- Unsafe
- FFI C/C++ integration
