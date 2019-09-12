# Copy String
`&str` on the stack. Gains `Copy` semantics.

## Development Note
This crate is in its infancy. Right now it is more an 'act on my inspiration quickly' than a fully featured library.

## Rust Docs
[Documentation is here.](https://docs.rs/copystr/0.0.1)

## Installation

Add to your `Cargo.toml`:
```toml
[dependencies]
copystr = "0.0.1"
```

## Usage
Various fixed length strings are defined, `s4`, `,s8`, `s16` and `s32`. Use `TryFrom` to convert a `&str` into a copy string. Alternatively, define your own custom lenght copy strings using the `csstring` macro.
