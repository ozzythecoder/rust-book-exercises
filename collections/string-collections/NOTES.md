## Strings

- Strings are collections
- "String" can refer to either `String` collections, or `&str` string slices.
- Complicated because humans and computers interpret strings very differently
- String slices are the only strings that exist in the core library, and include string literals ("Hello world!")
    - Stored in the binary

### Operations

**Creating a new string**

```rust
let mut s = String::new()
```

Creates a new, empty instance of the String collection
We can now load data into this empty String.

```rust
let data = "initial contents";
let s = data.to_string();
// equivalent to
let s = "initial contents".to_string();
// equivalent to
let s = String::from("initial contents");\
```

All strings are UTF-8 encoded, so we can include non-English characters as well.

**Updating a string**

Like a generic Vector `Vec<T>`, a String's size and contents can change.

```rust
let mut s = String::from("foo");
s.push_str("bar");
println!("{s}"); // "foobar"

// The push() method can also be used to add a char, but NOT a &str
s.push("z");

s.push("lol"); // ERROR: mismatched types, expect 'char', found '&str'
```

Can also combine strings with concatenation.
```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");

let s3 = s1 + &s2; // s3 takes ownership of s1, and so s1 can no longer be used
```
^^ Weird stuff going on up here. The + operator calls the `add()` method, which looks something like
`fn add(self, s: &str) -> String {}`

The + operator uses the `self` parameter, which is why `s3` takes ownership of `s1` in the example above.
Even though `&s2` is a `&String` and not a `&str`, the compiler uses *deref coercion* to turn `&s2` into a string slice.
(This seems to be one of the few times Rust actually uses type coercion.)

For more complex string combining, we can use the `format!` macro:
```rust
let s1 = String::from("foo");
let s2 = String::from("bar");
let s3 = String::from("baz");

let s = format!("The {foo}, the {bar}, and the {baz}");
```


