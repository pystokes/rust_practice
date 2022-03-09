# rust_practice

Practice and tips of Rust

## Setup

For Apple silicon, add `--platform linux/amd64` for both commands.

```bash
cd rust_practice
docker build -t rust .
docker run -it --rm -v `pwd`:/rust/ rust
```
