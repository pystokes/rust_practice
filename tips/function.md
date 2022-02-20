# Function

```rust
fn main() {
    let x = 5; // statement (not return anything)
    
    let y = { // expression (return something)
        let x = 3;
        x + 1 // expression (`;` converts expression to statement)
    };
    another_function(5);
}

// Need data type
fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}
```

```rust
// Functin with return
fn five() -> i32 {
    5 // expression (= without `;`)
}

fn main() {
    let x = five();

    println!("The value of x is: {}", x);
}
```
