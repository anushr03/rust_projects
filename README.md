This Repo keeps track of all the projects I did to learn the Rust language using the Rust official programming language pdf([book](https://doc.rust-lang.org/book/title-page.html))

# 1. Hello_world and Hello_cargo

Intial hello wrold code to introduce myself to the Rust Programming language

# 2. guessing_game

A user input guessing game, run via the terminal, where the user keeps on guessing the number until it matches the random number seelcted by the code. I added another functionality to this where the user can input q and exit the game at any given time.

# 3. variables

An introduction to variables in Rust. Learned about shadowing, floating points and difference between tuples and arrays in Rust

# 4. Functions

Learned about functions.

### a. Statements and Expressions

Firstly learned about the difference between statements and expression. Statements do not return a value and expressions return a value. This is why expressions do not have a semicolon. If we add the semicolon, it becomes a statement and therefore will not return a value.

### b. Functions with return values

Functions can return values to the code that calls them. We don’t name return values, but we must declare their type after an arrow (->)

# 5. Control Flow

Learned about if expressions and loops

### a. if statements

An if expression allows you to branch your code depending on conditions. You provide a condition and then state, “If this condition is met, run this block of code. If the condition is not met, do not run this block of code.”

### b. Loops

It’s often useful to execute a block of code more than once. For this task, Rust provides several loops, which will run through the code inside the loop body to the end and then start immediately back at the beginning. To experiment with loops, let’s make a new project called loops.

Rust has three kinds of loops: **loop**, **while**, and **for**.

# 6. Understanding Ownership

Ownership is a set of rules that govern how a Rust program manages memory. In Rust, memory is managed through a system of ownership with a set of rules that the compiler checks.

### a. Stacks and Heaps

The memory can find data faster when its on a stack as compared to the heap. All data stored on the stack must have a known, fixed size. Data with an unknown size at compile time or a size that might change must be stored on the heap instead.

### b. Ownership Rules

1. Each value in the rust has an _owner_
2. There can only be one owner at a time
3. When the owner goes out of scope, the value will be dropped

### c. Variable scope

```rust {"id":"01J2RWCZY390Y7X5968JF23SJ0"}
fn main() {
    let s = "hello";
    println!("{s}\n");
}
```

Here the scope of s is only valid between the two `{}` brackets.

In other words, there are two important points in time here:

- When s comes into scope, it is valid.
- It remains valid until it goes out of scope.

At this point, the relationship between scopes and when variables are valid is similar to that in other programming languages.

### d. The `string` type

Unlike other data types that we have seen, `string` is a complex data type, which is stored on the heap.

I can also create a `string` from the string literal by using the `from` keyword. For eg.

```rust {"id":"01J2RWCZY390Y7X5968M7YAQ0N"}
let s = String::from("hello");
```

### e. Memory and Allocation

In the case of a string literal, we know the contents at compile time, so the text is hardcoded directly into the final executable. This is why string literals are fast and efficient. But these properties only come from the string literal’s immutability. Unfortunately, we can’t put a blob of memory into the binary for each piece of text whose size is unknown at compile time and whose size might change while running the program.

With the `String` type, in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents. This means:

- The memory must be requested from the memory allocator at runtime.
- We need a way of returning this memory to the allocator when we’re done with our `String`.

That first part is done by us: when we call `String::from`, its implementation requests the memory it needs. This is pretty much universal in programming languages.

In Rust, the memory is instantly returned, once the variable goes out of scope. For eg, the code below but with `String`, instead of the string literal

```rust {"id":"01J2RWCZY4XE09TTCS6X37PB6C"}
{
    let s = String::from("hello"); // s is valid from this point forward

    // do stuff with s

}   // this scope is now over, and s is no longer valid
```

### f. Referencing and Borrowing

In Rust you cannot have two refereneces at the same time. The restriction preventing multiple mutable references to the same data at the same time allows for mutation but in a very controlled fashion.The benefit of having this restriction is that Rust can prevent data races at compile time. A _data race_ is similar to a race condition and happens when these three behaviors occur:

- Two or more pointers access the same data at the same time.
- At least one of the pointers is being used to write to the data.
- There’s no mechanism being used to synchronize access to the data.

Data races cause undefined behavior and can be difficult to diagnose and fix when you’re trying to track them down at runtime; Rust prevents this problem by refusing to compile code with data races!

Although to have multiple mutable references, we can always define a scope using `{}` brackets. For eg.

```rust {"id":"01J2RWCZY4XE09TTCS6XFW8RKS"}
let mut s = String::from("HellO");
{
    let s1 = &mut s;
} // s1 goes out of scope here, so another mutable reference can be made to s

let s = &mut s;
```

The issue is only with mutable reference. We can have as many immutable references as we like. So the code below would compile.

```rust {"id":"01J2RWCZY4XE09TTCS6YFXGB6E"}
let  s = String::from("HellO");
let s1 = &s;
let s2 = &s;
```

Also, we cannot have mutable and immutable references at the same time. For eg.

```rust {"id":"01J2RWCZY4XE09TTCS70YF2MER"}
let mut s = String::from("HellO");
let s1 = &s;
let s2 = &s;
let s3 = &mut s;
```

The code above will throw a compile-time error for s3. Although if the code below is implemented, the compiler will compile the code.

```rust {"id":"01J2RWCZY4XE09TTCS72QH5QM0"}
let mut s = String::from("HellO");
let s1 = &s;
let s2 = &s;
println!("s1 = {s1}, s2 = {s2}");
let s3 = &mut s;
println!("{s3}");
```

### The Rules of References

- At any given time, you can have either one mutable reference or any number of immutable references.
- References must always be valid.

### f. The Slice type

Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection. A slice is a kind of reference, so it does not have ownership.

# 7. Structs

A struct or a structure is a custom data type that lets you package together and name multiple related values that make up a meaningful group

Unlike Tuples, in struct we name each piece of data so its clear what the values mean

### a. Tuple Structs

Rust also supports structs that look similar to tuples, called *tuple structs*. Tuple structs have the added meaning the struct name provides but don’t have names associated with their fields; rather, they just have the types of the fields. Tuple structs are useful when you want to give the whole tuple a name and make the tuple a different type from other tuples, and when naming each field as in a regular struct would be verbose or redundant.

# 8. Methods

Methods are similar to functions: we declare them with the fn keyword and a name, they can have parameters and a return value, and they contain some code that’s run when the method is called from somewhere else. Unlike functions, methods are defined within the context of a struct (or an enum or a trait object), and their first parameter is always self, which represents the instance of the struct the method is being called on.

# 9. Enumerators

Where structs give you a way of grouping together related fields and data, like a `Rectangle` with its `width` and `height`, enums give you a way of saying a value is one of a possible set of values. For example, we may want to say that `Rectangle` is one of a set of possible shapes that also includes `Circle` and `Triangle`. To do this, Rust allows us to encode these possibilities as an enum.

### a. The `option` enum

Rust does not have a `null`. But the concept behind the `null` is important. So Rust has the `Option<t>` that is an enum that can encode the concept of the value being present or absent.

```rust {"id":"01J2RWCZY4XE09TTCS72TR3EGM"}
enum Option<T> {
    None,
    Some(T),
}
```

### b.`if let`

The `if let` syntax lets you combine `if` and `let` into a less verbose way to handle values that match one pattern while ignoring the rest.

```rust {"id":"01J2RWCZY4XE09TTCS7439MJ38"}
let config_max = Some(3u8);
if let Some(max) = config_max {
    println!("The maximum is configured to be {max}");
}
```

The syntax `if let` takes a pattern and an expression separated by an equal sign. It works the same way as a `match`, where the expression is given to the `match` and the pattern is its first arm. In this case, the pattern is `Some(max)`, and the `max` binds to the value inside the `Some`. We can then use `max` in the body of the `if let` block in the same way we used `max` in the corresponding match arm. The code in the `if let` block isn’t run if the value doesn’t match the pattern.

# 10. Packages and Crates

A _crate_ is the smallest amount of code that the Rust compiler considers at a time. Even if you run `rustc` rather than `cargo` and pass a single source code file, the compiler considers that file to be a crate. Crates can contain modules, and the modules may be defined in other files that get compiled with the crate, as we’ll see in the coming sections.\

Two Types of Crate:

* __Binary crates__ are programs you can compile to an executable that you can run, such as a command-line program or a server. Each must have a function called `main` that defines what happens when the executable runs. All the crates we’ve created so far have been binary crates.
* __Library crates__ don’t have a `main` function, and they don’t compile to an executable. Instead, they define functionality intended to be shared with multiple projects.

The _crate root_ is a source file that the Rust compiler starts from and makes up the root module of your crate

### a. Modules

Modules let us organize code within a crate for readability and easy reuse. Modules also allow us to control the privacy of items, because code within a module is private by default. Private items are internal implementation details not available for outside use. We can choose to make modules and the items within them public, which exposes them to allow external code to use and depend on them.

# 11. Common collections

1. Vector: A vector allows you to store a variable number of values next to each other
2. String: A string is a collection of data.
3. Hash Map: A hash map allows you to associate a value with a particular key.

### a. Vectors

Vectors can only store values of the same types.

### b. String
The `String` type, which is provided by Rust’s standard library rather than coded into the core language, is a growable, mutable, owned, UTF-8 encoded string type

Many of the same operations available with `Vec <T>` are available with `String` as well, because `String` is actually implemented as a wrapper around a vector of bytes with some extra guarantees, restrictions, and capabilities. An example of a function that works the same way with `Vec<T>` and `String` is the new function to create an instance

```rust
let mut s = String::new();
```

### c. Hash Maps
The type `HashMap<K, V>` stores a mapping of keys of type `K` to values of type `V` using a hashing function, which determines how it places these keys and values into memory.