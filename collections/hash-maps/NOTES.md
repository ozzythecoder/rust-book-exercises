## Hash Maps

- Type `HashMap<K, V>`
- Maps keys to values
- Similar to objects/maps in javascript

**Creation**

```rust
// Are not imported in the prelude
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("blue"), 10
scores.insert(String::from("Yellow"), 50);
```

**Accessing values**

```rust
// ...snip

let team_name = String::from("Blue");
let score = scores.get(&team_name).copied().unwrap_or(0);
```
This code fetches value from key "Blue", clones it, and assigns the variable score to the value (or 0 as a fallback if no value exists)

**Ownership**

```rust
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    let s = field_name; // compiler error - map has taken ownership of field_name
```

**Updating values**

Many options when updating data; what to do if there already exists data?

Overwriting (default behavior):
```rust
    score.insert(String::from("Blue"), 10);
    score.insert(String::from("Blue"), 25); // The value for Blue is now 25.
```

Keep existing value:
```rust

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 15);

    score.entry(String::from("Blue")).or_insert(50); // Value of Blue remains 15
    score.entry(String::from("Yellow")).or_insert(50); // Yellow doesn't exist; Yellow key is inserted with value 50
```

Update based on old value:
```rust
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    // this for loop counts the number of occurences of a word
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); // or_insert() returns a MUTABLE reference
        *count += 1; // dereferences, then mutates (still not sure what dereferencing is though)
    }

    println!("{:?}", map); // { "world": 2, "wonderful": 1, "hello": 1 }
```
