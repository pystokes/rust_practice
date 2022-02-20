# Struct

```rust
// Struct
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User { // `let mut` for mutable
        email: String::from("someone@example.com"),
        username: String::from("some_username"),
        sign_in_count: 10,
        active: true,
    };

    println!("{}", user1.username);
}
```

```rust
// Tuple struct
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0, );
let origin = Point(0, 0, 0);
```

```rust
// Debug for struct instance
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:#?}", rect1);

}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
```

```rust
// Implementation (method)
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {

}
```