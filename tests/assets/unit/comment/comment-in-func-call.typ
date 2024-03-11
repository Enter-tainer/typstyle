#let f(a, b, c: none) = {
  [#a]
}

#f(1, /* actually, b is ignored*/ 0, c: /* actually, c is ignored */ 0)

#f(1, /* .... */)[/* .... */b]
