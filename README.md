# Google Hashcode Rust Template

Basic Rust template for Google Hashcode challenges.

## Quickstart

1. Clone this repository
2. Change crate author in [Cargo.toml](./Cargo.toml) ;-)

### Parsing

1. Add fields to `InputData` struct in [data.rs](./src/data.rs)
2. Implement `parse_input` in [data.rs](./src/data.rs)
``` rust
fn parse_input(s: &str) -> anyhow::Result<InputData> {}
```

Tip: [utils.rs](./src/utils.rs) contains a few helper functions for parsing lists of integers or list of strings.

```rust
fn number_list_line(s: &str) -> Res<&str, Vec<N>> {}
fn number_list_line_exact(s: &str, expected_size: usize) -> Res<&str, Vec<N>> {}
// same as number_list_line excepts it logs errors if any
fn number_list_line_verbose(s: &str) -> Res<&str, Vec<N>> {}
// same as number_list_line_exact excepts it logs errors if any
fn number_list_line_exact_verbose(s: &str, expected_size: usize) -> Res<&str, Vec<N>> {}

fn str_list_line(s: &str) -> Res<&str, Vec<&str>> {}
fn str_list_line_exact(s: &str, expected_size: usize) -> Res<&str, Vec<&str>> {}
// same as str_list_line excepts it logs errors if any
fn str_list_line_verbose(s: &str) -> Res<&str, Vec<&str>> {}
// same as str_list_line_exact excepts it logs errors if any
fn str_list_line_exact_verbose(s: &str, expected_size: usize) -> Res<&str, Vec<&str>> {}
```

see tests in [utils.rs](./src/utils.rs) for usage.

### Solving

1. Add fields to `OutputData` struct in [data.rs](./src/data.rs).
2. Implement `solve` in [data.rs](./src/data.rs).
```rust
fn solve(input: &InputData) -> anyhow::Result<(OutputData, Option<usize>)> {}
```

### Dumping

1. Implement `output_as_lines` in [data.rs](./src/data.rs).
```rust
fn output_as_lines(output: &OutputData) -> anyhow::Result<Vec<String>> {}
```

## Run the solver

```
RUST_LOG=info cargo run --release -- res/input_0.txt res/input_1.txt
```

Generates `./output_0.out` and `./output_1.out`.

### Enabling Logs

prefix command with `RUST_LOG=<level>` e.g.

```
RUST_LOG=info cargo run --release -- res/input_0.txt res/input_1.txt
```
