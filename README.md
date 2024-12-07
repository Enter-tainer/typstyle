# typstyle

[![Packaging status](https://repology.org/badge/vertical-allrepos/typstyle.svg)](https://repology.org/project/typstyle/versions)


## Usage
### Use as a CLI

#### Installation

1. Download the binary from the [release page](https://github.com/Enter-tainer/typstyle/releases/)
2. Install it from your package manager: <https://repology.org/project/typstyle/versions>
3. Install using [`cargo-binstall`](https://github.com/cargo-bins/cargo-binstall): `cargo binstall typstyle`
4. Install it using cargo: `cargo install typstyle --locked`

Usage: 
```txt
Beautiful and reliable typst code formatter

Usage: typstyle.exe [OPTIONS] [INPUT]... [COMMAND]

Commands:
  format-all  Format all files in-place in the given directory
  help        Print this message or the help of the given subcommand(s)

Arguments:
  [INPUT]...  Path to the input files, if not provided, read from stdin. If multiple files are provided, they will be processed in order

Options:
  -c, --column <COLUMN>  The column width of the output [default: 80]
  -a, --ast              Print the AST of the input file
  -p, --pretty-doc       Print the pretty document
  -i, --inplace          Format the file in place
      --check            Run in 'check' mode. Exits with 0 if input is formatted correctly. Exits with 1 if formatting is required
  -h, --help             Print help
  -V, --version          Print version
```

Typical usage:

- Inplace format a file:
```sh
typstyle -i file.typ
```

- Format a file and print the result to stdout:
```sh
typstyle file.typ
```

- Inplace format file list:
```sh
typstyle -i file1.typ file2.typ file3.typ
```

- Format all files in a directory. If the not provided, it will recursively format all files in the current directory:
```sh
typstyle format-all dir
```

- Read from stdin and print the result to stdout:
```sh
cat file.typ | typstyle > file-formatted.typ
```

### Use in your editor

typstyle has been integrated into [tinymist](https://github.com/Myriad-Dreamin/tinymist). You can use it in your editor by installing the tinymist plugin and set `tinymist.formatterMode` to `typstyle`.

### Use as a web app

There is an online version of the formatter at <https://enter-tainer.github.io/typstyle/> that you can see how it formats your code.

### Use with [pre-commit](https://github.com/pre-commit/pre-commit)

Add this to your `.pre-commit-config.yaml`:

```yaml
  - repo: https://github.com/Enter-tainer/typstyle
    rev: ''  # The the revision or tag you want to use
    hooks:
      - id: typstyle
```

## Escape Hatch

If you find typstyle is not working as expected, you can use `// @typstyle off` or `/* @typstyle off */` to disable the formatter on the next node of code.

typstyle also gives up formatting **part** of the code if it is not able to format it correctly. Specifically, it will print that part as is if:

- contains syntax error
- contains syntaxes that are not supported by the formatter

And please let us know the issue by creating an issue on the [GitHub repository](https://github.com/Enter-tainer/typstyle)

## Design Goals

1. Opinionated: We want to have a consistent style across all codebases.
2. Code only: We want to format only the code. Contents should be left untouched as much as possible.
3. Convergence: Running the formatter twice should not change the code.
4. Correctness: The formatter should not change the looking of the rendered output.

## Test

```sh
cargo nextest run -E 'not test(~e2e)' --no-fail-fast
cargo insta review
```

We have set up multiple tests:

1. Convergence tests: format result must be the same when applied twice
2. Snapshot tests: format result are stored in the `snapshots` directory and are compared to the current result when running the tests
3. Correctness test: We compare the rendered output of the code before and after formatting and ensure they are the same
4. E2E Correctness test: We collect a bunch of typst code repo including tablex, cetz, fletcher... and format them to ensure (a) the format result converges and (b) the rendered output is the same.

We also have continuous benchmarking for each commit on master branch. See https://enter-tainer.github.io/typstyle-bench-results/dev/bench/

## Use as a library

- npm: <https://www.npmjs.com/package/typstyle>
- rust: <https://crates.io/crates/typstyle>

## Known issues

You tell us! Bad formatting? Incorrect output? Please create an issue on the [GitHub repository](https://github.com/Enter-tainer/typstyle)!

We've set up comprehensive test suites to ensure the correctness of the formatter. If you find any issues, please let us know! And we can add more tests to prevent the issue from happening again.

## Why another formatter?

Why there is a need for another formatter? We already have [typstfmt](https://github.com/astrale-sharp/typstfmt), [typstfmt](https://github.com/jeffa5/typstfmt), [prettypst](https://github.com/antonWetzel/prettypst). Why another one?

typstyle started because I had a bunch of ideas on how to improve typst source code formatting but kept finding typstfmt wasn't a good codebase to explore them with. Namely:

- I wanted to use Wadler's pretty printer to get consistent and beautiful output for any width. (Note that it is the same technique used in the prettier formatter)
- I didn't have much energy to maintain a bunch combination of configuration options. It turns out to be very hard to make everything correct. So I decided to make it opinionated.
- I wanted to experiment with more testing techniques and make sure the formatter is correct.

So I decided to write something from scratch. I started it about half a year ago and kept working on it in my spare time. Currently it lacks some advanced features but it is already usable for most of the cases. I hope you like it!
