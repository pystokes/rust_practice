# Error handle

```rust
// By match (With redundancy)
use std::fs::File;
use std::io;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == io::ErrorKind::NotFound => {
            // Need `ref` to prevent being moved (`&` is not appropriate in this case)
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!(
                        "Tried to create file but there was a problem: {:?}",
                        e
                    )
                },
            }
        },
        Err(error) => {
            panic!(
                "There was a problem opening the file: {:?}",
                error
            )
        },
    };
}
```

```rust
use std::fs::File;

fn main() {
    
    let f = File::open("hello.txt").unwrap();
    // or
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}
```

```rust
// Transfer error
use std::io;
use std::io::Read;
use std::fs::File;

fn main() {
    let a = read_username_from_file();
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e), // Need return
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s), // Not need return
        Err(e) => Err(e), // Not need return
    } // Not need semicolon
}
```

```rust
// Shortcut1 to transfer error (? operator)
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s) // Not need semicolon
}

// Shortcut2 to transfer error (? operator)
// More ergonomic
fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

```
