#let f(content) = {
  if type(content) in (float, int) {
          content = $#content$
  }
}

$ lr([sum_(k = 0)^n e^(k^2)], size: #50%) $

#let x = 5
$ #x < 17 $
