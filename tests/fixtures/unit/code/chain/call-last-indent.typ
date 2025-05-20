#{
  let tree = (1,2,3)
  let depth = 1
  let build-node(tree, depth: 0, sibling: 1) = {
    repr(tree) + repr(depth) + repr(sibling)
  }
  let children
        children = tree.slice(1).enumerate().map(((n, c)) =>
        build-node(c, depth: depth + 1, sibling: n))
  children
}
