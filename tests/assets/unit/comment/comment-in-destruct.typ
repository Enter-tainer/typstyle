#{
  let (/* test */ a, b) = (1, 2);
  let (a, /* test */ _) = (1, 2);
  let (a: /* test */ h) = (
    a: 1
  )
}


#let (
// abc
a, b, c,
) = (1, 2, 3)

#let (
// abc
a, /* 1 */b, /* 2 */c, /* 3 */
)  /* 4 */  = (1, 2, 3)


#let (a, b:/* 8 */ (../* 9 */, /* 10 */d), ..c) = (a:1, b:(c: 4, d: 5))
#let (a/* 11 */,) = (a:1, b:(c: 4, d: 5))
#let (../* 12 */) = (a:1, b:(c: 4, d: 5))

#let ( // b
  /* 2 */ (a )  , /* 3 */) = (    (  ),  )
