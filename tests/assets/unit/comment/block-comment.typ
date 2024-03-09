#if draw-edge == auto {
  draw-edge = (source-name, target-name, target-node) => {
    let (a, b) = (source-name + "." + direction,
                  target-name + "." + opposite-dir.at(direction))

    draw.line(a, b)
    /*
    if direction == "bottom" {
      draw.line(a, (rel: (0, -grow/3)), ((), "-|", b), b)
    } else if direction == "up" {
      draw.line(a, (rel: (0, grow/3)), ((), "-|", b), b)
    } else if direction == "left" {
      draw.line(a, (rel: (-grow/3, 0)), ((), "|-", b), b)
    } else if direction == "right" {
      draw.line(a, (rel: (grow/3, 0)), ((), "|-", b), b)
    }
    */
  }
}
