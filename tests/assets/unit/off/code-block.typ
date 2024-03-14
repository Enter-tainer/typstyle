#let alert(body, fill: red) = {
  set text(white)
  set align(center)
  // @geshihua off
  rect(
    fill: fill,
        inset: 8pt,
    radius: 4pt, [*Warning:\ #body*],
  )
}

// @geshihua off
#alert[
  Danger is imminent!]
