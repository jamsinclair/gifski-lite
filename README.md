# gifski-lite

### - ⚠️ This is a highly experimental fork and is not recommended for production use.
### - ℹ️ You probably want to use the JavaScript module [gifski-wasm](https://github.com/jamsinclair/gifski-wasm) which uses this library

Highest-quality GIF encoder for the Web based on [pngquant](https://pngquant.org) and the original [gifski](https://github.com/ImageOptim/gifski).

This fork simplifies the Original [gifski](https://gif.ski) library in order to make it more easily compatible with the Web and as a WebAssembly module.

## Usage

gifski-lite is intended to target the Web only (potentially Node.js and Deno too).

The recommended way is to add this library as a dependency to a rust project that compiles to WebAssembly.

Currently used in [gifski-wasm](https://github.com/jamsinclair/gifski-wasm) package. It is ~quite slow due to lack of~ now fast because it supports web-workers.

See the WASM Web App in action at [gifski-wasm.netlify.app](https://gifski-wasm.netlify.app) and other example usage at [jamsinclair/gifski-wasm/examples](https://github.com/jamsinclair/gifski-wasm/tree/main/examples).

## Building

1. [Install Rust via rustup](https://www.rust-lang.org/en-US/install.html) or run `rustup update`. This project only supports up-to-date versions of Rust. You may get compile errors, warnings about "unstable edition", etc. if you don't run `rustup update` regularly.
2. Clone the repository: `git clone https://github.com/jamsinclair/gifski-lite`
3. In the cloned directory, run: `cargo build --release`

## License

AGPL 3 or later.

### Acknowledgements
This is a fork of the original [gifski](https://github.com/ImageOptim/gifski) project. This fork makes some significant changes:
- Refactors to optionally use rayon for parallel processing
- Remove features for ffmpeg and gifsicle
- Removes CLI tool and other compiled binaries
- Removes support for adding PNG frame files
- Removes C API
