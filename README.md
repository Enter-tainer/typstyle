# typstyle

[![codecov](https://codecov.io/gh/Enter-tainer/typstyle/graph/badge.svg?token=Y2SuYfwd7y)](https://codecov.io/gh/Enter-tainer/typstyle)

Usage: 
```
A typst source code formatter

Usage: typstyle [OPTIONS] [INPUT]

Arguments:
  [INPUT]  Path to the input file, if not provided, read from stdin

Options:
  -c, --column <COLUMN>  The width of the output [default: 80]
  -a, --ast              Print the AST of the input file
  -p, --pretty-doc       Print the pretty document
  -i, --inplace          Format the file in place
  -h, --help             Print help
  -V, --version          Print version```
```

## Escape Route

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
cargo test
cargo insta review
```

We have set up multiple tests:

1. Convergence tests: format result must be the same when applied twice
2. Snapshot tests: format result are stored in the `snapshots` directory and are compared to the current result when running the tests
3. Correctness test: We compare the rendered output of the code before and after formatting and ensure they are the same

## Known issues

You tell us! Bad formatting? Incorrect output? Please create an issue on the [GitHub repository](https://github.com/Enter-tainer/typstyle)!

We've set up comprehensive test suites to ensure the correctness of the formatter. If you find any issues, please let us know! And we can add more tests to prevent the issue from happening again.

## Why another formatter?

Why there is a need for another formatter? We already have [typstfmt](https://github.com/astrale-sharp/typstfmt) and [typstfmt](https://github.com/jeffa5/typstfmt). Why another one?

typstyle started because I had a bunch of ideas on how to improve typst source code formatting but kept finding typstfmt wasn't a good codebase to explore them with. Namely:

- I wanted to use Wadler's pretty printer to get consistent and beautiful output for any width. (Note that it is the same technique used in the prettier formatter)
- I didn't have much energy to maintain a bunch combination of configuration options. It turns out to be very hard to make everything correct. So I decided to make it opinionated.
- I wanted to experiment with more testing techniques and make sure the formatter is correct.

So I decided to write something from scratch. I started it about half a year ago and kept working on it in my spare time. Currently it lacks some advanced features but it is already usable for most of the cases. I hope you like it!
