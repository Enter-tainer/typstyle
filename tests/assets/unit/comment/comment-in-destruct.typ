#{
  let (/* test */ a, b) = (1, 2);
  let (a, /* test */ _) = (1, 2);
  let (a: /* test */ h) = (
    a: 1
  )
}
