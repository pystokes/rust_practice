# Argument

```rust
//   & : reference
// mut : mutable argument
use std::io;
io::stdin().read_line(&mut input_str)
    .expect("Failed to read line.")
```
