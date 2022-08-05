# gifski-lite

Highest-quality GIF encoder for the Web based on [pngquant](https://pngquant.org) and the original [gifski](https://github.com/ImageOptim/gifski).

**gifski-lite** converts video frames to GIF animations using pngquant's fancy features for efficient cross-frame palettes and temporal dithering. It produces animated GIFs that use thousands of colors per frame.

![(CC) Blender Foundation | gooseberry.blender.org](https://gif.ski/demo.gif)

This fork simplifies the Original [gifski](https://gif.ski) library in order to make it more easily compatible with the Web and as a WebAssembly module.

## Usage

gifski-lite is intended to target the Web only (potentially Node.js and Deno too).

The recommended way is to add this library as a dependency to a rust project that compiles to WebAssembly.

Currently used in experimental [gifski-wasm](https://github.com/jamsinclair/gifski-wasm) package. ⚠️ It is quite slow due to lack of using web-workers.

## Building

1. [Install Rust via rustup](https://www.rust-lang.org/en-US/install.html) or run `rustup update`. This project only supports up-to-date versions of Rust. You may get compile errors, warnings about "unstable edition", etc. if you don't run `rustup update` regularly.
2. Clone the repository: `git clone https://github.com/jamsinclair/gifski-lite`
3. In the cloned directory, run: `cargo build --release`

## License

AGPL 3 or later.

### Acknowledgements
This is a fork of the original [gifski](https://github.com/ImageOptim/gifski) project. This fork makes some significant changes:
- Refactors to slower single-threaded approach (for now, until can get threading working with WebAssembly)
- Remove features for ffmpeg and gifsicle
- Removes CLI tool and other compiled binaries
- Removes support for adding PNG frame files
- Removes C API
