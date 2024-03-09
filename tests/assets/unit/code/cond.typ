#if not true {
  // false
}

#if (is-tablex-cell(cell)
        and type(cell.y) in (_int-type, _float-type)
        and cell.y > acc) {
    cell.y
} else {
    acc
}
