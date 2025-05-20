#let fun = (..args) => args
#let (aaaaaaaa, bbbbbbbbbb) = (1, 2)

#fun(aaaaaaaa, bbbbbbbbbb, () => {

})

#fun(aaaaaaaa, bbbbbbbbbb, () => {})

#fun(aaaaaaaa, bbbbbbbbbb, () => {
  // something
})

#fun(aaaaaaaa, bbbbbbbbbb, {
  // something
})
