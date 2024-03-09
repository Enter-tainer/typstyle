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

Tested against cetz manual and tablex
