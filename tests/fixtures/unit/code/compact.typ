#{
  let f(..) = none

  f(   if true {    let x = 3  })
  f(if true {
    let x = 3
  })
  f(    1111,    22222,    if true {
      let x = 3
      let y = 4
    },
  )
  f(    1111,    if true {      let x = 3   },    22222, )
  f(    1111,    if true {      let x = 3 ;  let y = 4   },    22222, )

  let base-weight = none
  assert(base-weight in (auto, none) or type(base-weight) in (str, int), message: "`base-weight` should be `auto`, `none`, `int` or `str` type.")
}


#{let c = 1 + 2 * 3 == 4 + 5 and 6 < 7}
#let c = 1 + 2 * 3 == 4 + 5 and 6 < 7

#str(
          "Unsupported content type "
            + str(type(content))
            + "! "
            + "Provide your own `draw-node` implementation.",
        )

#((1,2),2,3).rev().rev().at(0).rev().rev().rev()
