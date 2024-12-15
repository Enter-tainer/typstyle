#let alert(body, fill: red) = {
  set text(white)
  set align(center)
  // @typstyle off
  rect(
    fill: fill,
        inset: 8pt,
    radius: 4pt, [*Warning:\ #body*],
  )
}

// @typstyle off
#alert[
  Danger is imminent!]
