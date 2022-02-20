# Tips

```rust
// import examples
use std::io;
use std::cmp::Ordering;
use rand::Rng;
```

```rust
// stdin
use std::io;
io::stdin().read_line(&mut input_str)
    .expect("Failed to read line.")
```

```rust
// Convert string to number (Ex. u32)
// Note:
//   - `num` on the right side is String
//   - trim: Delete blanks on both sides (delete `\n`, too)
//   - parse: Convert String to number
let num: u32 = num.trim().parse()
    .expect("Please type a number!");
```

```rust
// Example of error handling

// Before
let num: u32 = num.trim().parse()
    .expect("Please type a number!");

// After
let num: u32 = match num.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
```

```rust
// Semicolon
// Semicolon `;` converts expression to statement
fn main() {
    let x = 5; // statement (not return anything)
    
    let y = { // expression (return something)
        let x = 3;
        x + 1 // expression (Must not add `;`)
    };
    another_function(5);
}
```

```rust
// Copy of heap (Deep copy)
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone(); // let s2 = s1 : s1 will be moved
    println!("s1 = {}, s2 = {}", s1, s2);
}
```

```rust
// Ownership and function
fn main() {
    let s = String::from("hello");

    takes_ownership(s); // s will be moved because String is not Copy
    //println!("{}", s); <- This occur compile error!!!

    let x = 5;
    // x will be moved, but enable because i32 is Copy
    makes_copy(x);
    println!("{}", x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer); // x will be moved
}
```

```rust
// Reference
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

```rust
// Mutable reference
fn main() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s); // s was changed
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

```rust
// Slice
let s = String::from("hello world");
let hello = &s[0..5];  // The same as &s[..5] 
let world = &s[6..11]; // The same sa &s[6..]

let a = [1, 2, 3, 4, 5];
let slice = &a[2..4]; // type: &[i32]
```

```rust
// Combine strings
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // s1 will be moved

let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");
// Method 1
let s = s1 + "-" + &s2 + "-" + &s3; // s1 will be moved
println!("{}", s);
// Method2 (Better)
let s = format!("{}-{}-{}", s1, s2, s3); // all will not be moved
println!("{}", s);
```

```rust
// Scanning string
fn main() {
    // chars (intuitive and best method)
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    // bytes
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
```

```rust
// Indexing for string
let c = "アイウエオ".chars().nth(3).unwrap();
println!("{}", c);
```

```rust
// How to access Option
match score {
    Some(result) => println!("{}", result),
    None => println!("None"),
}
```

```rust
// HashMap
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // print
    println!("{:?}", scores);

    // How to access
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    match score {
        Some(result) => println!("{}: {}", team_name, result),
        None => println!("Failure"),
    }

    // How to scan
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}
```

```rust
// HashMap: Update and Insert
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    // Update
    scores.insert(String::from("Blue"), 25);

    // Insert if key does not exist
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
}
```

```rust
// Split string by space
for word in text.split_whitespace() {
    // Any process
}
```

```rust
// Example to count by HashMap
use std::collections::HashMap;

fn main() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1; // dereference
    }

    println!("{:?}", map);
}
```

```rust
// Get minimum value from hashmap
let min = hashmap.iter()
  .min_by(|a, b| a.1.cmp(&b.1))
  .unwrap();
println!("Key: {}, Value: {}", min.0, min.1);

// Get maximum value from hashmap
let max = map.iter()
    .max_by(|a, b| a.1.cmp(&b.1))
    .unwrap();
println!("Key: {}, Value: {}", max.0, max.1);
```

```rust
// or condition for match
match x {
    0 | 1 => println!("It is not a prime number."),
    n => println!("The number {} might be a prime number.", n)
}
```
