# Data type

See [official doc](https://doc.rust-jp.rs/book-ja/ch03-02-data-types.html).

## String

```rust
let some_str = "must be double quartatin";
```

## Number

```rust
let sum = 5 + 10;
let difference = 95.5 - 4.3;
let product = 4 * 30;
let remainder = 43 % 3;
```

## Bool

```rust
let t = true;
let f: bool = false;
```

## Character (Single quartation)

```rust
let c = 'z';
let z = 'ℤ';
let heart_eyed_cat = '😻';
```

## Tuple

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);

// How to access
let (x, y, z) = tup;
let five_hundred = tup.0;
let six_point_four = tup.1;
let one = tup.2;

// Update element
let mut tup1: (i32, f64, u8) = (500, 6.4, 1);
println!("{:?}", tup1);
tup1.0 = 10;
println!("{:?}", tup1);
```

## Array (For stack or fixed length)

```rust
let a = [1, 2, 3, 4, 5];
let months = ["Jan", "Feb", "Mar", "Apr", "May", "June",
              "July", "Aug", "Sep", "Oct", "Nov", "Dec"];

// How to access
let first = a[0];
let second = a[1];
```

## Vector

```rust
fn main() {
    let v1: Vec<i32> = Vec::new();
    let mut v2 = vec![1, 2, 3, 4, 5];

    // How to show an element
    let third: &i32 = &v2[2]; // Method 1
    let third: Option<&i32> = v2.get(2); // Method 2

    // Difference between method 1 and 2
    let does_not_exist = &v2[100]; // Panick (Error)
    let does_not_exist = v2.get(100); // Return None

    // Scan all elements
    for i in &v {
        println!("{}", i);
    }

    // Add to all elements
    let mut v3 = vec![1, 2 ,3 ,4 ,5];
    for i in &mut v3 {
        *i += 50; // * : dereference operator
    }
}
```

```rust
let mut v = vec![1, 2, 3];
let mut v1 = vec![5, 6, 7];
v.insert(1, 10);
v.append(&mut v1); // Note: v1 will be moved
```

```rust
// Multi types in a vector
enum SpredsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let row = vec![
        SpredsheetCell::Int(3),
        SpredsheetCell::Text(String::from("blue")),
        SpredsheetCell::Float(10.12),
    ];
}
```

```rust
let mut s = String::from("hello");
s.push_str(" world");
```
