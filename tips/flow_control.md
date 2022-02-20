# Flow control

```rust
// If
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

```rust
// Loop
fn main() {
    loop {
        println!("again!");

        // To stop, insert `break` anywhere
    }
}
```

```rust
// While
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        
        number = number - 1;
    }

    println!("LIFTOFF!!!");
}
```

```rust
// For
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("The value is: {}", element);
    }
}

// Sample (Range)
fn main() {
    for number in (1..4).rev() {
        println!("{}!");
    }
    println!("LIFTOFF!!!");
}
```
