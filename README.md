# cli-stats-vault

A lightweight, educational CLI written in **Rust** that reads a JSON dataset of
performance scores and prints basic statistics — **average**, **minimum**,
**maximum**, **variance** and **standard deviation** — in a clean terminal table.

Built as a deliberately small project to demonstrate a clean, modular Rust CLI:

- `Parser` → `Calculator` → `Formatter` pipeline
- Unit tests that cover edge cases (empty data, negative values, single values)
- Zero configuration; works with a file or standard input

## Features

- Accepts JSON from a file argument or piped through standard input.
- Tolerates two input shapes: a plain array or an object with a
  `scores` / `data` / `values` key.
- Reports both **sample** (`n-1`) and **population** (`n`) standard deviation.
- Configurable precision with `--decimals`.
- Clean error messages with a non-zero exit code on invalid input.

## Requirements

- **Rust** 1.85 or newer (edition 2024). Install it with
  [rustup](https://rustup.rs/):

  ```sh
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

  On Windows: download and run `rustup-init.exe` from <https://rustup.rs/>.

## Installation

Clone the repository and build a release binary:

```sh
git clone https://github.com/<your-user>/cli-stats-vault.git
cd cli-stats-vault
cargo build --release
```

The binary will be available at `target/release/cli-stats-vault`
(`target\release\cli-stats-vault.exe` on Windows). Optionally, copy it to a
directory in your `PATH`.

## Usage

```
cli-stats-vault [FILE] [OPTIONS]
```

| Argument   | Description                                                        |
| ---------- | ------------------------------------------------------------------ |
| `FILE`     | Path to a JSON dataset. Omit it or pass `-` to read from stdin.     |
| `--decimals N` | Number of decimal places to print (default: `2`).              |
| `-h, --help`   | Show help.                                                     |
| `-V, --version`| Show the version.                                              |

### Input format

Plain array:

```json
[9.2, 8.4, 7.9, 8.8, 6.5]
```

Object with a recognized key:

```json
{ "scores": [9.2, 8.4, 7.9, 8.8, 6.5] }
```

`"data"` and `"values"` are accepted as alternatives to `"scores"`.

### Examples

From a file:

```sh
cli-stats-vault examples/performance-scores.json
```

From standard input (Unix):

```sh
cat examples/performance-scores.json | cli-stats-vault
echo '[9.2, 8.4, 7.9]' | cli-stats-vault
```

From standard input (PowerShell on Windows):

```powershell
Get-Content examples/performance-scores.json -Raw | .\target\release\cli-stats-vault.exe
'[9.2, 8.4, 7.9]' | .\target\release\cli-stats-vault.exe
```

With custom precision:

```sh
cli-stats-vault --decimals 4 examples/performance-scores.json
```

### Sample output

Running on `examples/performance-scores.json`:

```
+-----------------------+-------+
| Metric                | Value |
+-----------------------+-------+
| Count                 | 10    |
| Sum                   | 79.60 |
| Average               | 7.96  |
| Minimum               | 5.90  |
| Maximum               | 9.20  |
| Variance (sample)     | 1.21  |
| Std dev (sample)      | 1.10  |
| Variance (population) | 1.09  |
| Std dev (population)  | 1.04  |
+-----------------------+-------+
Std dev: "sample" uses denominator n-1, "population" uses n.
```

### Exit codes

- `0` — success.
- `1` — any error: unreadable file, invalid JSON, non-numeric elements or an
  empty dataset.

## Project structure

```
cli-stats-vault/
├── Cargo.toml            # manifest and dependencies
├── examples/
│   └── performance-scores.json
└── src/
    ├── main.rs           # CLI entry point, I/O orchestration
    ├── parser.rs         # JSON -> Vec<f64>
    ├── calculator.rs     # Vec<f64> -> Statistics
    └── formatter.rs      # Statistics -> ASCII table
```

The flow is intentionally linear and directionally isolated —

```
stdin / file
   → Parser::parse_scores  →  Vec<f64>
   → Calculator::calculate →  Option<Statistics>
   → Formatter::format_table →  String  →  stdout
```

## Running the tests

```sh
# run every unit test
cargo test

# show every test by name
cargo test -- --list

# run only the calculator tests
cargo test calculator::
```

The suite includes edge cases such as empty datasets, negative values, a
single value, identical values and fractional scores.

Quality bar before committing or opening a PR:

```sh
cargo fmt --check   # formatting
cargo clippy -- -D warnings   # lints
cargo test          # tests
```

## Contributing via Pull Requests

Contributions are welcome! This project keeps the bar intentionally low so it
is a good first PR for people learning Rust.

### Step-by-step

1. **Fork** the repository on GitHub and clone your fork:

   ```sh
   git clone git@github.com:<your-user>/cli-stats-vault.git
   cd cli-stats-vault
   ```

2. **Create a branch** with a descriptive name:

   ```sh
   git checkout -b feat/add-percentile-metric
   ```

3. **Make your change.** Keep it small and focused on a single concern. Follow
   the existing architecture: if your change adds a metric, extend
   `Calculator`; if it changes presentation, touch `Formatter` only.

4. **Add or update unit tests.** New calculations must come with tests that
   cover the happy path and at least one edge case.

5. **Verify locally** — everything must be green:

   ```sh
   cargo fmt
   cargo clippy -- -D warnings
   cargo test
   ```

6. **Commit** with a clear, imperative message:

   ```sh
   git add .
   git commit -m "Add percentile metric to the calculator"
   git push -u origin feat/add-percentile-metric
   ```

7. **Open a Pull Request** from your branch against `main`. In the PR
   description, briefly explain *what* you changed and *why*, and paste the
   output of `cargo test`.

### Guidelines

- One logical change per PR — smaller PRs are reviewed and merged faster.
- Match the existing style (run `cargo fmt`).
- Never silence or ignore a failing test to make CI pass.
- Update the README only if your change alters behavior visible to the user.
- Be kind and constructive when reviewing others' PRs; the same applies to you.

## License

MIT. See `LICENSE` (add one before publishing the repository).