# Rust hands-on, presentation with exercises

Link to [presentation](https://daanoz.github.io/rust-hands-on/).

This hands-on is targeted at rust beginners.

## Prerequisites

The workshop practices require that Rust is installed on your system.

Official install instructions: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Include `~/.cargo/bin` in your PATH environment variable, so rustc (the compiler), 
cargo (build/dependency tool) and rustup can be found.

Visual Studio Code is a convenient IDE for working with Rust. It is recommended to 
install the rust-analyzer extension, so during coding you get code completion, 
errors shown without having to compile first, ... The CodeLLDB debugger extension 
allows for debugging of your source code.

## Running the presentation

First install backslide:

```bash
npm install -g backslide
```

To run the presentation, change into the presentation folder, and run:

```bash
$ bs s
```
