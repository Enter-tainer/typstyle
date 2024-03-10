# typst-geshihua

Usage: 
```
Usage: typst-geshihua [OPTIONS] <INPUT>

Arguments:
  <INPUT>  

Options:
  -c, --column <COLUMN>  The width of the output [default: 80]
  -a, --ast              Print the AST of the input file
  -p, --pretty-doc       Print the pretty document
  -i, --inplace          Format the file in place
  -h, --help             Print help
  -V, --version          Print version
```

## Test

```
cargo test
cargo insta review
```

Tested against cetz manual and tablex. We have test for 40/80/120 columns. We also have convergence test.

## Known issues

- comments and white lines get removed when it is not in code block or content block
- currently doesn't recognize 2d matrix syntax `$mat(1, 2; 3, 4)$`
- cannot handling single character variable in math block as positional args
