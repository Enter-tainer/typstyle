# typstyle

A beautiful and reliable code formatter for [Typst](https://typst.app/).

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

Usage: typstyle [OPTIONS] [INPUT]... [COMMAND]

Commands:
  format-all   Format all files in-place in the given directory
  help         Print this message or the help of the given subcommand(s)

Arguments:
  [INPUT]...  Path to the input files, if not provided, read from stdin. If multiple files are provided, they will be processed in order

Options:
  -i, --inplace  Format the file in place
      --check    Run in 'check' mode. Exits with 0 if input is formatted correctly. Exits with 1 if formatting is required
  -h, --help     Print help
  -V, --version  Print version

Format Configuration:
  -c, --column <COLUMN>  The column width of the output [default: 80]

Debug Options:
  -a, --ast         Print the AST of the input file
  -p, --pretty-doc  Print the pretty document

Log Levels:
  -v, --verbose  Enable verbose logging
  -q, --quiet    Print diagnostics, but nothing else
```

```txt
Format all files in-place in the given directory

Usage: typstyle format-all [OPTIONS] [DIRECTORY]

Arguments:
  [DIRECTORY]  The directory to format. If not provided, the current directory is used

Options:
      --check  Run in 'check' mode. Exits with 0 if input is formatted correctly. Exits with 1 if formatting is required
  -h, --help   Print help

Format Configuration:
  -c, --column <COLUMN>  The column width of the output [default: 80]

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
  typstyle -i file1.typ file2.typ file3.typ
  ```

- Format all files in a directory. If the argument is not provided, it will recursively format all files in the current directory:

  ```sh
  typstyle format-all dir
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

### Known Issues

You tell us! Bad formatting? Incorrect output? Please create an issue on the [GitHub repository](https://github.com/Enter-tainer/typstyle)!

We've set up comprehensive test suites to ensure the correctness of the formatter. If you find any issues, please let us know! And we can add more tests to prevent the issue from happening again.

### Roadmap

See the [tracking issue](https://github.com/Enter-tainer/typstyle/issues/15).

### Documentation

See <https://enter-tainer.github.io/typstyle/>.

## Testing

We maintain a comprehensive suite of tests to ensure the correctness and reliability of typstyle.

### Types of Tests

1. **Convergence tests**: Format result must be the same when applied twice.
2. **Snapshot tests**: Format result are stored in the `snapshots` directory and are compared to the current result when running the tests.
3. **Correctness test**: We compare the rendered output of the code before and after formatting and ensure they are the same.
4. **E2E Correctness test**: We collect a bunch of typst code repo including `tablex`, `cetz`, `fletcher`... and format them to ensure: (a) the format result converges, and (b) the rendered output is the same.
5. **CLI tests**: We ensure that the CLI arguments can correctly control the behavior of the program, and the output is desirable.

### Running Tests

For developers, you need to install [cargo-nextest](https://nexte.st/) and [cargo-insta](https://insta.rs/) to run tests.

- List all tests:

  ```sh
  cargo nextest list --workspace
  ```

- Run all tests and review snapshots:

  ```sh
  cargo nextest run --workspace --no-fail-fast
  cargo insta review
  ```

- Run snapshot tests only:

  ```sh
  cargo nextest run --workspace -E 'test([typst])' --no-fail-fast --no-default-features
  cargo insta review
  ```

- Run tests excluding end-to-end (e2e):

  ```sh
  cargo nextest run --workspace -E 'not test(~e2e)' --no-fail-fast
  ```

- Run tests for CLI:

  ```sh
  cargo nextest run -p typstyle --no-fail-fast
  ```

## Benchmark

We provide benchmarks for node attribute computation and pretty printing. Typically, it can format a large document (given parsed source) within 5ms (e.g., `tablex.typ` with ~3000 lines).

We also have continuous benchmarking for each commit on `master` branch. See <https://enter-tainer.github.io/typstyle-bench-results/dev/bench/>.

### Running benches

- List benchmarks:

  ```sh
  cargo bench --workspace -- --list
  ```

- Run benchmarks:

  ```sh
  cargo bench --workspace
  ```

The benchmark results is generated by [criterion.rs](https://github.com/bheisler/criterion.rs). You can check `target/criterion/report` to see the reports.

## Why Another Formatter?

Why there is a need for another formatter? We already have [typstfmt](https://github.com/astrale-sharp/typstfmt), [typstfmt](https://github.com/jeffa5/typstfmt), [prettypst](https://github.com/antonWetzel/prettypst). Why another one?

Typstyle started because I had a bunch of ideas on how to improve typst source code formatting but kept finding typstfmt wasn't a good codebase to explore them with. Namely:

- I wanted to use Wadler's pretty printer (implemented by [pretty.rs](https://github.com/Marwes/pretty.rs)) to get consistent and beautiful output for any width. (Note that it is the same technique used in the prettier formatter)
- I didn't have much energy to maintain a bunch combination of configuration options. It turns out to be very hard to make everything correct. So I decided to make it opinionated.
- I wanted to experiment with more testing techniques and make sure the formatter is correct.

So I decided to write something from scratch. I started it about half a year ago and kept working on it in my spare time. Currently it lacks some advanced features but it is already usable for most of the cases. I hope you like it!
