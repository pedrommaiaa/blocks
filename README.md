# Blocks

Generate comment blocks (literally).

## Install 

You need Rust and Cargo installed on your machine. See the installation guide
[here](https://doc.rust-lang.org/cargo/getting-started/installation.html).

Then clone this repo and install the CLI globally like this:

```sh
cargo install --path .
```

## Usage

`blocks <TEXT_HERE>`

Example:
```sh
blocks Hello, World!
```

Result:
```sh
// =============
// Hello, World!
// =============
```

## Credits

Modified from [blocky](https://github.com/virtualjpeg/blocky) & [headers](https://github.com/transmissions11/headers).
