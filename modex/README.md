# Module system usage example
This project shows how you can effectively use Rust module system to structure complex projects.
The `lib_a` has a simple structure: all exposed functions (public API) are in the `lib_a/src/lib.rs` file.
The structure of the `lib_b` is more involved: there two modules `lib_b/src/add.rs` and `lib_b/src/sub.rs`. These modules are declared in the `lib_b/src/lib.rs` file and their public members are exposed as public API.

## Build
Build the entire workspace from the root directory and run tests:
```
cargo clean && cargo test
```
Run the CLI app with:
```
cargo run
```