#text[123]

#text()[123]

#text(weight: "bold")[123]

#table()[123][456]

#table[123][456]

#link("http://example.com")[test]

#let f = (..args) => args
#f({ "aaaaaaaaaaaaaaaaaaaaa" })
#f(((),))

#f({

})
#f(`xx
`)
#f(```aa
```)
#f((

))
#pad(left: 1em)[```typ

```]
