# typstyle

[![codecov](https://codecov.io/gh/Enter-tainer/typstyle/graph/badge.svg?token=Y2SuYfwd7y)](https://codecov.io/gh/Enter-tainer/typstyle)

Usage: 
```
Usage: typstyle [OPTIONS] <INPUT>

Arguments:
  <INPUT>  Path to the input file

Options:
  -c, --column <COLUMN>  The width of the output [default: 80]
  -a, --ast              Print the AST of the input file
  -p, --pretty-doc       Print the pretty document
  -i, --inplace          Format the file in place
  -h, --help             Print help
  -V, --version          Print version
```

## Escape Route

If you find typstyle is not working as expected, you can use `// @typstyle off` or `/* @typstyle off */` to disable the formatter on the next node of code.

And please let us know the issue by creating an issue on the [GitHub repository](https://github.com/Enter-tainer/typstyle)

## Design Goals

1. Opinionated: We want to have a consistent style across all codebases.
2. Code only: We want to format only the code. Contents should be left untouched as much as possible.
3. Convergence: Running the formatter twice should not change the code.
4. Correctness: The formatter should not change the looking of the rendered output.

## Test

```sh
cargo test
cargo insta review
```

We have set up multiple tests:

1. Convergence tests: format result must be the same when applied twice
2. Snapshot tests: format result are stored in the `snapshots` directory and are compared to the current result when running the tests
3. Correctness test: We compare the rendered output of the code before and after formatting and ensure they are the same

## Known issues

- comments and white lines get removed when in strange places
