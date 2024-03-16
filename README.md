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

## Test

```
cargo test
cargo insta review
```

We have set up multiple tests:

1. Convergence tests: format result must be the same when applied twice
2. Snapshot tests: format result are stored in the `snapshots` directory and are compared to the current result

## Known issues

- comments and white lines get removed when in strange places
