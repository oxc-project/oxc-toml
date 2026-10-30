# AGENTS.md - AI Assistant Guide for oxc-toml

A formatter-only fork of [Taplo](https://github.com/tamasfe/taplo): a TOML 1.1 lexer, parser and formatter, with no runtime dependencies.
oxc's TOML formatter (`oxc_formatter_toml`) calls `parse()` and then `format()`.

## Scope

- Comments and meaningful whitespace are preserved.
- `parse()` is fault-tolerant: it always returns a tree, with the syntax errors beside it (`Parse::errors`).
- `format()` takes a `Parse` and leaves ranges with syntax errors as written.
- Syntax only: semantic errors are not detected (`SKIP_INVALID` in `tests/toml_test.rs` lists them).

## Testing

| Command                                       | Covers                                                                                                                 |
| --------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------- |
| `cargo test`                                  | unit tests (`tests/formatter.rs`), `toml-test` (`tests/toml_test.rs`), formatted output snapshot (`tests/snapshot.rs`) |
| `cargo insta review`                          | accept snapshot changes after an intentional formatting change                                                         |
| `just ready`                                  | the CI checks; needs a clean working tree (it starts with `git diff --exit-code`)                                      |
| `cargo run --example format_directory [PATH]` | format every TOML file under a directory, respecting `.gitignore`                                                      |

First run: `just clone-test-data` for `toml-test`, which `cargo test` needs (`just init` also installs the dev tools).

- `tests/toml_test.rs` reads the TOML 1.1.0 file list of `toml-test`:
  every valid file must parse, format idempotently and keep its value (compared through the `toml` crate);
  every invalid file must report a parse error.
  Exceptions are `SKIP_INVALID` / `SKIP_VALID_PARSE`, each with its reason.
- `toml-test` is not pinned: `just clone-test-data` pulls it, which can change the snapshot.
- `src/` has no unit tests (`test = false` in `Cargo.toml`).
