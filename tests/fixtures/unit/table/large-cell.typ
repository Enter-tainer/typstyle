#figure(
  grid(
    columns: (auto, auto),
    rows: (auto, auto),
    gutter: 0em,
    [ #read("large-cell.typ", encoding: "utf8") ],
    [ #read("large-cell.typ", encoding: "utf8") ],
  ),
  caption: [],
)

#table(
  columns: 3,
    [Substance],
    [Subcritical °C],
    [Supercritical °C],
  
  [#read("large-cell.typ", encoding: "utf8")],
  [12.0], [92.1],
  [Sodium Myreth Sulfate],
  [16.6], [104],
  [#read("large-cell.typ", encoding: "utf8"), #read("large-cell.typ", encoding: "utf8")],
  [24.7],
  [114.514]
)
