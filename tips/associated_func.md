# Associated function

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size}
    }
}

fn main() {
    let sq = Rectangle::square(30);
    println!("Area is {}", sq.area());
}
```