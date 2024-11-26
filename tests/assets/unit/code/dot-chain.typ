#{
  let a = (1, (2, (3, (4,))))
  a.at(1, default: 0).at(1, default: 0).at(1, default: 0).at(4, default: 0)
}
