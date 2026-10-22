# Layout

Layout is a rust library and a stand alone utility that draws graphs. Layout
can parse Graphviz dot files and render them.

This repository is a fork of the original
[nadavrot/layout](https://github.com/nadavrot/layout) project. The original
repository is not actively maintained at the moment, so ongoing work is being
published here. Changes are intended to be pushed upstream when the original
project is maintained again.

[![Latest Version]][crates.io] [![Docs Badge]][docs]

[Latest Version]: https://img.shields.io/crates/v/layout-rs.svg
[crates.io]: https://crates.io/crates/layout-rs
[Docs Badge]: https://docs.rs/layout-rs/badge.svg
[docs]: https://docs.rs/layout-rs

## Library Usage

Add the following to `Cargo.toml`:

```toml
layout-rs = "0.1.2" # or
layout-rs = { version = "0.1.2", features = ["log"] }
```

Load, parse and print the AST:

```rust
use layout::gv;

let contents = "digraph { a -> b [label=\"foo\"]; }";
let mut parser = gv::DotParser::new(&contents);

match parser.process() {
    Ok(g) => gv::dump_ast(&g),
    Err(err) => {
        parser.print_error();
        # #[cfg(feature = "log")]
        log::error!("Error: {}", err);
    }
}
```

The example above would print the program AST, or a readable error message,
such as:

```txt
digraph {

node [fillcolor="purple"] A B;
node [fillcolor="orange"] Z;
node [fillcolor="green"] G; a = ;
                                ^
Error: Expected an identifier.
```


## Command Line Usage

Build the crate and render some dot files with the command

```bash
cargo run --bin layout ./inputs/bk.dot -o output.svg
```

## WebAssembly Usage

The `wasm` workspace member exposes the same DOT parsing, layout, and SVG
rendering pipeline as a browser-callable function. It takes DOT source as a
string and returns SVG as a string; it does not use the CLI's filesystem or
command-line handling.

Install the WebAssembly target and `wasm-pack`, then build the generated
JavaScript bindings for Vite from the repository root:

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
cd ../layout-web
npm run wasm:build
```

The generated package exports `render_dot(source, disable_optimizations,
disable_layout)`. Import and initialize that package from the Svelte app, then
pass the editor contents to `render_dot` and display the returned SVG. Rebuild
the package whenever the Rust wrapper or library changes. The generated output
is a build artifact and should not be committed.

## Gallery

This section presents a few graphs that were rendered from dot files:

A simple graph.

![](docs/graph.png)

A simple graph with multiple shapes and labels.

![](docs/graph2.png)

A graph with a few style properties.

![](docs/colors.png)

A large graph that demonstrates the edge crossing elimination optimization.

![](docs/bk.png)

Unicode, emoji and right-to-left languages:

![](docs/heb.png)

Support for Records (nested structures):

![](docs/records.png)

Debug-mode rendering that helps to visualize the layout decisions:

![](docs/debug.png)
