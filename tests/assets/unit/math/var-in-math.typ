#let f(content) = {
  if type(content) in (float, int) {
          content = $#content$
  }
}
