#show raw: it => it.text.ends-with(">")

#show raw: it => (
  it.text.ends-with(">")
)

#show raw: it => if true {
      set text(1.2em)
    } else {
      it
    }


#show raw: it => {
  it
}
