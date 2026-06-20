<div class="title-block" style="text-align: center;" align="center">

# 🤷 Who cares test unwrap 🤷‍♀️

</div>

> [!CAUTION]
>
> This crate uses nightly Rust to customize the behavior of `?`.
>
> If you don't want to use a nightly toolchain in your tests, this crate will not work
> for you.
>
> The good news is that the
> [Try trait is getting close to stabilization](https://github.com/rust-lang/rust-project-goals/blob/main/src/2026/stabilize-try.md).

## Introduction

`who_cares` is a test helper package
that lets tests use `?` like unwrap for both `Option` and `Result`.

> [!TIP]
> Yes, that means you can use `?` on both `Option` and `Result` in the
> same test function **without any transform**

so the test can focus on the behavior under test instead of
being cluttered with `.unwrap()`.

## Example

```toml
# Cargo.toml
[dev-dependencies]
who_cares = { version = "0.1", features = ["macro"] }
```

```rust
use who_cares::who_cares;

#[test]
#[who_cares]
fn parses_fixture() {
    let text = std::fs::read_to_string("tests/fixtures/input.json")?;
    //                  ^^^^^^^^^^^^^^
    //                  std::io::Result<String>
    let value: serde_json::Value =
        serde_json::from_str(&text)?;
    //              ^^^^^^^^
    //              serde_json::Result<serde_json::Value>

    let id = value.get("id")?.as_u64()?;
    //             ^^^        ^^^^^^
    //             Option<&serde_json::Value>,
    //                        Option<u64>
    assert_eq!(id, 42);
}
```

`#[who_cares]` rewrites the test function to return `WhoCares<()>` and appends
`WhoCares(())` for you. It does not rewrite `?`.

The `macro` feature is optional. Of course, if you do not want to pull in the
macro dependencies, you can write the original form directly:

```toml
# Cargo.toml
[dev-dependencies]
who_cares = "0.1"
```

```rust
use who_cares::WhoCares;

#[test]
fn parses_fixture() -> WhoCares<()> {
    let text = std::fs::read_to_string("tests/fixtures/input.json")?;
    let value: serde_json::Value = serde_json::from_str(&text)?;
    let id = value.get("id")?.as_u64()?;

    assert_eq!(id, 42);
    WhoCares(())
}
```

If a `Result` is `Err`, `?` panics with the error's `Debug` representation.

If an `Option` is `None`, `?` panics with a `None` message.

## Scope

> [!WARNING]
> The name `WhoCares` is meant to be a warning.
>
> This crate should only be imported from your `[dev-dependencies]` and only
> used in `#[test]` code.

`WhoCares` works with any `Option` and any `Result<T, E>` where `E: Debug`.

This includes common result aliases such as `std::io::Result<T>`, `anyhow::Result<T>`,
`serde_json::Result<T>`, and similar aliases from other crates.

They all flow through the same generic `Result` support.

See the runnable examples in [`examples/`](examples/).
