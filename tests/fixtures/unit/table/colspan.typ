#let ofi = [Office]
#let rem = [_Remote_]
#let lea = [*On leave*]

#show table.cell.where(y: 0): set text(
  fill: white,
  weight: "bold",
)

#table(
  columns: 6 * (1fr,),
  align: (x, y) => if x == 0 or y == 0 { left } else { center },
  stroke: (x, y) => (
    // Separate black cells with white strokes.
    left: if y == 0 and x > 0 { white } else { black },
    rest: black,
  ),
  fill: (_, y) => if y == 0 { black },

  table.header(
    [Team member],
    [Monday],
    [Tuesday],
    [Wednesday],
    [Thursday],
    [Friday]
  ),
  [Evelyn Archer],
    table.cell(colspan: 2, ofi),
    table.cell(colspan: 2, rem),
    ofi,
  [Lila Montgomery],
    table.cell(colspan: 5, lea),
  [Nolan Pearce],
    rem,
    table.cell(colspan: 2, ofi),
    rem,
    ofi,
)

#table(
  columns: 4 * (1fr,),
  
  [a], [b], [c], [d],
  fill: (_, y) => if y == 0 { black },
  table.cell(rowspan: 2)[aa], table.cell(colspan: 2)[bc], [d],
  [b], table.cell(colspan: 2)[cd],
)
