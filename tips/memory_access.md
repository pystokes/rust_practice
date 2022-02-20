# Memory access

## Same address

```rust
let mut x = 5;
println!("{:p}", &x); // 1: Same address with 2

x = x + 1;
println!("{:p}", &x); // 2: Same address with 1
```

## Different address

```rust
let x = 5;
println!("{:p}", &x); // 1: Same address with 2

let x = x + 1; // Binding
println!("{:p}", &x); // 2: Different address with 1
```
