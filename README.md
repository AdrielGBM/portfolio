# portfolio

Adriel Barrientos's personal portfolio: a [Telar](https://github.com/AdrielGBM/telar) `web-dom` app, built as scroll-driven kinetic typography.

## Development

The dev shell is provided by this project's own `flake.nix`, which builds `cargo-telar` from the sibling `../telar` checkout so the CLI always matches the `[patch.crates-io]` version of the `telar` crate. With direnv, `cd` into the project activates it automatically; otherwise:

```sh
nix develop
```

Then:

```sh
cargo telar dev --target web --renderer dom
```

After committing in `../telar`, run `nix flake update telar` so the shell's `cargo-telar` follows it.

## Building

```sh
cargo telar build --target web --renderer dom
```

The output lands in `target/telar-dist/web`.
