# Variable and constant

```rust
let foo = 5;     // immutalbe
let mut foo = 5; // mutable
```

```rust
// Shadowing (Different address)
let x = 5;
let x = x + 1;
let x = x * 2;
```

```rust
// Able to change data type by shadowing
// because `let` generates new variable
let spaces = "    ";
let spaces = spaces.len();
```

```rust
// Not able to change data type by `mut`
let mut spaces = "    ";
spaces = spaces.len(); // COMPILE ERROR!
```

```rust
// Need data type for constant
const FOO_BAR: u32 = 100_000;
```
