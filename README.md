# typstyle

A beautiful and reliable code formatter for [Typst](https://typst.app/).

[![crates.io](https://img.shields.io/crates/v/typstyle)](https://crates.io/crates/typstyle)
[![docs](https://img.shields.io/badge/docs-latest-blue)](https://enter-tainer.github.io/typstyle/)
[![CI](https://github.com/Enter-tainer/typstyle/workflows/Test%20and%20Release/badge.svg)](…)
[![License](https://img.shields.io/crates/l/typstyle)](LICENSE)

[![Packaging status](https://repology.org/badge/vertical-allrepos/typstyle.svg)](https://repology.org/project/typstyle/versions)

## Installation and Usage

### Use as a CLI

#### Installation

You can install `typstyle` using any of the following methods:

1. Download the binary from the [release page](https://github.com/Enter-tainer/typstyle/releases/).
2. Install it from your package manager: <https://repology.org/project/typstyle/versions>.
   1. Notably, typstyle is available in [Archlinux CN](https://www.archlinuxcn.org/archlinux-cn-repo-and-mirror/) repo.
3. Install using [`cargo-binstall`](https://github.com/cargo-bins/cargo-binstall):

   ```sh
   cargo binstall typstyle
   ```

4. Install it with `cargo`:

   ```sh
   cargo install typstyle --locked
   ```

#### CLI Usage

```txt
Beautiful and reliable typst code formatter

Usage: typstyle [OPTIONS] [INPUT]...

Arguments:
  [INPUT]...  List of files or directories to format [default: stdin]

Options:
  -i, --inplace  Format the file in place
      --check    Run in 'check' mode. Exits with 0 if input is formatted correctly. Exits with a non-zero status code if formatting is required
  -h, --help     Print help
  -V, --version  Print version

Format Configuration:
  -l, --line-width <LINE_WIDTH>      Maximum width of each line [default: 80] [aliases: column] [short aliases: c]
  -t, --indent-width <INDENT_WIDTH>  Number of spaces per indentation level [default: 2] [aliases: tab-width]
      --no-reorder-import-items      Disable alphabetical reordering of import items
      --wrap-text                    Wrap text in markup to fit within the line width. Implies `--collapse-spaces`

Debug Options:
  -a, --ast         Print the AST of the input file
  -p, --pretty-doc  Print the pretty document
      --timing      Show elapsed time taken by the formatter

Log Levels:
  -v, --verbose  Enable verbose logging
  -q, --quiet    Print diagnostics, but nothing else
```

#### Examples

- Format a file in place:

  ```sh
  typstyle -i file.typ
  ```

- Format a file and print the result to stdout:

  ```sh
  typstyle file.typ
  ```

- Format multiple files in place:

  ```sh
  typstyle -i file1.typ file2.typ dir/
  ```

- Format all files in a directory:

  ```sh
  typstyle -i dir
  ```

- Read from stdin and print the result to stdout:

  ```sh
  cat file.typ | typstyle > file-formatted.typ
  ```

### Use in your editor

Typstyle has been integrated into [tinymist](https://github.com/Myriad-Dreamin/tinymist). You can use it in your editor by installing the tinymist plugin and set `tinymist.formatterMode` to `typstyle`.

### Use as a web app

Try the online version of the formatter at: <https://enter-tainer.github.io/typstyle/demo/>. You can see how it formats your code.

### Use as a Library

- **NPM**: <https://www.npmjs.com/package/typstyle-core>
- **Rust**: <https://crates.io/crates/typstyle-core>

### [3rd party] Use as a GitHub Action

The [typstyle-action](https://github.com/grayespinoza/typstyle-action) maintained by [@grayespinoza](https://github.com/grayespinoza) can install and run typstyle in github action.

## Features & Design

### Design Goals

1. **Opinionated**: We want to have a consistent style across all codebases.
2. **Code only**: We want to format only the code. Contents should be left untouched as much as possible.
3. **Convergence**: Running the formatter twice should not change the code.
4. **Correctness**: The formatter should not change the looking of the rendered output.

### Escape Hatch

If you find typstyle is not working as expected, you can use `// @typstyle off` or `/* @typstyle off */` to disable the formatter on the next node of code.

Typstyle also gives up formatting **part** of the code if it is not able to format it correctly. Specifically, it will print that part as is if:

- contains syntax error
- contains syntaxes that are not supported by the formatter

And please let us know the issue by creating an issue on the [GitHub repository](https://github.com/Enter-tainer/typstyle)

### Testing

We maintain a comprehensive suite of tests to ensure the correctness and reliability of typstyle.

1. **Convergence tests**: Format result must be the same when applied twice.
2. **Snapshot tests**: Format result are stored in the `snapshots` directory and are compared to the current result when running the tests.
3. **Correctness test**: We compare the rendered output of the code before and after formatting and ensure they are the same.
4. **E2E Correctness test**: We collect a bunch of typst code repo including `tablex`, `cetz`, `fletcher`... and format them to ensure: (a) the format result converges, and (b) the rendered output is the same.
5. **CLI tests**: We ensure that the CLI arguments can correctly control the behavior of the program, and the output is desirable.

### Benchmark

We provide benchmarks for node attribute computation and pretty printing. Typically, it can format a large document (given parsed source) within 5ms (e.g., `tablex.typ` with ~3000 lines).

We also have continuous benchmarking for each commit on `master` branch. See <https://enter-tainer.github.io/typstyle-bench-results/dev/bench/>.

## Why Another Formatter?

Why there is a need for another formatter? We already have [typstfmt](https://github.com/astrale-sharp/typstfmt), [typstfmt](https://github.com/jeffa5/typstfmt), [prettypst](https://github.com/antonWetzel/prettypst). Why another one?

Typstyle started because I had a bunch of ideas on how to improve typst source code formatting but kept finding typstfmt wasn't a good codebase to explore them with. Namely:

- I wanted to use Wadler's pretty printer (implemented by [pretty.rs](https://github.com/Marwes/pretty.rs)) to get consistent and beautiful output for any width. (Note that it is the same technique used in the prettier formatter)
- I didn't have much energy to maintain a bunch combination of configuration options. It turns out to be very hard to make everything correct. So I decided to make it opinionated.
- I wanted to experiment with more testing techniques and make sure the formatter is correct.

So I decided to write something from scratch. I started it about half a year ago and kept working on it in my spare time. Currently it lacks some advanced features but it is already usable for most of the cases. I hope you like it!

## Documentation

See <https://enter-tainer.github.io/typstyle/>.

## Roadmap

See the [tracking issue](https://github.com/Enter-tainer/typstyle/issues/15).

## Known Issues

You tell us! Bad formatting? Incorrect output? Please create an issue on the [GitHub repository](https://github.com/Enter-tainer/typstyle)!

We've set up comprehensive test suites to ensure the correctness of the formatter. If you find any issues, please let us know! And we can add more tests to prevent the issue from happening again.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for details.
