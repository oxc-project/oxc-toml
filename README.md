# oxc-toml

This is a formatter-only fork of [Taplo](https://github.com/tamasfe/taplo) by [Ferenc Tamás](https://github.com/tamasfe) for Oxfmt.

`oxc-toml` is a TOML formatter library that preserves comments and meaningful whitespace.

How it works is in [AGENTS.md](./AGENTS.md).

## Example

```rust
use oxc_toml::{format, parse, Options};

const SOURCE: &str = "value=1\n[table]\nstring='some string'";

let parsed = parse(SOURCE);
assert!(parsed.errors.is_empty());

let formatted = format(&parsed, Options::default());
```

## License

MIT, as Taplo.
See [LICENSE](LICENSE).

# [Sponsored By](https://oxc.rs/sponsor)

<p align="center">
  <a href="https://oxc.rs/sponsor">
    <img src="https://raw.githubusercontent.com/oxc-project/sponsors/main/sponsors.svg" alt="Our sponsors" />
  </a>
</p>
